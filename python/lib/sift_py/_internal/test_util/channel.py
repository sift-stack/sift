from grpc_testing import Channel


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
