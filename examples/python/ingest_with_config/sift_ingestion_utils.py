from google.protobuf import timestamp_pb2
from typing import Any, cast, Generator, Optional
import grpc
from sift.common.type.v1.channel_data_type_pb2  import ChannelDataType
import sift.ingest.v1.ingest_pb2 as ingest
import sift.ingest.v1.ingest_pb2_grpc as ingest_grpc
import sift.ingestion_configs.v1.ingestion_configs_pb2 as ingestconf
import sift.ingestion_configs.v1.ingestion_configs_pb2_grpc as ingestconf_grpc
import sift.runs.v2.runs_pb2 as run
import sift.runs.v2.runs_pb2_grpc as run_grpc

def use_secure_channel(api_key: str, base_uri: str) -> grpc.Channel:
    """
    Produces channel that is used to create a secure connection to a gRPC server.
    This is intended to be used by all stubs.
    """
    credentials = grpc.ssl_channel_credentials()
    call_credentials = grpc.access_token_call_credentials(api_key)
    composite_credentials = grpc.composite_channel_credentials(credentials, call_credentials) 
    return grpc.secure_channel(base_uri,composite_credentials)

def create_double_type_channel_config(
    name: str,
    description: Optional[str],
    component: Optional[str],
    unit: Optional[str],
) -> ingestconf.ChannelConfig:
    """
    Creates a channel config for values that are double precision floating points.
    """
    config = ingestconf.ChannelConfig(name=name)
    config.data_type = ChannelDataType.CHANNEL_DATA_TYPE_DOUBLE

    if component is not None:
        config.component = component

    if unit is not None:
        config.unit = unit

    if description is not None:
        config.description = description

    return config

def create_flow_config(flow_name: str, *channel_configs: ingestconf.ChannelConfig) -> ingestconf.FlowConfig:
    """
    Creates a flow config that describes a group of channels that will telemeter data.
    """
    config = ingestconf.FlowConfig(name=flow_name)

    for channel_config in channel_configs:
        config.channels.append(channel_config)

    return config

def create_ingestion_config(
    channel: grpc.Channel,
    asset_name: str,
    *flow_configs: ingestconf.FlowConfig,
) -> ingestconf.IngestionConfig:
    """
    Reaches out to the Sift API to create an ingestion config that constitutes essential metadata for ingestion.
    """

    request = ingestconf.CreateIngestionConfigRequest(asset_name=asset_name)

    for flow_config in flow_configs:
        request.flows.append(flow_config)

    response = ingestconf_grpc.IngestionConfigServiceStub(channel).CreateIngestionConfig(request)

    return cast(ingestconf.CreateIngestionConfigResponse, response).ingestion_config

def create_run(
    channel: grpc.Channel,
    name: str,
    description: str,
    organization_id: Optional[str],
    start_time: Optional[timestamp_pb2.Timestamp],
    stop_time: Optional[timestamp_pb2.Timestamp],
    *tags: str,
) -> run.Run:
    """
    Reaches out to the Sift API to create a run that will be associated with ingested data.
    """

    request = run.CreateRunRequest(name=name, description=description)

    for tag in tags:
        request.tags.append(tag)

    if organization_id is not None:
        request.organization_id = organization_id

    if start_time is not None:
        request.start_time.CopyFrom(start_time)

    if stop_time is not None:
        request.stop_time.CopyFrom(stop_time)

    response = run_grpc.RunServiceStub(channel).CreateRun(request)

    return cast(run.CreateRunResponse, response).run

def ingest_with_config(
    channel: grpc.Channel,
    ingestion_iter: Generator[ingest.IngestWithConfigDataStreamRequest, Any, None],
):
    """
    Consume the ingestion_iter generator and send the data produced to the Sift's ingestion API.
    Data should be available to view shortly after this function concludes
    """
    try:
        ingest_grpc.IngestServiceStub(channel).IngestWithConfigDataStream(ingestion_iter)
    except Exception as e:
        print(f"Something went wrong during ingestion: {e}")

def delete_run(run_id: str, api_key: str, base_uri: str):
    """
    Handy utility to delete your test runs
    """

    with use_secure_channel(api_key, base_uri) as channel:
        try:
            req = run.DeleteRunRequest(run_id=run_id)
            run_grpc.RunServiceStub(channel).DeleteRun(req)
            print(f"Deleted run {run_id}")
        except Exception as e:
            print(f"Something went wrong trying to delete the run: {e}")
        finally:
            channel.close()
