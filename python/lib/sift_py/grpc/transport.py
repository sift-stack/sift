"""
This module is concerned with creating a gRPC transport channel specifically for
interacting with Sift's gRPC API. the `use_sift_channel` method creates said channel
and should generally be used within a with-block for correct resource management.
"""

from __future__ import annotations

from typing import List, TypedDict

import grpc
from grpc_interceptor import ClientInterceptor
from typing_extensions import TypeAlias

from sift_py.grpc._interceptors import Metadata, MetadataInterceptor

SiftChannel: TypeAlias = grpc.Channel


def use_sift_channel(config: SiftChannelConfig) -> SiftChannel:
    """
    Returns an intercepted channel that is meant to be used across all services that
    make RPCs to Sift's API. It is highly encouraged to use this within a with-block
    for correct resouce clean-up as to ensure no long-lived idle channels.
    """
    credentials = grpc.ssl_channel_credentials()
    channel = grpc.secure_channel(config["uri"], credentials)
    interceptors = _compute_sift_interceptors(config)
    return grpc.intercept_channel(channel, *interceptors)


def _use_insecure_sift_channel(config: SiftChannelConfig) -> SiftChannel:
    """
    FOR DEVELOPMENT PURPOSES ONLY
    """
    channel = grpc.insecure_channel(config["uri"])
    interceptors = _compute_sift_interceptors(config)
    return grpc.intercept_channel(channel, *interceptors)


def _compute_sift_interceptors(config: SiftChannelConfig) -> List[ClientInterceptor]:
    """
    Initialized all interceptors here.
    """
    return [
        _metadata_interceptor(config),
    ]


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
    """

    uri: str
    apikey: str
