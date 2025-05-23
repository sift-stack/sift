"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
INTERNAL NOTES FOR MAINTAINERS:
This protobuf package serves proxying layer for underlying services which still depend on
protobufs from the `azimuth.rules.v1` package. Any updates to this file need to be ported
in copy-paste fashion to `azimuth.rules.v1`.

Also, be sure to update the following Go package to convert between the new and legacy types.:
- azimuth.services.repo.rule.v1.pbmapper
"""

import abc
import collections.abc
import grpc
import grpc.aio
import sift.rules.v1.rules_pb2
import typing

_T = typing.TypeVar("_T")

class _MaybeAsyncIterator(collections.abc.AsyncIterator[_T], collections.abc.Iterator[_T], metaclass=abc.ABCMeta): ...

class _ServicerContext(grpc.ServicerContext, grpc.aio.ServicerContext):  # type: ignore[misc, type-arg]
    ...

class RuleServiceStub:
    def __init__(self, channel: typing.Union[grpc.Channel, grpc.aio.Channel]) -> None: ...
    SearchRules: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.SearchRulesRequest,
        sift.rules.v1.rules_pb2.SearchRulesResponse,
    ]
    """Queries rules based on provided search parameters."""

    GetRule: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.GetRuleRequest,
        sift.rules.v1.rules_pb2.GetRuleResponse,
    ]
    """Retrieves the latest version of a rule."""

    BatchGetRules: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.BatchGetRulesRequest,
        sift.rules.v1.rules_pb2.BatchGetRulesResponse,
    ]
    """Retrieve multiple rules."""

    CreateRule: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.CreateRuleRequest,
        sift.rules.v1.rules_pb2.CreateRuleResponse,
    ]
    """Creates a rule."""

    UpdateRule: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.UpdateRuleRequest,
        sift.rules.v1.rules_pb2.UpdateRuleResponse,
    ]
    """Updates an existing rule."""

    BatchUpdateRules: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.BatchUpdateRulesRequest,
        sift.rules.v1.rules_pb2.BatchUpdateRulesResponse,
    ]
    """Updates existing rules or creates rules that do not exist."""

    DeleteRule: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.DeleteRuleRequest,
        sift.rules.v1.rules_pb2.DeleteRuleResponse,
    ]
    """Deletes a rule"""

    BatchDeleteRules: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.BatchDeleteRulesRequest,
        sift.rules.v1.rules_pb2.BatchDeleteRulesResponse,
    ]
    """Deletes multiple rules"""

    UndeleteRule: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.UndeleteRuleRequest,
        sift.rules.v1.rules_pb2.UndeleteRuleResponse,
    ]
    """Undeletes a rule"""

    BatchUndeleteRules: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.BatchUndeleteRulesRequest,
        sift.rules.v1.rules_pb2.BatchUndeleteRulesResponse,
    ]
    """Undeletes multiple rules"""

    EvaluateRules: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.EvaluateRulesRequest,
        sift.rules.v1.rules_pb2.EvaluateRulesResponse,
    ]
    """Deprecated - use RuleEvaluationService instead."""

    ViewHumanFriendlyRules: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.ViewHumanFriendlyRulesRequest,
        sift.rules.v1.rules_pb2.ViewHumanFriendlyRulesResponse,
    ]
    """Deprecated - use ViewJsonRules instead. Retrieve a JSON object containing all of the rules for a given asset."""

    ViewJsonRules: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.ViewJsonRulesRequest,
        sift.rules.v1.rules_pb2.ViewJsonRulesResponse,
    ]
    """Deprecated - use BatchGetRules instead. Retrieve a JSON object containing all of the rules for a given asset."""

    UpdateHumanFriendlyRules: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.UpdateHumanFriendlyRulesRequest,
        sift.rules.v1.rules_pb2.UpdateHumanFriendlyRulesResponse,
    ]
    """Deprecated - use BatchUpdateRules instead. Batch update rules given the `rules_json` which is a JSON list of rules."""

    ValidateJsonRules: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.ValidateJsonRulesRequest,
        sift.rules.v1.rules_pb2.ValidateJsonRulesResponse,
    ]
    """Deprecated - use BatchUpdateRules with validate_only flag instead. Validate a batch update for rules given the `rules_json` which is a JSON list of rules. This is a dry-run operation."""

    UpdateJsonRules: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.UpdateJsonRulesRequest,
        sift.rules.v1.rules_pb2.UpdateJsonRulesResponse,
    ]
    """Deprecated - use BatchUpdateRules instead. Batch update rules given the `rules_json` which is a JSON list of rules."""

    ListRules: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.ListRulesRequest,
        sift.rules.v1.rules_pb2.ListRulesResponse,
    ]

    ListRuleVersions: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.ListRuleVersionsRequest,
        sift.rules.v1.rules_pb2.ListRuleVersionsResponse,
    ]
    """Retrieves a list of rule versions for the given rule."""

    GetRuleVersion: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.GetRuleVersionRequest,
        sift.rules.v1.rules_pb2.GetRuleVersionResponse,
    ]
    """Retrieves a specific version of a rule."""

    BatchGetRuleVersions: grpc.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.BatchGetRuleVersionsRequest,
        sift.rules.v1.rules_pb2.BatchGetRuleVersionsResponse,
    ]
    """Retrieves multiple rules by rule versions."""

class RuleServiceAsyncStub:
    SearchRules: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.SearchRulesRequest,
        sift.rules.v1.rules_pb2.SearchRulesResponse,
    ]
    """Queries rules based on provided search parameters."""

    GetRule: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.GetRuleRequest,
        sift.rules.v1.rules_pb2.GetRuleResponse,
    ]
    """Retrieves the latest version of a rule."""

    BatchGetRules: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.BatchGetRulesRequest,
        sift.rules.v1.rules_pb2.BatchGetRulesResponse,
    ]
    """Retrieve multiple rules."""

    CreateRule: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.CreateRuleRequest,
        sift.rules.v1.rules_pb2.CreateRuleResponse,
    ]
    """Creates a rule."""

    UpdateRule: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.UpdateRuleRequest,
        sift.rules.v1.rules_pb2.UpdateRuleResponse,
    ]
    """Updates an existing rule."""

    BatchUpdateRules: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.BatchUpdateRulesRequest,
        sift.rules.v1.rules_pb2.BatchUpdateRulesResponse,
    ]
    """Updates existing rules or creates rules that do not exist."""

    DeleteRule: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.DeleteRuleRequest,
        sift.rules.v1.rules_pb2.DeleteRuleResponse,
    ]
    """Deletes a rule"""

    BatchDeleteRules: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.BatchDeleteRulesRequest,
        sift.rules.v1.rules_pb2.BatchDeleteRulesResponse,
    ]
    """Deletes multiple rules"""

    UndeleteRule: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.UndeleteRuleRequest,
        sift.rules.v1.rules_pb2.UndeleteRuleResponse,
    ]
    """Undeletes a rule"""

    BatchUndeleteRules: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.BatchUndeleteRulesRequest,
        sift.rules.v1.rules_pb2.BatchUndeleteRulesResponse,
    ]
    """Undeletes multiple rules"""

    EvaluateRules: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.EvaluateRulesRequest,
        sift.rules.v1.rules_pb2.EvaluateRulesResponse,
    ]
    """Deprecated - use RuleEvaluationService instead."""

    ViewHumanFriendlyRules: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.ViewHumanFriendlyRulesRequest,
        sift.rules.v1.rules_pb2.ViewHumanFriendlyRulesResponse,
    ]
    """Deprecated - use ViewJsonRules instead. Retrieve a JSON object containing all of the rules for a given asset."""

    ViewJsonRules: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.ViewJsonRulesRequest,
        sift.rules.v1.rules_pb2.ViewJsonRulesResponse,
    ]
    """Deprecated - use BatchGetRules instead. Retrieve a JSON object containing all of the rules for a given asset."""

    UpdateHumanFriendlyRules: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.UpdateHumanFriendlyRulesRequest,
        sift.rules.v1.rules_pb2.UpdateHumanFriendlyRulesResponse,
    ]
    """Deprecated - use BatchUpdateRules instead. Batch update rules given the `rules_json` which is a JSON list of rules."""

    ValidateJsonRules: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.ValidateJsonRulesRequest,
        sift.rules.v1.rules_pb2.ValidateJsonRulesResponse,
    ]
    """Deprecated - use BatchUpdateRules with validate_only flag instead. Validate a batch update for rules given the `rules_json` which is a JSON list of rules. This is a dry-run operation."""

    UpdateJsonRules: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.UpdateJsonRulesRequest,
        sift.rules.v1.rules_pb2.UpdateJsonRulesResponse,
    ]
    """Deprecated - use BatchUpdateRules instead. Batch update rules given the `rules_json` which is a JSON list of rules."""

    ListRules: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.ListRulesRequest,
        sift.rules.v1.rules_pb2.ListRulesResponse,
    ]

    ListRuleVersions: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.ListRuleVersionsRequest,
        sift.rules.v1.rules_pb2.ListRuleVersionsResponse,
    ]
    """Retrieves a list of rule versions for the given rule."""

    GetRuleVersion: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.GetRuleVersionRequest,
        sift.rules.v1.rules_pb2.GetRuleVersionResponse,
    ]
    """Retrieves a specific version of a rule."""

    BatchGetRuleVersions: grpc.aio.UnaryUnaryMultiCallable[
        sift.rules.v1.rules_pb2.BatchGetRuleVersionsRequest,
        sift.rules.v1.rules_pb2.BatchGetRuleVersionsResponse,
    ]
    """Retrieves multiple rules by rule versions."""

class RuleServiceServicer(metaclass=abc.ABCMeta):
    @abc.abstractmethod
    def SearchRules(
        self,
        request: sift.rules.v1.rules_pb2.SearchRulesRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.SearchRulesResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.SearchRulesResponse]]:
        """Queries rules based on provided search parameters."""

    @abc.abstractmethod
    def GetRule(
        self,
        request: sift.rules.v1.rules_pb2.GetRuleRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.GetRuleResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.GetRuleResponse]]:
        """Retrieves the latest version of a rule."""

    @abc.abstractmethod
    def BatchGetRules(
        self,
        request: sift.rules.v1.rules_pb2.BatchGetRulesRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.BatchGetRulesResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.BatchGetRulesResponse]]:
        """Retrieve multiple rules."""

    @abc.abstractmethod
    def CreateRule(
        self,
        request: sift.rules.v1.rules_pb2.CreateRuleRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.CreateRuleResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.CreateRuleResponse]]:
        """Creates a rule."""

    @abc.abstractmethod
    def UpdateRule(
        self,
        request: sift.rules.v1.rules_pb2.UpdateRuleRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.UpdateRuleResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.UpdateRuleResponse]]:
        """Updates an existing rule."""

    @abc.abstractmethod
    def BatchUpdateRules(
        self,
        request: sift.rules.v1.rules_pb2.BatchUpdateRulesRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.BatchUpdateRulesResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.BatchUpdateRulesResponse]]:
        """Updates existing rules or creates rules that do not exist."""

    @abc.abstractmethod
    def DeleteRule(
        self,
        request: sift.rules.v1.rules_pb2.DeleteRuleRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.DeleteRuleResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.DeleteRuleResponse]]:
        """Deletes a rule"""

    @abc.abstractmethod
    def BatchDeleteRules(
        self,
        request: sift.rules.v1.rules_pb2.BatchDeleteRulesRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.BatchDeleteRulesResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.BatchDeleteRulesResponse]]:
        """Deletes multiple rules"""

    @abc.abstractmethod
    def UndeleteRule(
        self,
        request: sift.rules.v1.rules_pb2.UndeleteRuleRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.UndeleteRuleResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.UndeleteRuleResponse]]:
        """Undeletes a rule"""

    @abc.abstractmethod
    def BatchUndeleteRules(
        self,
        request: sift.rules.v1.rules_pb2.BatchUndeleteRulesRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.BatchUndeleteRulesResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.BatchUndeleteRulesResponse]]:
        """Undeletes multiple rules"""

    @abc.abstractmethod
    def EvaluateRules(
        self,
        request: sift.rules.v1.rules_pb2.EvaluateRulesRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.EvaluateRulesResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.EvaluateRulesResponse]]:
        """Deprecated - use RuleEvaluationService instead."""

    @abc.abstractmethod
    def ViewHumanFriendlyRules(
        self,
        request: sift.rules.v1.rules_pb2.ViewHumanFriendlyRulesRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.ViewHumanFriendlyRulesResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.ViewHumanFriendlyRulesResponse]]:
        """Deprecated - use ViewJsonRules instead. Retrieve a JSON object containing all of the rules for a given asset."""

    @abc.abstractmethod
    def ViewJsonRules(
        self,
        request: sift.rules.v1.rules_pb2.ViewJsonRulesRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.ViewJsonRulesResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.ViewJsonRulesResponse]]:
        """Deprecated - use BatchGetRules instead. Retrieve a JSON object containing all of the rules for a given asset."""

    @abc.abstractmethod
    def UpdateHumanFriendlyRules(
        self,
        request: sift.rules.v1.rules_pb2.UpdateHumanFriendlyRulesRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.UpdateHumanFriendlyRulesResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.UpdateHumanFriendlyRulesResponse]]:
        """Deprecated - use BatchUpdateRules instead. Batch update rules given the `rules_json` which is a JSON list of rules."""

    @abc.abstractmethod
    def ValidateJsonRules(
        self,
        request: sift.rules.v1.rules_pb2.ValidateJsonRulesRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.ValidateJsonRulesResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.ValidateJsonRulesResponse]]:
        """Deprecated - use BatchUpdateRules with validate_only flag instead. Validate a batch update for rules given the `rules_json` which is a JSON list of rules. This is a dry-run operation."""

    @abc.abstractmethod
    def UpdateJsonRules(
        self,
        request: sift.rules.v1.rules_pb2.UpdateJsonRulesRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.UpdateJsonRulesResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.UpdateJsonRulesResponse]]:
        """Deprecated - use BatchUpdateRules instead. Batch update rules given the `rules_json` which is a JSON list of rules."""

    @abc.abstractmethod
    def ListRules(
        self,
        request: sift.rules.v1.rules_pb2.ListRulesRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.ListRulesResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.ListRulesResponse]]: ...

    @abc.abstractmethod
    def ListRuleVersions(
        self,
        request: sift.rules.v1.rules_pb2.ListRuleVersionsRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.ListRuleVersionsResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.ListRuleVersionsResponse]]:
        """Retrieves a list of rule versions for the given rule."""

    @abc.abstractmethod
    def GetRuleVersion(
        self,
        request: sift.rules.v1.rules_pb2.GetRuleVersionRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.GetRuleVersionResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.GetRuleVersionResponse]]:
        """Retrieves a specific version of a rule."""

    @abc.abstractmethod
    def BatchGetRuleVersions(
        self,
        request: sift.rules.v1.rules_pb2.BatchGetRuleVersionsRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.rules.v1.rules_pb2.BatchGetRuleVersionsResponse, collections.abc.Awaitable[sift.rules.v1.rules_pb2.BatchGetRuleVersionsResponse]]:
        """Retrieves multiple rules by rule versions."""

def add_RuleServiceServicer_to_server(servicer: RuleServiceServicer, server: typing.Union[grpc.Server, grpc.aio.Server]) -> None: ...
