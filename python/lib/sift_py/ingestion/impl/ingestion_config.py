"""
Internal module: This module contains implementation details that are not meant to be
used by consumers of this library and are not garaunteed to be stable.
"""

from ...grpc.transport import SiftChannel
from ..flow import FlowConfig
from sift_internal.convert.protobuf import try_cast_pb
from sift.ingestion_configs.v1.ingestion_configs_pb2 import (
    IngestionConfig,
    CreateIngestionConfigRequest,
    CreateIngestionConfigResponse,
    ListIngestionConfigsRequest,
    ListIngestionConfigsResponse,
    FlowConfig as FlowConfigPb,
)
from sift.ingestion_configs.v1.ingestion_configs_pb2_grpc import (
    IngestionConfigServiceStub,
)
from typing import cast, List, Optional


def get_ingestion_config_by_client_key(
    channel: SiftChannel,
    client_key: str,
) -> Optional[IngestionConfig]:
    """
    Returns `None` if no ingestion config can be matched with the provided `client_key`
    """

    svc = IngestionConfigServiceStub(channel)
    req = ListIngestionConfigsRequest(
        filter=f'client_key=="{client_key}"',
        page_token="",
        page_size=1,
    )
    res = cast(ListIngestionConfigsResponse, svc.ListIngestionConfigs(req))

    if len(res.ingestion_configs) == 0:
        return None
    else:
        return res.ingestion_configs[0]


def create_ingestion_config(
    channel: SiftChannel,
    asset_name: str,
    flows: List[FlowConfig],
    client_key: str,
    organization_id: Optional[str],
) -> IngestionConfig:
    """
    Creates a new ingestion config
    """

    svc = IngestionConfigServiceStub(channel)
    req = CreateIngestionConfigRequest(
        asset_name=asset_name,
        client_key=client_key,
        organization_id=organization_id or "",
        flows=[try_cast_pb(flow, FlowConfigPb) for flow in flows],
    )
    res = cast(CreateIngestionConfigResponse, svc.CreateIngestionConfig(req))
    return res.ingestion_config
