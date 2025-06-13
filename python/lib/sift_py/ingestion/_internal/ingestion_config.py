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

from sift_py.error import ProtobufMaxSizeExceeded, raise_if_too_large
from sift_py.grpc.transport import SiftChannel
from sift_py.ingestion._internal.error import IngestionValidationError
from sift_py.ingestion.flow import FlowConfig


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
    Creates a new ingestion config.
    """

    svc = IngestionConfigServiceStub(channel)
    req = CreateIngestionConfigRequest(
        asset_name=asset_name,
        client_key=client_key,
        organization_id=organization_id or "",
        flows=[flow.as_pb(FlowConfigPb) for flow in flows],
    )

    try:
        raise_if_too_large(req)
        res = cast(CreateIngestionConfigResponse, svc.CreateIngestionConfig(req))
        return res.ingestion_config
    except ProtobufMaxSizeExceeded:
        return create_ingestion_config_batched(
            channel,
            asset_name,
            flows,
            client_key,
            organization_id,
            batch_size=max(1, len(flows) // 2),
        )


def create_ingestion_config_batched(
    channel: SiftChannel,
    asset_name: str,
    flows: List[FlowConfig],
    client_key: str,
    organization_id: Optional[str],
    batch_size: int,
) -> IngestionConfig:
    """
    Creates a new ingestion config by adding flows in batches.
    """
    svc = IngestionConfigServiceStub(channel)

    req = CreateIngestionConfigRequest(
        asset_name=asset_name,
        client_key=client_key,
        organization_id=organization_id or "",
        flows=[],
    )
    res = cast(CreateIngestionConfigResponse, svc.CreateIngestionConfig(req))

    create_flow_configs(
        channel, res.ingestion_config.ingestion_config_id, flows, batch_size=batch_size
    )

    return res.ingestion_config


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

    except grpc.RpcError as e:
        if e.code() != grpc.StatusCode.RESOURCE_EXHAUSTED or page_size == 1:
            raise
        return get_ingestion_config_flows(
            channel, ingestion_config_id, page_size=max(1, page_size // 2)
        )


def create_flow_configs(
    channel: SiftChannel,
    ingestion_config_id: str,
    flow_configs: Sequence[FlowConfig],
    batch_size: int = 1_000,
):
    """
    Adds flow configs to an existing ingestion config.
    """
    processed_flows = []
    try:
        svc = IngestionConfigServiceStub(channel)
        for i in range(0, len(flow_configs), batch_size):
            batch = flow_configs[i : i + batch_size]
            req = CreateIngestionConfigFlowsRequest(
                ingestion_config_id=ingestion_config_id,
                flows=[f.as_pb(FlowConfigPb) for f in batch],
            )
            _ = cast(CreateIngestionConfigFlowsResponse, svc.CreateIngestionConfigFlows(req))
            processed_flows.extend([flow_config.name for flow_config in batch])
    except grpc.RpcError as e:
        if e.code() == grpc.StatusCode.RESOURCE_EXHAUSTED and batch_size > 1:
            missed_flows = [
                flow_config
                for flow_config in flow_configs
                if flow_config.name not in processed_flows
            ]
            return create_flow_configs(
                channel, ingestion_config_id, missed_flows, batch_size=max(1, batch_size // 2)
            )
        else:
            raise
