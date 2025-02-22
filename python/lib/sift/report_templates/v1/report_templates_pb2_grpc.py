# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

from sift.report_templates.v1 import report_templates_pb2 as sift_dot_report__templates_dot_v1_dot_report__templates__pb2


class ReportTemplateServiceStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.GetReportTemplate = channel.unary_unary(
                '/sift.report_templates.v1.ReportTemplateService/GetReportTemplate',
                request_serializer=sift_dot_report__templates_dot_v1_dot_report__templates__pb2.GetReportTemplateRequest.SerializeToString,
                response_deserializer=sift_dot_report__templates_dot_v1_dot_report__templates__pb2.GetReportTemplateResponse.FromString,
                )
        self.CreateReportTemplate = channel.unary_unary(
                '/sift.report_templates.v1.ReportTemplateService/CreateReportTemplate',
                request_serializer=sift_dot_report__templates_dot_v1_dot_report__templates__pb2.CreateReportTemplateRequest.SerializeToString,
                response_deserializer=sift_dot_report__templates_dot_v1_dot_report__templates__pb2.CreateReportTemplateResponse.FromString,
                )
        self.ListReportTemplates = channel.unary_unary(
                '/sift.report_templates.v1.ReportTemplateService/ListReportTemplates',
                request_serializer=sift_dot_report__templates_dot_v1_dot_report__templates__pb2.ListReportTemplatesRequest.SerializeToString,
                response_deserializer=sift_dot_report__templates_dot_v1_dot_report__templates__pb2.ListReportTemplatesResponse.FromString,
                )
        self.UpdateReportTemplate = channel.unary_unary(
                '/sift.report_templates.v1.ReportTemplateService/UpdateReportTemplate',
                request_serializer=sift_dot_report__templates_dot_v1_dot_report__templates__pb2.UpdateReportTemplateRequest.SerializeToString,
                response_deserializer=sift_dot_report__templates_dot_v1_dot_report__templates__pb2.UpdateReportTemplateResponse.FromString,
                )


class ReportTemplateServiceServicer(object):
    """Missing associated documentation comment in .proto file."""

    def GetReportTemplate(self, request, context):
        """Retrieve a report template.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def CreateReportTemplate(self, request, context):
        """Create a report template.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListReportTemplates(self, request, context):
        """List report templates.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def UpdateReportTemplate(self, request, context):
        """Updates an existing report template using the list of fields specified in `update_mask`.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_ReportTemplateServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'GetReportTemplate': grpc.unary_unary_rpc_method_handler(
                    servicer.GetReportTemplate,
                    request_deserializer=sift_dot_report__templates_dot_v1_dot_report__templates__pb2.GetReportTemplateRequest.FromString,
                    response_serializer=sift_dot_report__templates_dot_v1_dot_report__templates__pb2.GetReportTemplateResponse.SerializeToString,
            ),
            'CreateReportTemplate': grpc.unary_unary_rpc_method_handler(
                    servicer.CreateReportTemplate,
                    request_deserializer=sift_dot_report__templates_dot_v1_dot_report__templates__pb2.CreateReportTemplateRequest.FromString,
                    response_serializer=sift_dot_report__templates_dot_v1_dot_report__templates__pb2.CreateReportTemplateResponse.SerializeToString,
            ),
            'ListReportTemplates': grpc.unary_unary_rpc_method_handler(
                    servicer.ListReportTemplates,
                    request_deserializer=sift_dot_report__templates_dot_v1_dot_report__templates__pb2.ListReportTemplatesRequest.FromString,
                    response_serializer=sift_dot_report__templates_dot_v1_dot_report__templates__pb2.ListReportTemplatesResponse.SerializeToString,
            ),
            'UpdateReportTemplate': grpc.unary_unary_rpc_method_handler(
                    servicer.UpdateReportTemplate,
                    request_deserializer=sift_dot_report__templates_dot_v1_dot_report__templates__pb2.UpdateReportTemplateRequest.FromString,
                    response_serializer=sift_dot_report__templates_dot_v1_dot_report__templates__pb2.UpdateReportTemplateResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'sift.report_templates.v1.ReportTemplateService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class ReportTemplateService(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def GetReportTemplate(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/sift.report_templates.v1.ReportTemplateService/GetReportTemplate',
            sift_dot_report__templates_dot_v1_dot_report__templates__pb2.GetReportTemplateRequest.SerializeToString,
            sift_dot_report__templates_dot_v1_dot_report__templates__pb2.GetReportTemplateResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def CreateReportTemplate(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/sift.report_templates.v1.ReportTemplateService/CreateReportTemplate',
            sift_dot_report__templates_dot_v1_dot_report__templates__pb2.CreateReportTemplateRequest.SerializeToString,
            sift_dot_report__templates_dot_v1_dot_report__templates__pb2.CreateReportTemplateResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def ListReportTemplates(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/sift.report_templates.v1.ReportTemplateService/ListReportTemplates',
            sift_dot_report__templates_dot_v1_dot_report__templates__pb2.ListReportTemplatesRequest.SerializeToString,
            sift_dot_report__templates_dot_v1_dot_report__templates__pb2.ListReportTemplatesResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def UpdateReportTemplate(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/sift.report_templates.v1.ReportTemplateService/UpdateReportTemplate',
            sift_dot_report__templates_dot_v1_dot_report__templates__pb2.UpdateReportTemplateRequest.SerializeToString,
            sift_dot_report__templates_dot_v1_dot_report__templates__pb2.UpdateReportTemplateResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)
