from typing import List, Optional, Sequence, cast

import grpc
from sift.ingestion_configs.v2.ingestion_configs_pb2 import (
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
from sift.ingestion_configs.v2.ingestion_configs_pb2 import (
    FlowConfig as FlowConfigPb,
)
from sift.ingestion_configs.v2.ingestion_configs_pb2_grpc import (
    IngestionConfigServiceStub,
)

from sift_py.grpc.transport import SiftChannel
from sift_py.ingestion._internal.error import IngestionValidationError
from sift_py.ingestion.flow import FlowConfig

# The default max message size for our gRPC server.
GRPC_MAX_MESSAGE_SIZE = 4194304


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
        flows=[flow.as_pb(FlowConfigPb) for flow in flows],
    )
    print(len(req.SerializeToString()))
    if len(req.SerializeToString()) > GRPC_MAX_MESSAGE_SIZE:
        return create_ingestion_config_with_iteratively(
            channel,
            asset_name,
            flows,
            client_key,
            organization_id,
        )
    else:
        res = cast(CreateIngestionConfigResponse, svc.CreateIngestionConfig(req))
        return res.ingestion_config


def create_ingestion_config_with_iteratively(
    channel: SiftChannel,
    asset_name: str,
    flows: List[FlowConfig],
    client_key: str,
    organization_id: Optional[str],
) -> IngestionConfig:
    """
    Creates a new ingestion config by adding flows one at a time if the request size exceeds the gRPC max message size.
    """
    svc = IngestionConfigServiceStub(channel)

    req = CreateIngestionConfigRequest(
        asset_name=asset_name,
        client_key=client_key,
        organization_id=organization_id or "",
        flows=[],
    )
    res = cast(CreateIngestionConfigResponse, svc.CreateIngestionConfig(req))
    ingestion_config = res.ingestion_config

    for flow in flows:
        try:
            create_flow_configs(channel, ingestion_config.ingestion_config_id, [flow])
        except grpc.RpcError as e:
            if e.code() == grpc.StatusCode.RESOURCE_EXHAUSTED:
                raise IngestionValidationError(
                    f"Flow {flow.name} is too large. Split this flow up to meet gRPC message size requirements."
                )
            elif e.code() == grpc.StatusCode.ALREADY_EXISTS:
                continue
            else:
                raise

    return ingestion_config


def get_ingestion_config_flow_names(
    channel: SiftChannel,
    ingestion_config_id: str,
) -> List[str]:
    """
    Gets all names of flow configs of an ingestion config.
    """
    flows = get_ingestion_config_flows(channel, ingestion_config_id)
    return [flow.name for flow in flows]


def get_ingestion_config_flows(
    channel: SiftChannel,
    ingestion_config_id: str,
    page_size: int = 1_000,
) -> List[FlowConfigPb]:
    svc = IngestionConfigServiceStub(channel)

    flows: List[FlowConfigPb] = []

    req = ListIngestionConfigFlowsRequest(
        ingestion_config_id=ingestion_config_id,
        page_size=page_size,
        filter="",
    )
    try:
        res = cast(ListIngestionConfigFlowsResponse, svc.ListIngestionConfigFlows(req))
    except grpc.RpcError as e:
        if e.code() != grpc.StatusCode.RESOURCE_EXHAUSTED or page_size == 1:
            raise
        return get_ingestion_config_flows(channel, ingestion_config_id, page_size=1)

    for flow in res.flows:
        flows.append(flow)

    page_token = res.next_page_token

    while len(page_token) > 0:
        req = ListIngestionConfigFlowsRequest(
            ingestion_config_id=ingestion_config_id,
            page_size=page_size,
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
    flow_configs: Sequence[FlowConfig],
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
