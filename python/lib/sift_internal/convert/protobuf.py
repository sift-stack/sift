from abc import ABC, abstractmethod
from google.protobuf.message import Message
from typing import cast, Type, TypeVar

ProtobufMessage = Message


class AsProtobuf(ABC):
    """
    Flexible abstract class to create classes that can be converted into different protobuf
    targets. All conversion logic goes inside of `as_pb` and should do a runtime check for
    class name, module name, and/or things of the like using the `klass` argument available
    to generate the appropriate protobuf target.

    The `as_pb` method should rarely be used directly since it returns the super-type. Prefer
    to use the `cast_pb` function to convert sub-types of `AsProtobuf` to the concrete protobuf type.
    """

    @abstractmethod
    def as_pb(self, klass: Type[Message]) -> Message:
        pass


T = TypeVar("T", bound=ProtobufMessage)


def try_convert_pb(val: AsProtobuf, target_type: Type[T]) -> T:
    """
    Utility to convert sub-types of `AsProtobuf` to its concrete protobuf type.
    Will raise a `TypeError` if the underlying type of `val` is not `target_type`.
    """

    pb_value = val.as_pb(target_type)

    if isinstance(pb_value, target_type):
        return cast(target_type, pb_value)
    else:
        raise TypeError(
            f"Expected `val` to be a '{target_type.__module__}.{target_type.__name__}' but it is a '{val.__module__}.{val.__class__.__name__}'."
        )
