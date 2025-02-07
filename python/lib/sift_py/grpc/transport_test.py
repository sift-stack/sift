# ruff: noqa: N802

import re
from concurrent import futures
from contextlib import contextmanager
from typing import Any, Callable, Iterator, cast

import grpc
import pytest
from pytest_mock import MockFixture, MockType
from sift.data.v2.data_pb2 import GetDataRequest, GetDataResponse
from sift.data.v2.data_pb2_grpc import (
    DataServiceServicer,
    DataServiceStub,
    add_DataServiceServicer_to_server,
)

from sift_py._internal.test_util.server_interceptor import ServerInterceptor
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel


class DataService(DataServiceServicer):
    def GetData(self, request: GetDataRequest, context: grpc.ServicerContext):
        return GetDataResponse(next_page_token="next-page-token")


class AuthInterceptor(ServerInterceptor):
    AUTH_REGEX = re.compile(r"^Bearer (.+)$")

    def intercept(
        self,
        method: Callable,
        request_or_iterator: Any,
        context: grpc.ServicerContext,
        method_name: str,
    ) -> Any:
        authenticated = False
        for metadata in context.invocation_metadata():
            if metadata.key == "authorization":
                auth = self.__class__.AUTH_REGEX.match(metadata.value)

                if auth is not None and len(auth.group(1)) > 0:
                    authenticated = True

                break

        if authenticated:
            return method(request_or_iterator, context)
        else:
            context.set_code(grpc.StatusCode.UNAUTHENTICATED)
            context.set_details("Invalid or missing API key")
            raise


class ForceFailInterceptor(ServerInterceptor):
    """
    Force RPC to fail a few times before letting it pass.

    `failed_attempts`: Count of how many times failed
    `expected_num_fails`: How many times you want call to fail
    """

    failed_attempts: int
    expected_num_fails: int

    def __init__(self, expected_num_fails: int):
        self.expected_num_fails = expected_num_fails
        self.failed_attempts = 0
        super().__init__()

    def intercept(
        self,
        method: Callable,
        request_or_iterator: Any,
        context: grpc.ServicerContext,
        method_name: str,
    ) -> Any:
        if self.failed_attempts < self.expected_num_fails:
            self.failed_attempts += 1
            context.set_code(grpc.StatusCode.UNKNOWN)
            context.set_details("something unknown happened")
            raise

        return method(request_or_iterator, context)


def test_sift_channel(mocker: MockFixture):
    @contextmanager
    def test_server_spy(*interceptors: ServerInterceptor) -> Iterator[MockType]:
        server = grpc.server(
            thread_pool=futures.ThreadPoolExecutor(max_workers=1), interceptors=list(interceptors)
        )

        data_service = DataService()
        spy = mocker.spy(data_service, "GetData")

        add_DataServiceServicer_to_server(data_service, server)
        server.add_insecure_port("[::]:50052")
        server.start()
        try:
            yield spy
        finally:
            server.stop(None)
            server.wait_for_termination()

    with test_server_spy(AuthInterceptor()) as get_data_spy:
        sift_channel_config_a: SiftChannelConfig = {
            "uri": "localhost:50052",
            "apikey": "",
            "use_ssl": False,
        }

        with use_sift_channel(sift_channel_config_a) as channel:
            with pytest.raises(grpc.RpcError, match="UNAUTHENTICATED"):
                stub = DataServiceStub(channel)
                _ = cast(GetDataResponse, stub.GetData(GetDataRequest()))

            get_data_spy.assert_not_called()

        sift_channel_config_b: SiftChannelConfig = {
            "uri": "localhost:50052",
            "apikey": "some-token",
            "use_ssl": False,
        }

        with use_sift_channel(sift_channel_config_b) as channel:
            stub = DataServiceStub(channel)
            res = cast(GetDataResponse, stub.GetData(GetDataRequest()))
            assert res.next_page_token == "next-page-token"
            get_data_spy.assert_called_once()

    force_fail_interceptor = ForceFailInterceptor(4)
    with test_server_spy(AuthInterceptor(), force_fail_interceptor) as get_data_spy:
        sift_channel_config_c: SiftChannelConfig = {
            "uri": "localhost:50052",
            "apikey": "some-token",
            "use_ssl": False,
        }

        with use_sift_channel(sift_channel_config_c) as channel:
            stub = DataServiceStub(channel)
            # This will attempt 5 times: fail 4 times, succeed on 5th
            res = cast(GetDataResponse, stub.GetData(GetDataRequest()))
            assert res.next_page_token == "next-page-token"
            get_data_spy.assert_called_once()

    # fail 4 times, pass the 5th attempt
    assert force_fail_interceptor.failed_attempts == 4

    # Now we're going to fail beyond the max retry attempts

    force_fail_interceptor_max = ForceFailInterceptor(7)
    with test_server_spy(AuthInterceptor(), force_fail_interceptor_max) as get_data_spy:
        sift_channel_config_d: SiftChannelConfig = {
            "uri": "localhost:50052",
            "apikey": "some-token",
            "use_ssl": False,
        }

        with use_sift_channel(sift_channel_config_d) as channel:
            stub = DataServiceStub(channel)

            # This will go beyond the max number of attempts
            with pytest.raises(Exception):
                res = cast(GetDataResponse, stub.GetData(GetDataRequest()))

            get_data_spy.assert_not_called()

    # All attempts failed
    assert force_fail_interceptor_max.failed_attempts == 5
