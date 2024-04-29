# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

from sift.runs.v2 import runs_pb2 as sift_dot_runs_dot_v2_dot_runs__pb2


class RunServiceStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.GetRun = channel.unary_unary(
                '/sift.runs.v2.RunService/GetRun',
                request_serializer=sift_dot_runs_dot_v2_dot_runs__pb2.GetRunRequest.SerializeToString,
                response_deserializer=sift_dot_runs_dot_v2_dot_runs__pb2.GetRunResponse.FromString,
                )
        self.ListRuns = channel.unary_unary(
                '/sift.runs.v2.RunService/ListRuns',
                request_serializer=sift_dot_runs_dot_v2_dot_runs__pb2.ListRunsRequest.SerializeToString,
                response_deserializer=sift_dot_runs_dot_v2_dot_runs__pb2.ListRunsResponse.FromString,
                )
        self.CreateRun = channel.unary_unary(
                '/sift.runs.v2.RunService/CreateRun',
                request_serializer=sift_dot_runs_dot_v2_dot_runs__pb2.CreateRunRequest.SerializeToString,
                response_deserializer=sift_dot_runs_dot_v2_dot_runs__pb2.CreateRunResponse.FromString,
                )
        self.UpdateRun = channel.unary_unary(
                '/sift.runs.v2.RunService/UpdateRun',
                request_serializer=sift_dot_runs_dot_v2_dot_runs__pb2.UpdateRunRequest.SerializeToString,
                response_deserializer=sift_dot_runs_dot_v2_dot_runs__pb2.UpdateRunResponse.FromString,
                )
        self.CreateAutomaticRunAssociationForAssets = channel.unary_unary(
                '/sift.runs.v2.RunService/CreateAutomaticRunAssociationForAssets',
                request_serializer=sift_dot_runs_dot_v2_dot_runs__pb2.CreateAutomaticRunAssociationForAssetsRequest.SerializeToString,
                response_deserializer=sift_dot_runs_dot_v2_dot_runs__pb2.CreateAutomaticRunAssociationForAssetsResponse.FromString,
                )


class RunServiceServicer(object):
    """Missing associated documentation comment in .proto file."""

    def GetRun(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListRuns(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def CreateRun(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def UpdateRun(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def CreateAutomaticRunAssociationForAssets(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_RunServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'GetRun': grpc.unary_unary_rpc_method_handler(
                    servicer.GetRun,
                    request_deserializer=sift_dot_runs_dot_v2_dot_runs__pb2.GetRunRequest.FromString,
                    response_serializer=sift_dot_runs_dot_v2_dot_runs__pb2.GetRunResponse.SerializeToString,
            ),
            'ListRuns': grpc.unary_unary_rpc_method_handler(
                    servicer.ListRuns,
                    request_deserializer=sift_dot_runs_dot_v2_dot_runs__pb2.ListRunsRequest.FromString,
                    response_serializer=sift_dot_runs_dot_v2_dot_runs__pb2.ListRunsResponse.SerializeToString,
            ),
            'CreateRun': grpc.unary_unary_rpc_method_handler(
                    servicer.CreateRun,
                    request_deserializer=sift_dot_runs_dot_v2_dot_runs__pb2.CreateRunRequest.FromString,
                    response_serializer=sift_dot_runs_dot_v2_dot_runs__pb2.CreateRunResponse.SerializeToString,
            ),
            'UpdateRun': grpc.unary_unary_rpc_method_handler(
                    servicer.UpdateRun,
                    request_deserializer=sift_dot_runs_dot_v2_dot_runs__pb2.UpdateRunRequest.FromString,
                    response_serializer=sift_dot_runs_dot_v2_dot_runs__pb2.UpdateRunResponse.SerializeToString,
            ),
            'CreateAutomaticRunAssociationForAssets': grpc.unary_unary_rpc_method_handler(
                    servicer.CreateAutomaticRunAssociationForAssets,
                    request_deserializer=sift_dot_runs_dot_v2_dot_runs__pb2.CreateAutomaticRunAssociationForAssetsRequest.FromString,
                    response_serializer=sift_dot_runs_dot_v2_dot_runs__pb2.CreateAutomaticRunAssociationForAssetsResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'sift.runs.v2.RunService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class RunService(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def GetRun(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/sift.runs.v2.RunService/GetRun',
            sift_dot_runs_dot_v2_dot_runs__pb2.GetRunRequest.SerializeToString,
            sift_dot_runs_dot_v2_dot_runs__pb2.GetRunResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def ListRuns(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/sift.runs.v2.RunService/ListRuns',
            sift_dot_runs_dot_v2_dot_runs__pb2.ListRunsRequest.SerializeToString,
            sift_dot_runs_dot_v2_dot_runs__pb2.ListRunsResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def CreateRun(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/sift.runs.v2.RunService/CreateRun',
            sift_dot_runs_dot_v2_dot_runs__pb2.CreateRunRequest.SerializeToString,
            sift_dot_runs_dot_v2_dot_runs__pb2.CreateRunResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def UpdateRun(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/sift.runs.v2.RunService/UpdateRun',
            sift_dot_runs_dot_v2_dot_runs__pb2.UpdateRunRequest.SerializeToString,
            sift_dot_runs_dot_v2_dot_runs__pb2.UpdateRunResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def CreateAutomaticRunAssociationForAssets(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/sift.runs.v2.RunService/CreateAutomaticRunAssociationForAssets',
            sift_dot_runs_dot_v2_dot_runs__pb2.CreateAutomaticRunAssociationForAssetsRequest.SerializeToString,
            sift_dot_runs_dot_v2_dot_runs__pb2.CreateAutomaticRunAssociationForAssetsResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)
