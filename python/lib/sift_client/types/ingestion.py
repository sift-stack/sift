from __future__ import annotations

from typing import TYPE_CHECKING

from sift.ingestion_configs.v2.ingestion_configs_pb2 import IngestionConfig as IngestionConfigProto

from sift_client.types._base import BaseType

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class IngestionConfig(BaseType[IngestionConfigProto, "IngestionConfig"]):
    """
    Model of the Sift Ingestion Config.
    """

    id: str
    asset_id: str
    client_key: str

    @classmethod
    def _from_proto(
        cls, proto: IngestionConfigProto, sift_client: SiftClient | None = None
    ) -> "IngestionConfig":
        return cls(
            id=proto.ingestion_config_id,
            asset_id=proto.asset_id,
            client_key=proto.client_key,
            _client=sift_client,
        )
