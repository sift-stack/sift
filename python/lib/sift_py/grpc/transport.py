"""
This module is concerned with creating a gRPC transport channel specifically for
interacting with Sift's gRPC API. the `use_sift_channel` method creates said channel
and should generally be used within a with-block for correct resource management.
"""

from __future__ import annotations

from typing import Any, List, Tuple, TypedDict

import grpc
from grpc_interceptor import ClientInterceptor
from typing_extensions import NotRequired, TypeAlias

from sift_py.grpc._interceptors import Metadata, MetadataInterceptor
from sift_py.grpc._retry import RetryPolicy

SiftChannel: TypeAlias = grpc.Channel


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
    options = _compute_channel_options()
    channel = grpc.secure_channel(config["uri"], credentials, options)
    interceptors = _compute_sift_interceptors(config)
    return grpc.intercept_channel(channel, *interceptors)


def _use_insecure_sift_channel(config: SiftChannelConfig) -> SiftChannel:
    """
    FOR DEVELOPMENT PURPOSES ONLY
    """
    options = _compute_channel_options()
    channel = grpc.insecure_channel(config["uri"], options)
    interceptors = _compute_sift_interceptors(config)
    return grpc.intercept_channel(channel, *interceptors)


def _compute_sift_interceptors(config: SiftChannelConfig) -> List[ClientInterceptor]:
    """
    Initialized all interceptors here.
    """
    return [
        _metadata_interceptor(config),
    ]


def _compute_channel_options() -> List[Tuple[str, Any]]:
    """
    Initialize all [channel options](https://github.com/grpc/grpc/blob/v1.64.x/include/grpc/impl/channel_arg_names.h) here.
    """
    return [("grpc.enable_retries", 1), ("grpc.service_config", RetryPolicy.default().as_json())]


def _metadata_interceptor(config: SiftChannelConfig) -> ClientInterceptor:
    """
    Any new metadata goes here.
    """
    apikey = config["apikey"]
    metadata: Metadata = [
        ("authorization", f"Bearer {apikey}"),
    ]
    return MetadataInterceptor(metadata)


class SiftChannelConfig(TypedDict):
    """
    Config class used to instantiate a `SiftChannel` via `use_sift_channel`.
    - `uri`: The URI of Sift's gRPC API. The scheme portion of the URI i.e. `https://` should be ommitted.
    - `apikey`: User-generated API key generated via the Sift application.
    - `use_ssl`: INTERNAL USE. Meant to be used for local development.
    """

    uri: str
    apikey: str
    use_ssl: NotRequired[bool]
