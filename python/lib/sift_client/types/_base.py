from __future__ import annotations

from abc import ABC, abstractmethod
from typing import TYPE_CHECKING, Any, Callable, Generic, Optional, Type, TypeVar

from google.protobuf import field_mask_pb2, message
from pydantic import BaseModel, ConfigDict, PrivateAttr

if TYPE_CHECKING:
    from sift_client.client import SiftClient


ProtoT = TypeVar("ProtoT", bound=message.Message)
SelfT = TypeVar("SelfT", bound="BaseType")


class BaseType(BaseModel, Generic[ProtoT, SelfT], ABC):
    model_config = ConfigDict(frozen=True)

    _client: SiftClient | None = None

    @property
    def client(self) -> SiftClient:
        if self._client is None:
            raise AttributeError(
                "Sift client not set. Please retrieve with the SiftClient to use this method."
            )
        return self._client

    @classmethod
    @abstractmethod
    def _from_proto(cls, proto: ProtoT, sift_client: SiftClient | None = None) -> SelfT: ...

    def _apply_client_to_instance(self, client: SiftClient) -> None:
        # This bypasses the frozen status of the model
        self.__dict__["_client"] = client

    def _update(self, other: BaseType[ProtoT, SelfT]) -> BaseType[ProtoT, SelfT]:
        """Update this instance with the values from another instance"""
        # This bypasses the frozen status of the model
        for key in other.__class__.model_fields.keys():
            if key in self.model_fields:
                self.__dict__.update({key: getattr(other, key)})
        return self


class MappingHelper(BaseModel):
    """Helper class for mapping fields to proto attributes and update fields
    Args:
        proto_attr_path: The path to the proto attribute to update
        update_field: The optional field to set in the update mask
        converter: The optional proto class or function to use for converting the value of associated field.
    """

    proto_attr_path: str
    update_field: str | None = None
    converter: Type[Any] | Callable[[Any], Any] | None = None


# TODO: how to handle nulling fields, needs to be default value for the type
class ModelUpdate(BaseModel, Generic[ProtoT], ABC):
    """Base class for Pydantic models that generate proto patches with field masks"""

    model_config = ConfigDict(frozen=False)

    _resource_id: Optional[Any] = PrivateAttr(default=None)
    _to_proto_helpers: dict[str, MappingHelper] = PrivateAttr(default={})

    def __init__(self, **data: Any):
        super().__init__(**data)
        if self._to_proto_helpers:
            data = self.model_dump()
            for expected_field in self._to_proto_helpers.keys():
                if expected_field not in data:
                    raise ValueError(
                        f"MappingHelper created for {expected_field} but {self.__class__.__name__} has no matching variable names."
                    )

    @property
    def resource_id(self):
        return self._resource_id

    @resource_id.setter
    def resource_id(self, value):
        self._resource_id = value

    def to_proto_with_mask(self) -> tuple[ProtoT, field_mask_pb2.FieldMask]:
        """Convert to proto with field mask"""
        # Get the corresponding proto class
        proto_cls: Type[ProtoT] = self._get_proto_class()
        proto_msg = proto_cls()

        # Get only explicitly set fields, including those set to None
        data = self.model_dump(exclude_unset=True, exclude_none=False)
        paths = self._build_proto_and_paths(proto_msg, data)

        self._add_resource_id_to_proto(proto_msg)
        mask = field_mask_pb2.FieldMask(paths=paths)
        return proto_msg, mask

    def _build_proto_and_paths(
        self, proto_msg, data, prefix="", already_setting_path_override=False
    ) -> list[str]:
        """Recursively build proto message and collect field paths
        Args:
            proto_msg: The proto message to build
            data: The data to build the proto message with
            prefix: The prefix to add to the field path
            already_setting_path_override: If true, skips path helper handling for this field.This ensures we don't get stuck in a loop if the expanded path includes the field name we're overriding.
        """
        paths = []
        for field_name, value in data.items():
            path = f"{prefix}.{field_name}" if prefix else field_name

            if not already_setting_path_override and field_name in self._to_proto_helpers:
                mapping_helper = self._to_proto_helpers[field_name]
                # Expand the proto path to a dictionary and parse recursively
                for layer in reversed(mapping_helper.proto_attr_path.split(".")):
                    temp = {}
                    temp[layer] = value
                    value = temp
                sub_paths = self._build_proto_and_paths(
                    proto_msg, value, "", already_setting_path_override=True
                )
                if mapping_helper.update_field:
                    paths.append(mapping_helper.update_field)
            elif isinstance(value, dict):
                if field_name in self._to_proto_helpers:
                    assert self._to_proto_helpers[field_name].converter, (
                        f"Expecting to run a coverter given a helper was defined for: {field_name}"
                    )
                    sub_paths = self._build_proto_and_paths(
                        proto_msg,
                        {field_name: self._to_proto_helpers[field_name].converter(value)},  # type: ignore[misc]
                        "",
                        already_setting_path_override=True,
                    )
                    paths.extend(sub_paths)
                else:
                    # Get the submessage
                    sub_msg = getattr(proto_msg, field_name)
                    # Recursively process nested fields
                    sub_paths = self._build_proto_and_paths(
                        sub_msg,
                        value,
                        path,
                        already_setting_path_override=already_setting_path_override,
                    )
                    paths.extend(sub_paths)
            elif isinstance(value, list):
                repeated_field = getattr(proto_msg, field_name)
                del repeated_field[:]  # Remove all existing items
                try:
                    repeated_field.extend(value)  # Add all new values
                except TypeError as e:
                    if field_name in self._to_proto_helpers:
                        assert self._to_proto_helpers[field_name].converter, (
                            f"Expecting to run a coverter given a helper was defined for: {field_name}"
                        )
                        for item in value:
                            repeated_field.append(
                                self._to_proto_helpers[field_name].converter(**item)  # type: ignore
                            )
                    else:
                        raise e
                paths.append(path)
            else:
                try:
                    setattr(proto_msg, field_name, value)
                    paths.append(path)
                except TypeError:
                    raise TypeError(
                        f"Can't set {field_name} to {value} on {proto_msg.__class__.__name__}"
                    )

        return paths

    def _get_proto_class(self) -> Type[ProtoT]:
        """Get the corresponding proto class - override in subclasses since typing is not strict."""
        raise NotImplementedError("Subclasses must implement this")

    def _add_resource_id_to_proto(self, proto_msg: ProtoT):
        """Assigns a resource ID (such as Asset ID) to the proto message"""
        raise NotImplementedError("Subclasses must implement this")
