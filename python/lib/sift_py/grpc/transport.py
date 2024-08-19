"""
This module is concerned with creating a gRPC transport channel specifically for
interacting with Sift's gRPC API. the `use_sift_channel` method creates said channel
and should generally be used within a with-block for correct resource management.
"""

from __future__ import annotations

from typing import Any, List, Optional, Tuple, TypedDict, Union

import grpc
import grpc.aio as grpc_aio
from sift_py.grpc.keepalive import DEFAULT_KEEPALIVE_CONFIG, KeepaliveConfig
from typing_extensions import NotRequired, TypeAlias

from sift_py.grpc._async_interceptors.base import ClientAsyncInterceptor
from sift_py.grpc._async_interceptors.metadata import MetadataAsyncInterceptor
from sift_py.grpc._interceptors.base import ClientInterceptor
from sift_py.grpc._interceptors.metadata import Metadata, MetadataInterceptor
from sift_py.grpc._retry import RetryPolicy

SiftChannel: TypeAlias = grpc.Channel
SiftAsyncChannel: TypeAlias = grpc_aio.Channel


def use_sift_channel(config: SiftChannelConfig) -> SiftChannel:
    """
    Returns an intercepted channel that is meant to be used across all services that
    make RPCs to Sift's API. It is highly encouraged to use this within a with-block
    for correct resource clean-up.

    Should an RPC fail for a reason that isn't explicitly controlled by Sift, `SiftChannel`
    will automatically leverage gRPC's retry mechanism to try and recover until the max-attempts
    are exceeded, after which the underlying exception will be raised.
    """
    if not config.get("use_ssl", True):
        return _use_insecure_sift_channel(config)

    credentials = grpc.ssl_channel_credentials()
    options = _compute_channel_options(config)
    channel = grpc.secure_channel(config["uri"], credentials, options)
    interceptors = _compute_sift_interceptors(config)
    return grpc.intercept_channel(channel, *interceptors)


def use_sift_async_channel(config: SiftChannelConfig) -> SiftAsyncChannel:
    """
    Like `use_sift_channel` but returns a channel meant to be used within the context
    of an async runtime when asynchonous I/O is required.
    """
    if not config.get("use_ssl", True):
        return _use_insecure_sift_async_channel(config)

    return grpc_aio.secure_channel(
        target=config["uri"],
        credentials=grpc.ssl_channel_credentials(),
        options=_compute_channel_options(config),
        interceptors=_compute_sift_async_interceptors(config),
    )


def _use_insecure_sift_channel(config: SiftChannelConfig) -> SiftChannel:
    """
    FOR DEVELOPMENT PURPOSES ONLY
    """
    options = _compute_channel_options()
    channel = grpc.insecure_channel(config["uri"], options)
    interceptors = _compute_sift_interceptors(config)
    return grpc.intercept_channel(channel, *interceptors)


def _use_insecure_sift_async_channel(config: SiftChannelConfig) -> SiftAsyncChannel:
    """
    FOR DEVELOPMENT PURPOSES ONLY
    """
    return grpc_aio.insecure_channel(
        target=config["uri"],
        options=_compute_channel_options(),
        interceptors=_compute_sift_async_interceptors(config),
    )


def _compute_sift_interceptors(config: SiftChannelConfig) -> List[ClientInterceptor]:
    """
    Initialized all interceptors here.
    """
    return [
        _metadata_interceptor(config),
    ]


def _compute_sift_async_interceptors(config: SiftChannelConfig) -> List[grpc_aio.ClientInterceptor]:
    return [
        _metadata_async_interceptor(config),
    ]


def _compute_channel_options(opts: Optional[SiftChannelConfig] = None) -> List[Tuple[str, Any]]:
    """
    Initialize all [channel options](https://github.com/grpc/grpc/blob/v1.64.x/include/grpc/impl/channel_arg_names.h) here.
    """

    options = [("grpc.enable_retries", 1), ("grpc.service_config", RetryPolicy.default().as_json())]

    if opts is None:
        return options

    if keepalive := opts.get("enable_keepalive"):
        config = DEFAULT_KEEPALIVE_CONFIG if isinstance(keepalive, bool) else keepalive
        options.extend(
            [
                ("grpc.keepalive_time_ms", config["keepalive_time_ms"]),
                ("grpc.keepalive_timeout_ms", config["keepalive_timeout_ms"]),
                ("grpc.http2.max_pings_without_data", config["max_pings_without_data"]),
                ("grpc.keepalive_permit_without_calls", config["keepalive_permit_without_calls"]),
            ]
        )

    return options


def _metadata_interceptor(config: SiftChannelConfig) -> ClientInterceptor:
    """
    Any new metadata goes here.
    """
    apikey = config["apikey"]
    metadata: Metadata = [
        ("authorization", f"Bearer {apikey}"),
    ]
    return MetadataInterceptor(metadata)


def _metadata_async_interceptor(config: SiftChannelConfig) -> ClientAsyncInterceptor:
    """
    Any new metadata goes here for unary-unary calls.
    """
    apikey = config["apikey"]
    metadata: Metadata = [
        ("authorization", f"Bearer {apikey}"),
    ]
    return MetadataAsyncInterceptor(metadata)


class SiftChannelConfig(TypedDict):
    """
    Config class used to instantiate a `SiftChannel` via `use_sift_channel`.
    - `uri`: The URI of Sift's gRPC API. The scheme portion of the URI i.e. `https://` should be ommitted.
    - `apikey`: User-generated API key generated via the Sift application.
    - `enable_keepalive`: Enable HTTP/2 PING-based keepalive to allow long-lived connections with idle long periods. If
    set to `True`, it will use the default values configured in `sift_py.grpc.keepalive` to configure keepalive. A custom
    `sift_py.grpc.keepalive.KeepaliveConfig` may also be provided. Default disabled.
    - `use_ssl`: INTERNAL USE. Meant to be used for local development.
    """

    uri: str
    apikey: str
    enable_keepalive: NotRequired[Union[bool, KeepaliveConfig]]
    use_ssl: NotRequired[bool]
