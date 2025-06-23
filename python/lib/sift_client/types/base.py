from __future__ import annotations

from abc import ABC, abstractmethod
from typing import TYPE_CHECKING, Any, Generic, Optional, Type, TypeVar

from google.protobuf import field_mask_pb2, message
from pydantic import BaseModel, PrivateAttr

if TYPE_CHECKING:
    from sift_client.client import SiftClient


ProtoT = TypeVar("ProtoT", bound=message.Message)
SelfT = TypeVar("SelfT", bound="BaseType")


class BaseType(BaseModel, Generic[ProtoT, SelfT], ABC):
    _client: SiftClient | None = None

    class Config:
        frozen = True

    @property
    def client(self) -> SiftClient:
        if self._client is None:
            raise ValueError(
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
        for key in other.model_fields.keys():
            if key in self.model_fields:
                self.__dict__.update({key: getattr(other, key)})
        return self


# TODO: how to handle nulling fields (undefined or something?)
class ModelUpdate(BaseModel, Generic[ProtoT], ABC):
    """Base class for Pydantic models that generate proto patches with field masks"""

    _resource_id: Optional[Any] = PrivateAttr(default=None)

    class Config:
        frozen = False

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

        # Get only explicitly set fields
        data = self.model_dump(exclude_unset=True, exclude_none=True)
        paths = self._build_proto_and_paths(proto_msg, data)

        self._add_resource_id_to_proto(proto_msg)

        return proto_msg, field_mask_pb2.FieldMask(paths=paths)

    def _build_proto_and_paths(self, proto_msg, data, prefix="") -> list[str]:
        """Recursively build proto message and collect field paths"""
        paths = []
        for field_name, value in data.items():
            path = f"{prefix}.{field_name}" if prefix else field_name

            if isinstance(value, dict):
                # Get the submessage
                sub_msg = getattr(proto_msg, field_name)
                # Recursively process nested fields
                sub_paths = self._build_proto_and_paths(sub_msg, value, path)
                paths.extend(sub_paths)
            elif isinstance(value, list):
                repeated_field = getattr(proto_msg, field_name)
                del repeated_field[:]  # Remove all existing items
                repeated_field.extend(value)  # Add all new values
                paths.append(path)
            else:
                setattr(proto_msg, field_name, value)
                paths.append(path)

        return paths

    def _get_proto_class(self) -> Type[ProtoT]:
        """Get the corresponding proto class - override in subclasses"""
        raise NotImplementedError("Subclasses must implement this")

    def _add_resource_id_to_proto(self, proto_msg: ProtoT):
        """Assigns a resource ID (such as Asset ID) to the proto message"""
        raise NotImplementedError("Subclasses must implement this")
