from typing import List, Optional, cast

from sift.ingestion_configs.v1.ingestion_configs_pb2 import (
    CreateIngestionConfigFlowsRequest,
    CreateIngestionConfigFlowsResponse,
    CreateIngestionConfigRequest,
    CreateIngestionConfigResponse,
    IngestionConfig,
    ListIngestionConfigFlowsRequest,
    ListIngestionConfigFlowsResponse,
    ListIngestionConfigsRequest,
    ListIngestionConfigsResponse,
)
from sift.ingestion_configs.v1.ingestion_configs_pb2 import (
    FlowConfig as FlowConfigPb,
)
from sift.ingestion_configs.v1.ingestion_configs_pb2_grpc import (
    IngestionConfigServiceStub,
)

from sift_py.grpc.transport import SiftChannel
from sift_py.ingestion.flow import FlowConfig


def get_ingestion_config_by_client_key(
    channel: SiftChannel,
    client_key: str,
) -> Optional[IngestionConfig]:
    """
    Returns `None` if no ingestion config can be matched with the provided `client_key`
    """
    results = list_ingestion_configs(channel, f'client_key=="{client_key}"', page_size=1)

    if len(results) == 0:
        return None
    else:
        return results[0]


def list_ingestion_configs(
    channel: SiftChannel,
    filter: str,
    page_size: int = 1_000,
) -> List[IngestionConfig]:
    """
    Returns a list of ingestion configs that can be matched with the provided `filter`.
    """
    svc = IngestionConfigServiceStub(channel)

    results: List[IngestionConfig] = []
    next_page_token = ""
    while True:
        req = ListIngestionConfigsRequest(
            page_token=next_page_token,
            filter=filter,
            page_size=page_size,
        )
        res = cast(ListIngestionConfigsResponse, svc.ListIngestionConfigs(req))
        results.extend(res.ingestion_configs)

        next_page_token = res.next_page_token
        if not next_page_token:
            break

    return results


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
        flows=[flow.as_pb(FlowConfigPb) for flow in flows],
    )
    res = cast(CreateIngestionConfigResponse, svc.CreateIngestionConfig(req))
    return res.ingestion_config


def get_ingestion_config_flow_names(
    channel: SiftChannel,
    ingestion_config_id: str,
) -> List[str]:
    """
    Gets all names of flow configs of an ingestion config.
    """
    flows = get_ingestion_config_flows(channel, ingestion_config_id)
    breakpoint()
    return [flow.name for flow in flows]


def get_ingestion_config_flows(
    channel: SiftChannel, ingestion_config_id: str
) -> List[FlowConfigPb]:
    svc = IngestionConfigServiceStub(channel)

    flows: List[FlowConfigPb] = []

    req = ListIngestionConfigFlowsRequest(
        ingestion_config_id=ingestion_config_id,
        page_size=1_000,
        filter="",
    )
    res = cast(ListIngestionConfigFlowsResponse, svc.ListIngestionConfigFlows(req))

    for flow in res.flows:
        flows.append(flow)

    page_token = res.next_page_token

    while len(page_token) > 0:
        req = ListIngestionConfigFlowsRequest(
            ingestion_config_id=ingestion_config_id,
            page_size=1_000,
            filter="",
            page_token=page_token,
        )
        res = cast(ListIngestionConfigFlowsResponse, svc.ListIngestionConfigFlows(req))

        for flow in res.flows:
            flows.append(flow)

        page_token = res.next_page_token

    return flows


def create_flow_configs(
    channel: SiftChannel,
    ingestion_config_id: str,
    flow_configs: List[FlowConfig],
):
    """
    Adds flow configs to an existing ingestion config.
    """
    svc = IngestionConfigServiceStub(channel)
    req = CreateIngestionConfigFlowsRequest(
        ingestion_config_id=ingestion_config_id,
        flows=[f.as_pb(FlowConfigPb) for f in flow_configs],
    )
    _ = cast(CreateIngestionConfigFlowsResponse, svc.CreateIngestionConfigFlows(req))
