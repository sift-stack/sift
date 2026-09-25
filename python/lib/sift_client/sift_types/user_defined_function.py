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
from sift_client.sift_types.calculated_channel import CalculatedChannel  # noqa: TC001
from sift_client.sift_types.rule import Rule  # noqa: TC001
from sift_client.util.metadata import metadata_dict_to_proto, metadata_proto_to_dict

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.sift_types.user import User


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
        scalar: Whether the input is a literal value rather than a channel.
    """

    identifier: str
    data_type: FunctionDataType = FunctionDataType.NUMERIC
    scalar: bool = False

    @classmethod
    def _from_proto(cls, proto: FunctionInputProto) -> FunctionInput:
        return cls(
            identifier=proto.identifier,
            data_type=FunctionDataType(proto.data_type),
            scalar=proto.constant,
        )


def _function_input_to_proto(**kwargs) -> FunctionInputProto:
    """Convert a function input dict (from model_dump) into its proto form."""
    data_type = kwargs.get("data_type", FunctionDataType.NUMERIC)
    return FunctionInputProto(
        identifier=kwargs["identifier"],
        data_type=data_type.value if isinstance(data_type, FunctionDataType) else data_type,
        constant=kwargs.get("scalar", False),
    )


class UserDefinedFunctionVersion(BaseType[UserDefinedFunctionProto, "UserDefinedFunctionVersion"]):
    """One saved revision of a function. `id_` is the version ID."""

    user_defined_function_id: str
    version: int
    expression: str
    change_message: str
    change_notes: str
    function_inputs: list[FunctionInput]
    dependency_version_ids: list[str]
    created_date: datetime
    created_by_user_id: str
    output_type: FunctionDataType | None

    @classmethod
    def _from_proto(
        cls, proto: UserDefinedFunctionProto, sift_client: SiftClient | None = None
    ) -> UserDefinedFunctionVersion:
        return cls(
            proto=proto,
            id_=proto.user_defined_function_version_id,
            user_defined_function_id=proto.user_defined_function_id,
            version=proto.version,
            expression=proto.expression,
            change_message=proto.change_message,
            change_notes=proto.user_notes,
            function_inputs=[FunctionInput._from_proto(i) for i in proto.function_inputs],
            dependency_version_ids=[
                d.user_defined_function_version_id for d in proto.function_dependencies
            ],
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            output_type=(
                FunctionDataType(proto.function_output_type) if proto.function_output_type else None
            ),
            _client=sift_client,
        )

    @property
    def function(self) -> UserDefinedFunction:
        """Fetch the function this version belongs to."""
        return self.client.user_defined_functions.get(
            user_defined_function_id=self.user_defined_function_id
        )

    @property
    def created_by(self) -> User:
        """Fetch the User that created this version."""
        return self.client.users.get(user_id=self.created_by_user_id)

    @property
    def dependencies(self) -> list[UserDefinedFunctionVersion]:
        """Fetch the function versions this one calls."""
        if not self.dependency_version_ids:
            return []
        return self.client.user_defined_functions.versions.batch_get(
            versions=self.dependency_version_ids
        )


class UserDefinedFunction(BaseType[UserDefinedFunctionProto, "UserDefinedFunction"]):
    """A reusable expression that calculated channels and rules can call.

    Version agnostic. The expression, inputs, and output type live on
    `latest_version`; each save produces a new one.
    """

    name: str
    description: str
    metadata: dict[str, str | float | bool]
    created_date: datetime
    modified_date: datetime
    created_by_user_id: str
    modified_by_user_id: str
    is_archived: bool
    latest_version: UserDefinedFunctionVersion

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
            metadata=metadata_proto_to_dict(proto.metadata),  # type: ignore
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            is_archived=proto.is_archived,
            latest_version=UserDefinedFunctionVersion._from_proto(proto, sift_client),
            archived_date=(
                proto.archived_date.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("archived_date")
                else None
            ),
            _client=sift_client,
        )

    @property
    def versions(self) -> list[UserDefinedFunctionVersion]:
        """Fetch every version of this function, newest first."""
        return self.client.user_defined_functions.versions.list_(user_defined_function=self)

    @property
    def created_by(self) -> User:
        """Fetch the User that created this function."""
        return self.client.users.get(user_id=self.created_by_user_id)

    @property
    def modified_by(self) -> User:
        """Fetch the User that last modified this function."""
        return self.client.users.get(user_id=self.modified_by_user_id)

    def update(self, update: UserDefinedFunctionUpdate | dict) -> UserDefinedFunction:
        """Update the function. This creates a new version.

        Args:
            update: The update to apply. See UserDefinedFunctionUpdate for updatable fields.

        Returns:
            The updated function.
        """
        updated = self.client.user_defined_functions.update(
            user_defined_function=self, update=update
        )
        self._update(updated)
        return self

    def archive(self) -> UserDefinedFunction:
        """Archive the function."""
        updated = self.client.user_defined_functions.archive(user_defined_function=self)
        self._update(updated)
        return self

    def unarchive(self) -> UserDefinedFunction:
        """Unarchive the function."""
        updated = self.client.user_defined_functions.unarchive(user_defined_function=self)
        self._update(updated)
        return self


class UserDefinedFunctionBase(ModelCreateUpdateBase):
    """Base class for UserDefinedFunction create and update models."""

    description: str | None = None
    metadata: dict[str, str | float | bool] | None = None

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        "metadata": MappingHelper(
            proto_attr_path="metadata",
            update_field="metadata",
            converter=metadata_dict_to_proto,
        ),
    }


class UserDefinedFunctionCreate(
    UserDefinedFunctionBase, ModelCreate[CreateUserDefinedFunctionRequestProto]
):
    """Create model for UserDefinedFunction."""

    name: str
    expression: str
    """The CEL expression, referring to inputs positionally as `$1`, `$2`."""
    function_inputs: list[FunctionInput]
    change_notes: str | None = None

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        **UserDefinedFunctionBase._to_proto_helpers,
        "function_inputs": MappingHelper(
            proto_attr_path="function_inputs",
            update_field="function_inputs",
            converter=_function_input_to_proto,  # type: ignore[arg-type]
        ),
        "change_notes": MappingHelper(proto_attr_path="user_notes", update_field="user_notes"),
    }

    def _get_proto_class(self) -> type[CreateUserDefinedFunctionRequestProto]:
        return CreateUserDefinedFunctionRequestProto


class UserDefinedFunctionUpdate(UserDefinedFunctionBase, ModelUpdate[UserDefinedFunctionProto]):
    """Update model for UserDefinedFunction.

    Once a function has dependents the server refuses to change `name`,
    `function_inputs`, or the output type. Check `get_where_used` first.
    """

    name: str | None = None
    expression: str | None = None
    function_inputs: list[FunctionInput] | None = None
    is_archived: bool | None = None

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        **UserDefinedFunctionBase._to_proto_helpers,
        "function_inputs": MappingHelper(
            proto_attr_path="function_inputs",
            update_field="function_inputs",
            converter=_function_input_to_proto,  # type: ignore[arg-type]
        ),
    }

    def _get_proto_class(self) -> type[UserDefinedFunctionProto]:
        return UserDefinedFunctionProto

    def _add_resource_id_to_proto(self, proto_msg: UserDefinedFunctionProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.user_defined_function_id = self._resource_id


class UserDefinedFunctionValidation(BaseModel):
    """The result of checking an expression before saving it.

    Attributes:
        is_valid: Whether the expression compiles.
        error: Why it failed, when it is not valid.
        output_type: The type the expression returns, when it is valid.
    """

    is_valid: bool
    error: str | None = None
    output_type: FunctionDataType | None = None


class FunctionUsage(BaseModel):
    """Where a function is used.

    Attributes:
        functions: Other functions that call it.
        calculated_channels: Calculated channels that use it.
        rules: Rules that use it.
    """

    functions: list[UserDefinedFunction] = []
    calculated_channels: list[CalculatedChannel] = []
    rules: list[Rule] = []

    @property
    def is_used(self) -> bool:
        """Whether anything uses the function."""
        return bool(self.functions or self.calculated_channels or self.rules)
