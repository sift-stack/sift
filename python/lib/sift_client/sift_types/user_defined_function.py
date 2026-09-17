from __future__ import annotations

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING, ClassVar

from pydantic import BaseModel
from sift.common.type.v1.user_defined_functions_pb2 import (
    FunctionDataType as FunctionDataTypeProto,
)
from sift.common.type.v1.user_defined_functions_pb2 import (
    FunctionInput as FunctionInputProto,
)
from sift.common.type.v1.user_defined_functions_pb2 import (
    UserDefinedFunction as UserDefinedFunctionProto,
)
from sift.user_defined_functions.v1.user_defined_functions_pb2 import (
    CreateUserDefinedFunctionRequest as CreateUserDefinedFunctionRequestProto,
)

from sift_client.sift_types._base import (
    BaseType,
    MappingHelper,
    ModelCreate,
    ModelCreateUpdateBase,
    ModelUpdate,
)
from sift_client.util.metadata import metadata_dict_to_proto, metadata_proto_to_dict

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class FunctionDataType(Enum):
    """Enum for the data type of a function input or output."""

    UNSPECIFIED = FunctionDataTypeProto.FUNCTION_DATA_TYPE_UNSPECIFIED  # 0
    NUMERIC = FunctionDataTypeProto.FUNCTION_DATA_TYPE_NUMERIC  # 1
    STRING = FunctionDataTypeProto.FUNCTION_DATA_TYPE_STRING  # 2
    BOOL = FunctionDataTypeProto.FUNCTION_DATA_TYPE_BOOL  # 3


class FunctionInput(BaseModel):
    """An input the function's expression refers to.

    Attributes:
        identifier: How the expression refers to this input, such as `$1`.
        data_type: The type the input accepts.
        constant: Whether the input is a literal rather than a channel.
    """

    identifier: str
    data_type: FunctionDataType = FunctionDataType.NUMERIC
    constant: bool = False

    @classmethod
    def _from_proto(cls, proto: FunctionInputProto) -> FunctionInput:
        return cls(
            identifier=proto.identifier,
            data_type=FunctionDataType(proto.data_type),
            constant=proto.constant,
        )


def _function_input_to_proto(**kwargs) -> FunctionInputProto:
    """Convert a function input dict (from model_dump) into its proto form."""
    data_type = kwargs.get("data_type", FunctionDataType.NUMERIC)
    return FunctionInputProto(
        identifier=kwargs["identifier"],
        data_type=data_type.value if isinstance(data_type, FunctionDataType) else data_type,
        constant=kwargs.get("constant", False),
    )


class UserDefinedFunction(BaseType[UserDefinedFunctionProto, "UserDefinedFunction"]):
    """A reusable expression that calculated channels and rules can call.

    Each save produces a new version. `id_` identifies the function; `version_id`
    identifies this particular version.
    """

    # Required fields
    name: str
    description: str
    expression: str
    version: int
    version_id: str
    change_message: str
    user_notes: str
    function_inputs: list[FunctionInput]
    dependency_version_ids: list[str]
    metadata: dict[str, str | float | bool]
    created_date: datetime
    modified_date: datetime
    created_by_user_id: str
    modified_by_user_id: str
    is_archived: bool

    # Optional fields
    output_type: FunctionDataType | None
    archived_date: datetime | None

    @classmethod
    def _from_proto(
        cls, proto: UserDefinedFunctionProto, sift_client: SiftClient | None = None
    ) -> UserDefinedFunction:
        return cls(
            proto=proto,
            id_=proto.user_defined_function_id,
            name=proto.name,
            description=proto.description,
            expression=proto.expression,
            version=proto.version,
            version_id=proto.user_defined_function_version_id,
            change_message=proto.change_message,
            user_notes=proto.user_notes,
            function_inputs=[FunctionInput._from_proto(i) for i in proto.function_inputs],
            dependency_version_ids=[
                d.user_defined_function_version_id for d in proto.function_dependencies
            ],
            metadata=metadata_proto_to_dict(proto.metadata),  # type: ignore
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            is_archived=proto.is_archived,
            output_type=(
                FunctionDataType(proto.function_output_type) if proto.function_output_type else None
            ),
            archived_date=(
                proto.archived_date.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("archived_date")
                else None
            ),
            _client=sift_client,
        )

    @property
    def versions(self) -> list[UserDefinedFunction]:
        """Return every version of this function, newest first."""
        return self.client.user_defined_functions.versions.list_(function=self._id_or_error)

    def update(self, update: UserDefinedFunctionUpdate | dict) -> UserDefinedFunction:
        """Update the function. This creates a new version.

        Args:
            update: The update to apply. See UserDefinedFunctionUpdate for updatable fields.

        Returns:
            The updated function.
        """
        updated = self.client.user_defined_functions.update(function=self, update=update)
        self._update(updated)
        return self

    def archive(self) -> UserDefinedFunction:
        """Archive the function."""
        updated = self.client.user_defined_functions.archive(function=self)
        self._update(updated)
        return self

    def unarchive(self) -> UserDefinedFunction:
        """Unarchive the function."""
        updated = self.client.user_defined_functions.unarchive(function=self)
        self._update(updated)
        return self


class UserDefinedFunctionBase(ModelCreateUpdateBase):
    """Base class for UserDefinedFunction create and update models."""

    description: str | None = None
    function_inputs: list[FunctionInput] | None = None
    metadata: dict[str, str | float | bool] | None = None

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        "metadata": MappingHelper(
            proto_attr_path="metadata",
            update_field="metadata",
            converter=metadata_dict_to_proto,
        ),
        "function_inputs": MappingHelper(
            proto_attr_path="function_inputs",
            update_field="function_inputs",
            converter=_function_input_to_proto,  # type: ignore[arg-type]
        ),
    }


class UserDefinedFunctionCreate(
    UserDefinedFunctionBase, ModelCreate[CreateUserDefinedFunctionRequestProto]
):
    """Create model for UserDefinedFunction."""

    name: str
    expression: str
    user_notes: str | None = None

    def _get_proto_class(self) -> type[CreateUserDefinedFunctionRequestProto]:
        return CreateUserDefinedFunctionRequestProto


class UserDefinedFunctionUpdate(UserDefinedFunctionBase, ModelUpdate[UserDefinedFunctionProto]):
    """Update model for UserDefinedFunction.

    Once a function has dependents the server refuses to change `name`,
    `function_inputs`, or the output type. Check `dependents` first.
    """

    name: str | None = None
    expression: str | None = None
    is_archived: bool | None = None

    def _get_proto_class(self) -> type[UserDefinedFunctionProto]:
        return UserDefinedFunctionProto

    def _add_resource_id_to_proto(self, proto_msg: UserDefinedFunctionProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.user_defined_function_id = self._resource_id


class UserDefinedFunctionValidation(BaseModel):
    """The result of checking an expression before saving it.

    Attributes:
        valid: Whether the expression compiles.
        error: Why it failed, when it is not valid.
        output_type: The type the expression returns, when it is valid.
    """

    valid: bool
    error: str | None = None
    output_type: FunctionDataType | None = None


class FunctionDependents(BaseModel):
    """What would break if a function changed.

    Attributes:
        function_ids: IDs of other functions that call it.
        calculated_channel_ids: IDs of calculated channels that use it.
        rule_ids: IDs of rules that use it.
    """

    function_ids: list[str] = []
    calculated_channel_ids: list[str] = []
    rule_ids: list[str] = []

    @property
    def any(self) -> bool:
        """Whether anything depends on the function."""
        return bool(self.function_ids or self.calculated_channel_ids or self.rule_ids)
