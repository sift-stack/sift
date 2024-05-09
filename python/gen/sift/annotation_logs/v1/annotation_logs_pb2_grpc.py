# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

from sift.annotation_logs.v1 import annotation_logs_pb2 as sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2


class AnnotationLogServiceStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.CreateAnnotationLog = channel.unary_unary(
                '/sift.annotation_logs.v1.AnnotationLogService/CreateAnnotationLog',
                request_serializer=sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.CreateAnnotationLogRequest.SerializeToString,
                response_deserializer=sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.CreateAnnotationLogResponse.FromString,
                )
        self.ListAnnotationLogs = channel.unary_unary(
                '/sift.annotation_logs.v1.AnnotationLogService/ListAnnotationLogs',
                request_serializer=sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.ListAnnotationLogsRequest.SerializeToString,
                response_deserializer=sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.ListAnnotationLogsResponse.FromString,
                )
        self.DeleteAnnotationLog = channel.unary_unary(
                '/sift.annotation_logs.v1.AnnotationLogService/DeleteAnnotationLog',
                request_serializer=sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.DeleteAnnotationLogRequest.SerializeToString,
                response_deserializer=sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.DeleteAnnotationLogResponse.FromString,
                )


class AnnotationLogServiceServicer(object):
    """Missing associated documentation comment in .proto file."""

    def CreateAnnotationLog(self, request, context):
        """Creates an annotation log on an annotation.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListAnnotationLogs(self, request, context):
        """Retrieves annotation logs using an optional filter.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def DeleteAnnotationLog(self, request, context):
        """Deletes an annotation log.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_AnnotationLogServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'CreateAnnotationLog': grpc.unary_unary_rpc_method_handler(
                    servicer.CreateAnnotationLog,
                    request_deserializer=sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.CreateAnnotationLogRequest.FromString,
                    response_serializer=sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.CreateAnnotationLogResponse.SerializeToString,
            ),
            'ListAnnotationLogs': grpc.unary_unary_rpc_method_handler(
                    servicer.ListAnnotationLogs,
                    request_deserializer=sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.ListAnnotationLogsRequest.FromString,
                    response_serializer=sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.ListAnnotationLogsResponse.SerializeToString,
            ),
            'DeleteAnnotationLog': grpc.unary_unary_rpc_method_handler(
                    servicer.DeleteAnnotationLog,
                    request_deserializer=sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.DeleteAnnotationLogRequest.FromString,
                    response_serializer=sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.DeleteAnnotationLogResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'sift.annotation_logs.v1.AnnotationLogService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class AnnotationLogService(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def CreateAnnotationLog(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/sift.annotation_logs.v1.AnnotationLogService/CreateAnnotationLog',
            sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.CreateAnnotationLogRequest.SerializeToString,
            sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.CreateAnnotationLogResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def ListAnnotationLogs(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/sift.annotation_logs.v1.AnnotationLogService/ListAnnotationLogs',
            sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.ListAnnotationLogsRequest.SerializeToString,
            sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.ListAnnotationLogsResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def DeleteAnnotationLog(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/sift.annotation_logs.v1.AnnotationLogService/DeleteAnnotationLog',
            sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.DeleteAnnotationLogRequest.SerializeToString,
            sift_dot_annotation__logs_dot_v1_dot_annotation__logs__pb2.DeleteAnnotationLogResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)
