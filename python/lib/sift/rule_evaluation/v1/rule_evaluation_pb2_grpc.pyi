"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import abc
import collections.abc
import grpc
import grpc.aio
import sift.rule_evaluation.v1.rule_evaluation_pb2
import typing

_T = typing.TypeVar("_T")

class _MaybeAsyncIterator(collections.abc.AsyncIterator[_T], collections.abc.Iterator[_T], metaclass=abc.ABCMeta): ...

class _ServicerContext(grpc.ServicerContext, grpc.aio.ServicerContext):  # type: ignore[misc, type-arg]
    ...

class RuleEvaluationServiceStub:
    def __init__(self, channel: typing.Union[grpc.Channel, grpc.aio.Channel]) -> None: ...
    EvaluateRules: grpc.UnaryUnaryMultiCallable[
        sift.rule_evaluation.v1.rule_evaluation_pb2.EvaluateRulesRequest,
        sift.rule_evaluation.v1.rule_evaluation_pb2.EvaluateRulesResponse,
    ]
    """Evaluate rules from a designated source against a run or asset and return the total amount of annotations created and the ID of the generated report."""

    EvaluateRulesPreview: grpc.UnaryUnaryMultiCallable[
        sift.rule_evaluation.v1.rule_evaluation_pb2.EvaluateRulesPreviewRequest,
        sift.rule_evaluation.v1.rule_evaluation_pb2.EvaluateRulesPreviewResponse,
    ]
    """Perform a dry run evaluation for existing rules or rule configurations against a run and return the annotations that would be generated."""

class RuleEvaluationServiceAsyncStub:
    EvaluateRules: grpc.aio.UnaryUnaryMultiCallable[
        sift.rule_evaluation.v1.rule_evaluation_pb2.EvaluateRulesRequest,
        sift.rule_evaluation.v1.rule_evaluation_pb2.EvaluateRulesResponse,
    ]
    """Evaluate rules from a designated source against a run or asset and return the total amount of annotations created and the ID of the generated report."""

    EvaluateRulesPreview: grpc.aio.UnaryUnaryMultiCallable[
        sift.rule_evaluation.v1.rule_evaluation_pb2.EvaluateRulesPreviewRequest,
        sift.rule_evaluation.v1.rule_evaluation_pb2.EvaluateRulesPreviewResponse,
    ]
    """Perform a dry run evaluation for existing rules or rule configurations against a run and return the annotations that would be generated."""

class RuleEvaluationServiceServicer(metaclass=abc.ABCMeta):
    @abc.abstractmethod
    def EvaluateRules(
        self,
        request: sift.rule_evaluation.v1.rule_evaluation_pb2.EvaluateRulesRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rule_evaluation.v1.rule_evaluation_pb2.EvaluateRulesResponse, collections.abc.Awaitable[sift.rule_evaluation.v1.rule_evaluation_pb2.EvaluateRulesResponse]]:
        """Evaluate rules from a designated source against a run or asset and return the total amount of annotations created and the ID of the generated report."""

    @abc.abstractmethod
    def EvaluateRulesPreview(
        self,
        request: sift.rule_evaluation.v1.rule_evaluation_pb2.EvaluateRulesPreviewRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rule_evaluation.v1.rule_evaluation_pb2.EvaluateRulesPreviewResponse, collections.abc.Awaitable[sift.rule_evaluation.v1.rule_evaluation_pb2.EvaluateRulesPreviewResponse]]:
        """Perform a dry run evaluation for existing rules or rule configurations against a run and return the annotations that would be generated."""

def add_RuleEvaluationServiceServicer_to_server(servicer: RuleEvaluationServiceServicer, server: typing.Union[grpc.Server, grpc.aio.Server]) -> None: ...
