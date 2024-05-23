# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

from sift.ingestion_configs.v1 import ingestion_configs_pb2 as sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2


class IngestionConfigServiceStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.GetIngestionConfig = channel.unary_unary(
                '/sift.ingestion_configs.v1.IngestionConfigService/GetIngestionConfig',
                request_serializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.GetIngestionConfigRequest.SerializeToString,
                response_deserializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.GetIngestionConfigResponse.FromString,
                )
        self.CreateIngestionConfig = channel.unary_unary(
                '/sift.ingestion_configs.v1.IngestionConfigService/CreateIngestionConfig',
                request_serializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.CreateIngestionConfigRequest.SerializeToString,
                response_deserializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.CreateIngestionConfigResponse.FromString,
                )
        self.ListIngestionConfigs = channel.unary_unary(
                '/sift.ingestion_configs.v1.IngestionConfigService/ListIngestionConfigs',
                request_serializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.ListIngestionConfigsRequest.SerializeToString,
                response_deserializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.ListIngestionConfigsResponse.FromString,
                )
        self.CreateIngestionConfigFlows = channel.unary_unary(
                '/sift.ingestion_configs.v1.IngestionConfigService/CreateIngestionConfigFlows',
                request_serializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.CreateIngestionConfigFlowsRequest.SerializeToString,
                response_deserializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.CreateIngestionConfigFlowsResponse.FromString,
                )
        self.ListIngestionConfigFlows = channel.unary_unary(
                '/sift.ingestion_configs.v1.IngestionConfigService/ListIngestionConfigFlows',
                request_serializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.ListIngestionConfigFlowsRequest.SerializeToString,
                response_deserializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.ListIngestionConfigFlowsResponse.FromString,
                )


class IngestionConfigServiceServicer(object):
    """Missing associated documentation comment in .proto file."""

    def GetIngestionConfig(self, request, context):
        """Retrieves an ingestion config.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def CreateIngestionConfig(self, request, context):
        """Create an ingestion config.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListIngestionConfigs(self, request, context):
        """List ingestion configs using an optional filter.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def CreateIngestionConfigFlows(self, request, context):
        """Create ingestion config [flows](/glossary#flow).
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListIngestionConfigFlows(self, request, context):
        """List ingestion config [flows](/glossary#flow) using an optional filter.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_IngestionConfigServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'GetIngestionConfig': grpc.unary_unary_rpc_method_handler(
                    servicer.GetIngestionConfig,
                    request_deserializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.GetIngestionConfigRequest.FromString,
                    response_serializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.GetIngestionConfigResponse.SerializeToString,
            ),
            'CreateIngestionConfig': grpc.unary_unary_rpc_method_handler(
                    servicer.CreateIngestionConfig,
                    request_deserializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.CreateIngestionConfigRequest.FromString,
                    response_serializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.CreateIngestionConfigResponse.SerializeToString,
            ),
            'ListIngestionConfigs': grpc.unary_unary_rpc_method_handler(
                    servicer.ListIngestionConfigs,
                    request_deserializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.ListIngestionConfigsRequest.FromString,
                    response_serializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.ListIngestionConfigsResponse.SerializeToString,
            ),
            'CreateIngestionConfigFlows': grpc.unary_unary_rpc_method_handler(
                    servicer.CreateIngestionConfigFlows,
                    request_deserializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.CreateIngestionConfigFlowsRequest.FromString,
                    response_serializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.CreateIngestionConfigFlowsResponse.SerializeToString,
            ),
            'ListIngestionConfigFlows': grpc.unary_unary_rpc_method_handler(
                    servicer.ListIngestionConfigFlows,
                    request_deserializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.ListIngestionConfigFlowsRequest.FromString,
                    response_serializer=sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.ListIngestionConfigFlowsResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'sift.ingestion_configs.v1.IngestionConfigService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class IngestionConfigService(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def GetIngestionConfig(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/sift.ingestion_configs.v1.IngestionConfigService/GetIngestionConfig',
            sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.GetIngestionConfigRequest.SerializeToString,
            sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.GetIngestionConfigResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def CreateIngestionConfig(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/sift.ingestion_configs.v1.IngestionConfigService/CreateIngestionConfig',
            sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.CreateIngestionConfigRequest.SerializeToString,
            sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.CreateIngestionConfigResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def ListIngestionConfigs(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/sift.ingestion_configs.v1.IngestionConfigService/ListIngestionConfigs',
            sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.ListIngestionConfigsRequest.SerializeToString,
            sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.ListIngestionConfigsResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def CreateIngestionConfigFlows(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/sift.ingestion_configs.v1.IngestionConfigService/CreateIngestionConfigFlows',
            sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.CreateIngestionConfigFlowsRequest.SerializeToString,
            sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.CreateIngestionConfigFlowsResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def ListIngestionConfigFlows(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/sift.ingestion_configs.v1.IngestionConfigService/ListIngestionConfigFlows',
            sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.ListIngestionConfigFlowsRequest.SerializeToString,
            sift_dot_ingestion__configs_dot_v1_dot_ingestion__configs__pb2.ListIngestionConfigFlowsResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)