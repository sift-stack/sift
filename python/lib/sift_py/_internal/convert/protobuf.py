from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Generic, Type, TypeVar

from google.protobuf.message import Message

ProtobufMessage = Message

T = TypeVar("T", bound=ProtobufMessage)


class AsProtobuf(ABC, Generic[T]):
    """
    Abstract base class used to create create sub-types that can be treated
    as an object that can be converted into an instance of `ProtobufMessage`.

    If there are multiple possible protobuf targets then `as_pb` may be overloaded.
    """

    @abstractmethod
    def as_pb(self, klass: Type[T]) -> T:
        """
        Performs the conversion into a sub-type of `ProtobufMessage`.
        """
        pass

    @classmethod
    @abstractmethod
    def from_pb(cls, message: T) -> T:
        """
        Converts a protobuf object to the type of the sub-class class.
        """
        pass
