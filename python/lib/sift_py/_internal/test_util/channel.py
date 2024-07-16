from collections.abc import AsyncIterable, Iterable
from typing import Any, Callable, Optional, Union

import grpc
import grpc.aio as grpc_aio
from grpc.aio import Channel as AsyncChannel
from grpc_testing import Channel

SerializingFunction = Callable[[Any], bytes]
DeserializingFunction = Callable[[bytes], Any]
DoneCallbackType = Callable[[Any], None]
RequestIterableType = Union[Iterable, AsyncIterable]
ResponseIterableType = AsyncIterable


class MockChannel(Channel):
    """
    Used as a mock gRPC channel
    """

    def take_unary_unary(self, method_descriptor):
        pass

    def take_unary_stream(self, method_descriptor):
        pass

    def take_stream_unary(self, method_descriptor):
        pass

    def take_stream_stream(self, method_descriptor):
        pass

    def subscribe(self, callback, try_to_connect=False):
        pass

    def unsubscribe(self, callback):
        pass

    def unary_unary(
        self,
        method,
        request_serializer=None,
        response_deserializer=None,
        _registered_method=False,
    ):
        pass

    def unary_stream(
        self,
        method,
        request_serializer=None,
        response_deserializer=None,
        _registered_method=False,
    ):
        pass

    def stream_unary(
        self,
        method,
        request_serializer=None,
        response_deserializer=None,
        _registered_method=False,
    ):
        pass

    def stream_stream(
        self,
        method,
        request_serializer=None,
        response_deserializer=None,
        _registered_method=False,
    ):
        pass

    def close(self):
        pass

    def __enter__(self):
        pass

    def __exit__(self, exc_type, exc_val, exc_tb):
        pass


class MockAsyncChannel(AsyncChannel):
    async def __aenter__(self):
        pass

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        pass

    async def close(self, grace: Optional[float] = None):
        pass

    def get_state(self, try_to_connect: bool = False) -> grpc.ChannelConnectivity: ...

    async def wait_for_state_change(
        self,
        last_observed_state: grpc.ChannelConnectivity,
    ) -> None:
        return None

    async def channel_ready(self) -> None:
        return None

    def unary_unary(
        self,
        method: str,
        request_serializer: Optional[SerializingFunction] = None,
        response_deserializer: Optional[DeserializingFunction] = None,
        _registered_method: Optional[bool] = False,
    ) -> grpc_aio.UnaryUnaryMultiCallable: ...

    def unary_stream(
        self,
        method: str,
        request_serializer: Optional[SerializingFunction] = None,
        response_deserializer: Optional[DeserializingFunction] = None,
        _registered_method: Optional[bool] = False,
    ) -> grpc_aio.UnaryStreamMultiCallable: ...

    def stream_unary(
        self,
        method: str,
        request_serializer: Optional[SerializingFunction] = None,
        response_deserializer: Optional[DeserializingFunction] = None,
        _registered_method: Optional[bool] = False,
    ) -> grpc_aio.StreamUnaryMultiCallable: ...

    def stream_stream(
        self,
        method: str,
        request_serializer: Optional[SerializingFunction] = None,
        response_deserializer: Optional[DeserializingFunction] = None,
        _registered_method: Optional[bool] = False,
    ) -> grpc_aio.StreamStreamMultiCallable: ...
