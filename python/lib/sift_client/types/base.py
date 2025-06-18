from __future__ import annotations

from abc import ABC, abstractmethod
from typing import TYPE_CHECKING,Dict, Any, Type, TypeVar, Optional, List, Iterable, Generic

from pydantic import BaseModel, PrivateAttr

from google.protobuf import field_mask_pb2, message

if TYPE_CHECKING:
    from sift_client.client import SiftClient


T = TypeVar('T', bound=BaseModel)
ProtoT = TypeVar('ProtoT', bound=message.Message)

class BaseType(BaseModel, ABC):
    _client: Optional["SiftClient"] = PrivateAttr(default=None)

    class Config:
        frozen = True

    @property
    def client(self) -> "SiftClient":
        if self._client is None:
            raise ValueError("Sift client not set. Please retrieve with the SiftClient to use this method.")
        return self._client

    @classmethod
    @abstractmethod
    def _from_proto(cls, proto: ProtoT, sift_client: Optional["SiftClient"] = None) -> "BaseType":
        ...

class ModelUpdate(BaseModel, ABC):
    """Base class for Pydantic models that generate proto patches with field masks"""
    def to_proto_with_mask(self) -> tuple[message.Message, field_mask_pb2.FieldMask]:
        """Convert to proto with field mask"""
        # Get the corresponding proto class
        proto_cls = self._get_proto_class()
        proto_msg = proto_cls()

        # Get only explicitly set fields
        data = self.model_dump(exclude_unset=True, exclude_none=True)
        paths = self._build_proto_and_paths(proto_msg, data)

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
            else:
                setattr(proto_msg, field_name, value)
                paths.append(path)

        return paths

    def _get_proto_class(self) -> Type[message.Message]:
        """Get the corresponding proto class - override in subclasses"""
        raise NotImplementedError("Subclasses must implement this")