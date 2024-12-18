"""
This module is concerned with creating a gRPC transport channel specifically for
interacting with Sift's gRPC API. the `use_sift_channel` method creates said channel
and should generally be used within a with-block for correct resource management.
"""

from __future__ import annotations

from importlib.metadata import PackageNotFoundError, version
from typing import Any, Dict, List, Optional, Tuple, TypedDict, Union, cast
from urllib.parse import ParseResult, urlparse

import grpc
import grpc.aio as grpc_aio
from typing_extensions import NotRequired, TypeAlias

from sift_py.grpc._async_interceptors.base import ClientAsyncInterceptor
from sift_py.grpc._async_interceptors.metadata import MetadataAsyncInterceptor
from sift_py.grpc._interceptors.base import ClientInterceptor
from sift_py.grpc._interceptors.metadata import Metadata, MetadataInterceptor
from sift_py.grpc._retry import RetryPolicy
from sift_py.grpc.keepalive import DEFAULT_KEEPALIVE_CONFIG, KeepaliveConfig

SiftChannel: TypeAlias = grpc.Channel
SiftAsyncChannel: TypeAlias = grpc_aio.Channel


def get_ssl_credentials(cert_via_openssl: bool) -> grpc.ChannelCredentials:
    """
    Returns SSL credentials for use with gRPC.
    Workaround for this issue: https://github.com/grpc/grpc/issues/29682
    """
    if not cert_via_openssl:
        return grpc.ssl_channel_credentials()

    try:
        import ssl

        from OpenSSL import crypto

        ssl_context = ssl.create_default_context()
        certs_der = ssl_context.get_ca_certs(binary_form=True)
        certs_x509 = [crypto.load_certificate(crypto.FILETYPE_ASN1, x) for x in certs_der]
        certs_pem = [crypto.dump_certificate(crypto.FILETYPE_PEM, x) for x in certs_x509]
        certs_bytes = b"".join(certs_pem)

        return grpc.ssl_channel_credentials(certs_bytes)
    except ImportError as e:
        raise Exception(
            "Missing required dependencies for cert_via_openssl. Run `pip install sift-stack-py[openssl]` to install the required dependencies."
        ) from e


def use_sift_channel(
    config: SiftChannelConfig, metadata: Optional[Dict[str, Any]] = None
) -> SiftChannel:
    """
    Returns an intercepted channel that is meant to be used across all services that
    make RPCs to Sift's API. It is highly encouraged to use this within a with-block
    for correct resource clean-up.

    Should an RPC fail for a reason that isn't explicitly controlled by Sift, `SiftChannel`
    will automatically leverage gRPC's retry mechanism to try and recover until the max-attempts
    are exceeded, after which the underlying exception will be raised.
    """
    use_ssl = config.get("use_ssl", True)
    cert_via_openssl = config.get("cert_via_openssl", False)

    if not use_ssl:
        return _use_insecure_sift_channel(config, metadata)

    credentials = get_ssl_credentials(cert_via_openssl)
    options = _compute_channel_options(config)
    api_uri = _clean_uri(config["uri"], use_ssl)
    channel = grpc.secure_channel(api_uri, credentials, options)
    interceptors = _compute_sift_interceptors(config, metadata)
    return grpc.intercept_channel(channel, *interceptors)


def use_sift_async_channel(
    config: SiftChannelConfig, metadata: Optional[Dict[str, Any]] = None
) -> SiftAsyncChannel:
    """
    Like `use_sift_channel` but returns a channel meant to be used within the context
    of an async runtime when asynchonous I/O is required.
    """
    use_ssl = config.get("use_ssl", True)
    cert_via_openssl = config.get("cert_via_openssl", False)

    if not use_ssl:
        return _use_insecure_sift_async_channel(config, metadata)

    return grpc_aio.secure_channel(
        target=_clean_uri(config["uri"], use_ssl),
        credentials=get_ssl_credentials(cert_via_openssl),
        options=_compute_channel_options(config),
        interceptors=_compute_sift_async_interceptors(config, metadata),
    )


def _use_insecure_sift_channel(
    config: SiftChannelConfig, metadata: Optional[Dict[str, Any]] = None
) -> SiftChannel:
    """
    FOR DEVELOPMENT PURPOSES ONLY
    """
    options = _compute_channel_options(config)
    api_uri = _clean_uri(config["uri"], False)
    channel = grpc.insecure_channel(api_uri, options)
    interceptors = _compute_sift_interceptors(config, metadata)
    return grpc.intercept_channel(channel, *interceptors)


def _use_insecure_sift_async_channel(
    config: SiftChannelConfig, metadata: Optional[Dict[str, Any]] = None
) -> SiftAsyncChannel:
    """
    FOR DEVELOPMENT PURPOSES ONLY
    """
    return grpc_aio.insecure_channel(
        target=config["uri"],
        options=_compute_channel_options(config),
        interceptors=_compute_sift_async_interceptors(config, metadata),
    )


def _compute_sift_interceptors(
    config: SiftChannelConfig, metadata: Optional[Dict[str, Any]] = None
) -> List[ClientInterceptor]:
    """
    Initialized all interceptors here.
    """
    return [
        _metadata_interceptor(config, metadata),
    ]


def _compute_sift_async_interceptors(
    config: SiftChannelConfig, metadata: Optional[Dict[str, Any]] = None
) -> List[grpc_aio.ClientInterceptor]:
    return [
        _metadata_async_interceptor(config, metadata),
    ]


def _compute_channel_options(opts: SiftChannelConfig) -> List[Tuple[str, Any]]:
    """
    Initialize all [channel options](https://github.com/grpc/grpc/blob/v1.64.x/include/grpc/impl/channel_arg_names.h) here.
    """

    options = [
        ("grpc.enable_retries", 1),
        ("grpc.service_config", RetryPolicy.default().as_json()),
        # Primary cannot be overriden:
        #  https://github.com/grpc/grpc/blob/0498194240f55d7f4b12633ad01339fb690621bf/src/core/ext/filters/http/client/http_client_filter.cc#L97
        ("grpc.secondary_user_agent", _compute_user_agent()),
    ]

    enable_keepalive = opts.get("enable_keepalive", True)
    if isinstance(enable_keepalive, dict):
        config = cast(KeepaliveConfig, enable_keepalive)
        options.extend(_compute_keep_alive_channel_opts(config))
    elif enable_keepalive:
        options.extend(_compute_keep_alive_channel_opts(DEFAULT_KEEPALIVE_CONFIG))

    return options


def _metadata_interceptor(
    config: SiftChannelConfig, metadata: Optional[Dict[str, Any]] = None
) -> ClientInterceptor:
    """
    Any new metadata goes here.
    """
    apikey = config["apikey"]
    md: Metadata = [("authorization", f"Bearer {apikey}")]

    if metadata:
        for key, val in metadata.items():
            md.append((key, val))

    return MetadataInterceptor(md)


def _metadata_async_interceptor(
    config: SiftChannelConfig, metadata: Optional[Dict[str, Any]] = None
) -> ClientAsyncInterceptor:
    """
    Any new metadata goes here for unary-unary calls.
    """
    apikey = config["apikey"]
    md: Metadata = [("authorization", f"Bearer {apikey}")]

    if metadata:
        for key, val in metadata.items():
            md.append((key, val))

    return MetadataAsyncInterceptor(md)


def _clean_uri(uri: str, use_ssl: bool) -> str:
    """
    This will automatically transform the URI to an acceptable form regardless of whether or not
    users included the scheme in the URL or included trailing slashes.
    """

    if "http://" in uri or "https://" in uri:
        parsed: ParseResult = urlparse(uri)
        return parsed.netloc

    full_uri = f"https://{uri}" if use_ssl else f"http://{uri}"
    parsed_res: ParseResult = urlparse(full_uri)
    return parsed_res.netloc


def _compute_user_agent() -> str:
    try:
        return f"sift_stack_py/{version('sift_stack_py')}"
    except PackageNotFoundError:
        return "sift-stack-py"


def _compute_keep_alive_channel_opts(config: KeepaliveConfig) -> List[Tuple[str, int]]:
    return [
        ("grpc.keepalive_time_ms", config["keepalive_time_ms"]),
        ("grpc.keepalive_timeout_ms", config["keepalive_timeout_ms"]),
        ("grpc.http2.max_pings_without_data", config["max_pings_without_data"]),
        ("grpc.keepalive_permit_without_calls", config["keepalive_permit_without_calls"]),
    ]


class SiftChannelConfig(TypedDict):
    """
    Config class used to instantiate a `SiftChannel` via `use_sift_channel`.
    - `uri`: The URI of Sift's gRPC API. The scheme portion of the URI i.e. `https://` should be ommitted.
    - `apikey`: User-generated API key generated via the Sift application.
    - `enable_keepalive`: Enabled by default, but can be disabled by passing in `False`. HTTP/2 keep-alive prevents connections from
    being terminated during idle periods. A custom `sift_py.grpc.keepalive.KeepaliveConfig` may also be provided.
    - `use_ssl`: INTERNAL USE. Meant to be used for local development.
    - `cert_via_openssl`: Enable this if you want to use OpenSSL to load the certificates.
    Run `pip install sift-stack-py[openssl]` to install the dependencies required to use this option.
    This works around this issue with grpc loading SSL certificates: https://github.com/grpc/grpc/issues/29682.
    Default is False.
    """

    uri: str
    apikey: str
    enable_keepalive: NotRequired[Union[bool, KeepaliveConfig]]
    use_ssl: NotRequired[bool]
    cert_via_openssl: NotRequired[bool]
