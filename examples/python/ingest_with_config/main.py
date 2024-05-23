from dotenv import load_dotenv
from google.protobuf.timestamp_pb2 import Timestamp
from typing import Any, Generator, List, Optional, Tuple
import math
import os
import sift_ingestion_utils as ingest
import sift.ingest.v1.ingest_pb2 as ingestpb
import sift.ingestion_configs.v1.ingestion_configs_pb2 as ingestconf
import sift.runs.v2.runs_pb2 as runpb

def main():
    """
    This is an example script demonstrating how to use Sift's IngestService_IngestWithConfigDataStream
    API. In this example we will be creating an asset that contains two channels: velocity and pressure.
    With this asset and its associated channels we will create a run which describes the window of time
    to associate with data that gets ingested.

    Streaming data using this API at a high level involves the following steps:

    1. Create channel configs
    2. Create flow configs
    3. Create an ingestion config
    4. Create a run
    5. Ingest data

    Once this script runs through completion the data should be ready to visualize on the Sift UI not long after.

    If in running this example you created a lot of unwanted runs that you wish to delete, the sift_ingestion_utils
    module has a delete_run method that you can use from the Python console.
    """

    load_dotenv()
    api_key = os.getenv("SIFT_API_KEY", "")
    base_uri = os.getenv("BASE_URI", "")

    run_start_time = Timestamp()
    run_start_time.GetCurrentTime()

    with ingest.use_secure_channel(api_key, base_uri) as channel:
        channel_configs = []

        for channel_config in ExampleTestRunConfig.CHANNEL_CONFIG_PARAMS: 
            name, component, desc, unit = channel_config
            conf = ingest.create_double_type_channel_config(name, component, desc, unit)
            channel_configs.append(conf)

        flow_config = ingest.create_flow_config(ExampleTestRunConfig.FLOW_NAME, *channel_configs)

        print("Creating ingestion config... ", end="")
        ingestion_config = ingest.create_ingestion_config(
            channel,
            ExampleTestRunConfig.ASSET_NAME,
            flow_config,
        )
        print(f"ok [ingestion_config_id {ingestion_config.ingestion_config_id}]")

        print(f"Creating {ExampleTestRunConfig.RUN_NAME}... ", end="")
        run = ingest.create_run(
            channel,
            ExampleTestRunConfig.RUN_NAME,
            ExampleTestRunConfig.RUN_DESCRIPTION,
            None,
            run_start_time,
            None,
            *ExampleTestRunConfig.RUN_TAGS,
        )
        print(f"ok [run_id {run.run_id}]")

        print("Beginning ingestion...")
        ingestion_simulator = create_ingestion_simulator(run, ingestion_config, flow_config)
        ingest.ingest_with_config(channel, ingestion_simulator)
        print("Simulation completed.")

        channel.close()

class ExampleTestRunConfig:
    """
    This is a sample of various properties we'll use to constitute an asset, its associated channels,
    and a run. We're going to be telemetering data for our asset, 'example_asset_name', from two channels:
    a channel called 'pressure' and the other, 'velocity'
    """

    ASSET_NAME = "example_asset_name"
    RUN_NAME = "example_run"
    RUN_DESCRIPTION = "This is an example run"
    RUN_TAGS = ["foo", "bar"]
    FLOW_NAME = "example_flow"

    # (name, component, description, units)
    CHANNEL_CONFIG_PARAMS: List[Tuple[str, Optional[str], Optional[str], Optional[str]]] = [
        ("pressure", None, "pressure applied", "mmHg"),
        ("velocity", "mainmotor", None, "m/s"),
    ]

def create_ingestion_simulator(
    run: runpb.Run,
    ingestion_config: ingestconf.IngestionConfig,
    flow_config: ingestconf.FlowConfig,
    num_data_points: int = 100,
) -> Generator[ingestpb.IngestWithConfigDataStreamRequest, Any, None]:
    """
    This function will create generator which we'll use to simulate ingestion.
    """

    current_timestamp = run.start_time 
    total_messages_sent = 0

    for i in range(num_data_points):
        # 5 milliseconds apart
        timestamp = Timestamp()
        timestamp.FromMilliseconds(current_timestamp.ToMilliseconds() + 5)
        current_timestamp = timestamp

        request = ingestpb.IngestWithConfigDataStreamRequest(
            run_id=run.run_id,
            ingestion_config_id=ingestion_config.ingestion_config_id,
            flow=flow_config.name,
            end_stream_on_validation_error=True,
            timestamp=timestamp,
        )

        pressure = ingestpb.IngestWithConfigDataChannelValue(double=math.sin(60 * i))
        request.channel_values.append(pressure)

        velocity = ingestpb.IngestWithConfigDataChannelValue(double=math.sin(40 * i))
        request.channel_values.append(velocity)

        total_messages_sent += 1
        print(f"Sending message [time={timestamp.ToJsonString()}  run={run.run_id} total_messages_sent={total_messages_sent}]")

        yield request

if __name__ == "__main__":
    main()
