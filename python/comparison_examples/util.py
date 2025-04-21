# uses grplib instead of gprc

from typing import Optional, Type, cast

from grpclib.client import Channel, Stream
from grpclib.const import Cardinality
from grpclib.metadata import Deadline, _Metadata, _MetadataLike
from grpclib.stream import _RecvType, _SendType
from multidict import MultiDict
from sift_py.grpc.transport import SiftChannelConfig

# setup Channel, similar to the `use_sift_channel`


# Adds authorization header
class SiftChannel(Channel):
    def __init__(self, host, port, *, token=None, **kwargs):
        super().__init__(host, port, **kwargs)
        self.token = token

    def request(
        self,
        name: str,
        cardinality: Cardinality,
        request_type: Type[_SendType],
        reply_type: Type[_RecvType],
        *,
        timeout: Optional[float] = None,
        deadline: Optional[Deadline] = None,
        metadata: Optional[_MetadataLike] = None,
    ) -> Stream[_SendType, _RecvType]:
        metadata = cast(_Metadata, MultiDict(metadata or ()))

        if self.token:
            metadata["authorization"] = f"Bearer {self.token}"

        return super().request(
            name,
            cardinality,
            request_type,
            reply_type,
            timeout=timeout,
            deadline=deadline,
            metadata=metadata,
        )


def use_sift_betterproto_channel(config: SiftChannelConfig) -> SiftChannel:
    # TODO: keep alive settings if not default
    return SiftChannel(host=config["uri"], port=443, ssl=True, token=config["apikey"])
