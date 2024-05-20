from google.api import annotations_pb2 as _annotations_pb2
from google.api import field_behavior_pb2 as _field_behavior_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from protoc_gen_openapiv2.options import annotations_pb2 as _annotations_pb2_1
from sift.annotations.v1 import annotations_pb2 as _annotations_pb2_1_1
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class SearchOrder(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    SEARCH_ORDER_UNSPECIFIED: _ClassVar[SearchOrder]
    SEARCH_ORDER_ASC: _ClassVar[SearchOrder]
    SEARCH_ORDER_DESC: _ClassVar[SearchOrder]

class ActionKind(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    ACTION_KIND_UNSPECIFIED: _ClassVar[ActionKind]
    NOTIFICATION: _ClassVar[ActionKind]
    ANNOTATION: _ClassVar[ActionKind]

class ConditionComparator(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    CONDITION_COMPARATOR_UNSPECIFIED: _ClassVar[ConditionComparator]
    LESS_THAN: _ClassVar[ConditionComparator]
    LESS_THAN_OR_EQUAL: _ClassVar[ConditionComparator]
    GREATER_THAN: _ClassVar[ConditionComparator]
    GREATER_THAN_OR_EQUAL: _ClassVar[ConditionComparator]
    EQUAL: _ClassVar[ConditionComparator]
    NOT_EQUAL: _ClassVar[ConditionComparator]
SEARCH_ORDER_UNSPECIFIED: SearchOrder
SEARCH_ORDER_ASC: SearchOrder
SEARCH_ORDER_DESC: SearchOrder
ACTION_KIND_UNSPECIFIED: ActionKind
NOTIFICATION: ActionKind
ANNOTATION: ActionKind
CONDITION_COMPARATOR_UNSPECIFIED: ConditionComparator
LESS_THAN: ConditionComparator
LESS_THAN_OR_EQUAL: ConditionComparator
GREATER_THAN: ConditionComparator
GREATER_THAN_OR_EQUAL: ConditionComparator
EQUAL: ConditionComparator
NOT_EQUAL: ConditionComparator

class Rule(_message.Message):
    __slots__ = ("rule_id", "asset_id", "name", "description", "current_status", "is_enabled", "created_date", "modified_date", "created_by_user_id", "modified_by_user_id", "organization_id", "conditions")
    RULE_ID_FIELD_NUMBER: _ClassVar[int]
    ASSET_ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    CURRENT_STATUS_FIELD_NUMBER: _ClassVar[int]
    IS_ENABLED_FIELD_NUMBER: _ClassVar[int]
    CREATED_DATE_FIELD_NUMBER: _ClassVar[int]
    MODIFIED_DATE_FIELD_NUMBER: _ClassVar[int]
    CREATED_BY_USER_ID_FIELD_NUMBER: _ClassVar[int]
    MODIFIED_BY_USER_ID_FIELD_NUMBER: _ClassVar[int]
    ORGANIZATION_ID_FIELD_NUMBER: _ClassVar[int]
    CONDITIONS_FIELD_NUMBER: _ClassVar[int]
    rule_id: str
    asset_id: str
    name: str
    description: str
    current_status: str
    is_enabled: bool
    created_date: _timestamp_pb2.Timestamp
    modified_date: _timestamp_pb2.Timestamp
    created_by_user_id: str
    modified_by_user_id: str
    organization_id: str
    conditions: _containers.RepeatedCompositeFieldContainer[RuleCondition]
    def __init__(self, rule_id: _Optional[str] = ..., asset_id: _Optional[str] = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., current_status: _Optional[str] = ..., is_enabled: bool = ..., created_date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., modified_date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., created_by_user_id: _Optional[str] = ..., modified_by_user_id: _Optional[str] = ..., organization_id: _Optional[str] = ..., conditions: _Optional[_Iterable[_Union[RuleCondition, _Mapping]]] = ...) -> None: ...

class RuleCondition(_message.Message):
    __slots__ = ("rule_condition_id", "rule_id", "expression", "status", "created_date", "modified_date", "created_by_user_id", "modified_by_user_id", "actions")
    RULE_CONDITION_ID_FIELD_NUMBER: _ClassVar[int]
    RULE_ID_FIELD_NUMBER: _ClassVar[int]
    EXPRESSION_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    CREATED_DATE_FIELD_NUMBER: _ClassVar[int]
    MODIFIED_DATE_FIELD_NUMBER: _ClassVar[int]
    CREATED_BY_USER_ID_FIELD_NUMBER: _ClassVar[int]
    MODIFIED_BY_USER_ID_FIELD_NUMBER: _ClassVar[int]
    ACTIONS_FIELD_NUMBER: _ClassVar[int]
    rule_condition_id: str
    rule_id: str
    expression: RuleConditionExpression
    status: str
    created_date: _timestamp_pb2.Timestamp
    modified_date: _timestamp_pb2.Timestamp
    created_by_user_id: str
    modified_by_user_id: str
    actions: _containers.RepeatedCompositeFieldContainer[RuleAction]
    def __init__(self, rule_condition_id: _Optional[str] = ..., rule_id: _Optional[str] = ..., expression: _Optional[_Union[RuleConditionExpression, _Mapping]] = ..., status: _Optional[str] = ..., created_date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., modified_date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., created_by_user_id: _Optional[str] = ..., modified_by_user_id: _Optional[str] = ..., actions: _Optional[_Iterable[_Union[RuleAction, _Mapping]]] = ...) -> None: ...

class RuleAction(_message.Message):
    __slots__ = ("rule_action_id", "rule_condition_id", "action_type", "configuration", "created_date", "modified_date", "created_by_user_id", "modified_by_user_id")
    RULE_ACTION_ID_FIELD_NUMBER: _ClassVar[int]
    RULE_CONDITION_ID_FIELD_NUMBER: _ClassVar[int]
    ACTION_TYPE_FIELD_NUMBER: _ClassVar[int]
    CONFIGURATION_FIELD_NUMBER: _ClassVar[int]
    CREATED_DATE_FIELD_NUMBER: _ClassVar[int]
    MODIFIED_DATE_FIELD_NUMBER: _ClassVar[int]
    CREATED_BY_USER_ID_FIELD_NUMBER: _ClassVar[int]
    MODIFIED_BY_USER_ID_FIELD_NUMBER: _ClassVar[int]
    rule_action_id: str
    rule_condition_id: str
    action_type: ActionKind
    configuration: RuleActionConfiguration
    created_date: _timestamp_pb2.Timestamp
    modified_date: _timestamp_pb2.Timestamp
    created_by_user_id: str
    modified_by_user_id: str
    def __init__(self, rule_action_id: _Optional[str] = ..., rule_condition_id: _Optional[str] = ..., action_type: _Optional[_Union[ActionKind, str]] = ..., configuration: _Optional[_Union[RuleActionConfiguration, _Mapping]] = ..., created_date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., modified_date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., created_by_user_id: _Optional[str] = ..., modified_by_user_id: _Optional[str] = ...) -> None: ...

class SearchRulesRequest(_message.Message):
    __slots__ = ("limit", "offset", "order", "name_matches", "case_sensitive", "regexp", "order_by", "rule_ids", "asset_ids")
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    ORDER_FIELD_NUMBER: _ClassVar[int]
    NAME_MATCHES_FIELD_NUMBER: _ClassVar[int]
    CASE_SENSITIVE_FIELD_NUMBER: _ClassVar[int]
    REGEXP_FIELD_NUMBER: _ClassVar[int]
    ORDER_BY_FIELD_NUMBER: _ClassVar[int]
    RULE_IDS_FIELD_NUMBER: _ClassVar[int]
    ASSET_IDS_FIELD_NUMBER: _ClassVar[int]
    limit: int
    offset: int
    order: SearchOrder
    name_matches: str
    case_sensitive: bool
    regexp: bool
    order_by: str
    rule_ids: _containers.RepeatedScalarFieldContainer[str]
    asset_ids: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, limit: _Optional[int] = ..., offset: _Optional[int] = ..., order: _Optional[_Union[SearchOrder, str]] = ..., name_matches: _Optional[str] = ..., case_sensitive: bool = ..., regexp: bool = ..., order_by: _Optional[str] = ..., rule_ids: _Optional[_Iterable[str]] = ..., asset_ids: _Optional[_Iterable[str]] = ...) -> None: ...

class SearchRulesResponse(_message.Message):
    __slots__ = ("count", "rules")
    COUNT_FIELD_NUMBER: _ClassVar[int]
    RULES_FIELD_NUMBER: _ClassVar[int]
    count: int
    rules: _containers.RepeatedCompositeFieldContainer[Rule]
    def __init__(self, count: _Optional[int] = ..., rules: _Optional[_Iterable[_Union[Rule, _Mapping]]] = ...) -> None: ...

class GetRuleRequest(_message.Message):
    __slots__ = ("rule_id",)
    RULE_ID_FIELD_NUMBER: _ClassVar[int]
    rule_id: str
    def __init__(self, rule_id: _Optional[str] = ...) -> None: ...

class GetRuleResponse(_message.Message):
    __slots__ = ("rule",)
    RULE_FIELD_NUMBER: _ClassVar[int]
    rule: Rule
    def __init__(self, rule: _Optional[_Union[Rule, _Mapping]] = ...) -> None: ...

class BatchGetRulesRequest(_message.Message):
    __slots__ = ("rule_ids",)
    RULE_IDS_FIELD_NUMBER: _ClassVar[int]
    rule_ids: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, rule_ids: _Optional[_Iterable[str]] = ...) -> None: ...

class BatchGetRulesResponse(_message.Message):
    __slots__ = ("rules",)
    RULES_FIELD_NUMBER: _ClassVar[int]
    rules: _containers.RepeatedCompositeFieldContainer[Rule]
    def __init__(self, rules: _Optional[_Iterable[_Union[Rule, _Mapping]]] = ...) -> None: ...

class CreateRuleRequest(_message.Message):
    __slots__ = ("update",)
    UPDATE_FIELD_NUMBER: _ClassVar[int]
    update: UpdateRuleRequest
    def __init__(self, update: _Optional[_Union[UpdateRuleRequest, _Mapping]] = ...) -> None: ...

class CreateRuleResponse(_message.Message):
    __slots__ = ("rule_id",)
    RULE_ID_FIELD_NUMBER: _ClassVar[int]
    rule_id: str
    def __init__(self, rule_id: _Optional[str] = ...) -> None: ...

class UpdateRuleRequest(_message.Message):
    __slots__ = ("rule_id", "name", "description", "asset_id", "is_enabled", "conditions", "organization_id")
    RULE_ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    ASSET_ID_FIELD_NUMBER: _ClassVar[int]
    IS_ENABLED_FIELD_NUMBER: _ClassVar[int]
    CONDITIONS_FIELD_NUMBER: _ClassVar[int]
    ORGANIZATION_ID_FIELD_NUMBER: _ClassVar[int]
    rule_id: str
    name: str
    description: str
    asset_id: str
    is_enabled: bool
    conditions: _containers.RepeatedCompositeFieldContainer[UpdateConditionRequest]
    organization_id: str
    def __init__(self, rule_id: _Optional[str] = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., asset_id: _Optional[str] = ..., is_enabled: bool = ..., conditions: _Optional[_Iterable[_Union[UpdateConditionRequest, _Mapping]]] = ..., organization_id: _Optional[str] = ...) -> None: ...

class UpdateConditionRequest(_message.Message):
    __slots__ = ("rule_condition_id", "status", "expression", "actions")
    RULE_CONDITION_ID_FIELD_NUMBER: _ClassVar[int]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    EXPRESSION_FIELD_NUMBER: _ClassVar[int]
    ACTIONS_FIELD_NUMBER: _ClassVar[int]
    rule_condition_id: str
    status: str
    expression: RuleConditionExpression
    actions: _containers.RepeatedCompositeFieldContainer[UpdateActionRequest]
    def __init__(self, rule_condition_id: _Optional[str] = ..., status: _Optional[str] = ..., expression: _Optional[_Union[RuleConditionExpression, _Mapping]] = ..., actions: _Optional[_Iterable[_Union[UpdateActionRequest, _Mapping]]] = ...) -> None: ...

class UpdateActionRequest(_message.Message):
    __slots__ = ("rule_action_id", "action_type", "configuration")
    RULE_ACTION_ID_FIELD_NUMBER: _ClassVar[int]
    ACTION_TYPE_FIELD_NUMBER: _ClassVar[int]
    CONFIGURATION_FIELD_NUMBER: _ClassVar[int]
    rule_action_id: str
    action_type: ActionKind
    configuration: RuleActionConfiguration
    def __init__(self, rule_action_id: _Optional[str] = ..., action_type: _Optional[_Union[ActionKind, str]] = ..., configuration: _Optional[_Union[RuleActionConfiguration, _Mapping]] = ...) -> None: ...

class UpdateRuleResponse(_message.Message):
    __slots__ = ("rule_id",)
    RULE_ID_FIELD_NUMBER: _ClassVar[int]
    rule_id: str
    def __init__(self, rule_id: _Optional[str] = ...) -> None: ...

class DeleteRuleRequest(_message.Message):
    __slots__ = ("rule_id",)
    RULE_ID_FIELD_NUMBER: _ClassVar[int]
    rule_id: str
    def __init__(self, rule_id: _Optional[str] = ...) -> None: ...

class DeleteRuleResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class ViewHumanFriendlyRulesRequest(_message.Message):
    __slots__ = ("asset_id",)
    ASSET_ID_FIELD_NUMBER: _ClassVar[int]
    asset_id: str
    def __init__(self, asset_id: _Optional[str] = ...) -> None: ...

class ViewHumanFriendlyRulesResponse(_message.Message):
    __slots__ = ("rules_json",)
    RULES_JSON_FIELD_NUMBER: _ClassVar[int]
    rules_json: str
    def __init__(self, rules_json: _Optional[str] = ...) -> None: ...

class UpdateHumanFriendlyRulesRequest(_message.Message):
    __slots__ = ("asset_id", "rules_json", "organization_id")
    ASSET_ID_FIELD_NUMBER: _ClassVar[int]
    RULES_JSON_FIELD_NUMBER: _ClassVar[int]
    ORGANIZATION_ID_FIELD_NUMBER: _ClassVar[int]
    asset_id: str
    rules_json: str
    organization_id: str
    def __init__(self, asset_id: _Optional[str] = ..., rules_json: _Optional[str] = ..., organization_id: _Optional[str] = ...) -> None: ...

class UpdateHumanFriendlyRulesResponse(_message.Message):
    __slots__ = ("success", "rules_count", "messages")
    SUCCESS_FIELD_NUMBER: _ClassVar[int]
    RULES_COUNT_FIELD_NUMBER: _ClassVar[int]
    MESSAGES_FIELD_NUMBER: _ClassVar[int]
    success: bool
    rules_count: int
    messages: str
    def __init__(self, success: bool = ..., rules_count: _Optional[int] = ..., messages: _Optional[str] = ...) -> None: ...

class RuleConditionExpression(_message.Message):
    __slots__ = ("single_channel_comparison", "calculated_channel")
    SINGLE_CHANNEL_COMPARISON_FIELD_NUMBER: _ClassVar[int]
    CALCULATED_CHANNEL_FIELD_NUMBER: _ClassVar[int]
    single_channel_comparison: SingleChannelComparisonExpression
    calculated_channel: CalculatedChannelConfig
    def __init__(self, single_channel_comparison: _Optional[_Union[SingleChannelComparisonExpression, _Mapping]] = ..., calculated_channel: _Optional[_Union[CalculatedChannelConfig, _Mapping]] = ...) -> None: ...

class SingleChannelComparisonExpression(_message.Message):
    __slots__ = ("channel_component", "channel_name", "comparator", "double", "string", "last_value")
    CHANNEL_COMPONENT_FIELD_NUMBER: _ClassVar[int]
    CHANNEL_NAME_FIELD_NUMBER: _ClassVar[int]
    COMPARATOR_FIELD_NUMBER: _ClassVar[int]
    DOUBLE_FIELD_NUMBER: _ClassVar[int]
    STRING_FIELD_NUMBER: _ClassVar[int]
    LAST_VALUE_FIELD_NUMBER: _ClassVar[int]
    channel_component: str
    channel_name: str
    comparator: ConditionComparator
    double: float
    string: str
    last_value: LastValueThreshold
    def __init__(self, channel_component: _Optional[str] = ..., channel_name: _Optional[str] = ..., comparator: _Optional[_Union[ConditionComparator, str]] = ..., double: _Optional[float] = ..., string: _Optional[str] = ..., last_value: _Optional[_Union[LastValueThreshold, _Mapping]] = ...) -> None: ...

class LastValueThreshold(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class CalculatedChannelConfig(_message.Message):
    __slots__ = ("channel_references", "expression")
    class ChannelReferencesEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: ChannelReference
        def __init__(self, key: _Optional[str] = ..., value: _Optional[_Union[ChannelReference, _Mapping]] = ...) -> None: ...
    CHANNEL_REFERENCES_FIELD_NUMBER: _ClassVar[int]
    EXPRESSION_FIELD_NUMBER: _ClassVar[int]
    channel_references: _containers.MessageMap[str, ChannelReference]
    expression: str
    def __init__(self, channel_references: _Optional[_Mapping[str, ChannelReference]] = ..., expression: _Optional[str] = ...) -> None: ...

class ChannelReference(_message.Message):
    __slots__ = ("name", "component")
    NAME_FIELD_NUMBER: _ClassVar[int]
    COMPONENT_FIELD_NUMBER: _ClassVar[int]
    name: str
    component: str
    def __init__(self, name: _Optional[str] = ..., component: _Optional[str] = ...) -> None: ...

class RuleActionConfiguration(_message.Message):
    __slots__ = ("notification", "annotation")
    NOTIFICATION_FIELD_NUMBER: _ClassVar[int]
    ANNOTATION_FIELD_NUMBER: _ClassVar[int]
    notification: NotificationActionConfiguration
    annotation: AnnotationActionConfiguration
    def __init__(self, notification: _Optional[_Union[NotificationActionConfiguration, _Mapping]] = ..., annotation: _Optional[_Union[AnnotationActionConfiguration, _Mapping]] = ...) -> None: ...

class NotificationActionConfiguration(_message.Message):
    __slots__ = ("recipient_user_ids",)
    RECIPIENT_USER_IDS_FIELD_NUMBER: _ClassVar[int]
    recipient_user_ids: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, recipient_user_ids: _Optional[_Iterable[str]] = ...) -> None: ...

class AnnotationActionConfiguration(_message.Message):
    __slots__ = ("tag_ids", "annotation_type", "assigned_to_user_id")
    TAG_IDS_FIELD_NUMBER: _ClassVar[int]
    ANNOTATION_TYPE_FIELD_NUMBER: _ClassVar[int]
    ASSIGNED_TO_USER_ID_FIELD_NUMBER: _ClassVar[int]
    tag_ids: _containers.RepeatedScalarFieldContainer[str]
    annotation_type: _annotations_pb2_1_1.AnnotationType
    assigned_to_user_id: str
    def __init__(self, tag_ids: _Optional[_Iterable[str]] = ..., annotation_type: _Optional[_Union[_annotations_pb2_1_1.AnnotationType, str]] = ..., assigned_to_user_id: _Optional[str] = ...) -> None: ...

class EvaluateRulesRequest(_message.Message):
    __slots__ = ("rule_ids", "annotation_options", "run_id", "time_range")
    RULE_IDS_FIELD_NUMBER: _ClassVar[int]
    ANNOTATION_OPTIONS_FIELD_NUMBER: _ClassVar[int]
    RUN_ID_FIELD_NUMBER: _ClassVar[int]
    TIME_RANGE_FIELD_NUMBER: _ClassVar[int]
    rule_ids: _containers.RepeatedScalarFieldContainer[str]
    annotation_options: EvaluatedAnnotationOptions
    run_id: str
    time_range: TimeRangeQuery
    def __init__(self, rule_ids: _Optional[_Iterable[str]] = ..., annotation_options: _Optional[_Union[EvaluatedAnnotationOptions, _Mapping]] = ..., run_id: _Optional[str] = ..., time_range: _Optional[_Union[TimeRangeQuery, _Mapping]] = ...) -> None: ...

class EvaluatedAnnotationOptions(_message.Message):
    __slots__ = ("tag_ids",)
    TAG_IDS_FIELD_NUMBER: _ClassVar[int]
    tag_ids: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, tag_ids: _Optional[_Iterable[str]] = ...) -> None: ...

class TimeRangeQuery(_message.Message):
    __slots__ = ("start_time", "end_time")
    START_TIME_FIELD_NUMBER: _ClassVar[int]
    END_TIME_FIELD_NUMBER: _ClassVar[int]
    start_time: _timestamp_pb2.Timestamp
    end_time: _timestamp_pb2.Timestamp
    def __init__(self, start_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., end_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class EvaluateRulesResponse(_message.Message):
    __slots__ = ("created_annotation_count",)
    CREATED_ANNOTATION_COUNT_FIELD_NUMBER: _ClassVar[int]
    created_annotation_count: int
    def __init__(self, created_annotation_count: _Optional[int] = ...) -> None: ...
