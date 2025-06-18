from __future__ import annotations

from typing import Type, TYPE_CHECKING

from sift.metadata.v1.metadata_pb2 import  MetadataValue as MetadataProto

from sift_client.types.base import ModelUpdate, BaseType

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class MetadataUpdate(ModelUpdate):
    ...

class MetadataValue(BaseType):
    key: str
    value: str | float | bool

    @classmethod
    def _from_proto(cls, proto: MetadataProto, sift_client: SiftClient = None) -> MetadataValue:
        return cls(
           key=proto.key.name,
            value=proto.string_value or proto.float_value or proto.bool_value,
            _client=sift_client,
        )