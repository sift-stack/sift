from abc import ABC, abstractmethod
from typing import Optional, Type, TypeVar, cast

from google.protobuf.message import Message

ProtobufMessage = Message


class AsProtobuf(ABC):
    """
    Abstract base class used to create create sub-types that can be treated
    as an object that can be converted into an instance of `ProtobufMessage`.
    """

    @abstractmethod
    def as_pb(self, klass: Type[ProtobufMessage]) -> Optional[ProtobufMessage]:
        """
        Performs the conversion into a sub-type of `ProtobufMessage`. Should return `None`
        if conversion fails.
        """
        pass


T = TypeVar("T", bound=ProtobufMessage)


def try_cast_pb(value: AsProtobuf, target_klass: Type[T]) -> T:
    """
    Tries to cast the `value` to `target_klass`, otherwise, returns a `TypeError`.
    """
    value_pb = value.as_pb(target_klass)
    if isinstance(value_pb, target_klass):
        return cast(target_klass, value_pb)
    raise TypeError(
        f"Expected a '{target_klass.__module__}{target_klass.__name__}' but got {value.__module__}{value.__class__.__name__}"
    )
