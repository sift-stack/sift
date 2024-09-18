import csv
import os
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Tuple

from dotenv import load_dotenv
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.ingestion.channel import ChannelConfig, ChannelDataType, double_value, empty_value
from sift_py.ingestion.config.telemetry import TelemetryConfig
from sift_py.ingestion.flow import FlowConfig, FlowOrderedChannelValues
from sift_py.ingestion.service import IngestionService


def parse_csv(
    data: List[Dict], telemetry_config: TelemetryConfig
) -> List[FlowOrderedChannelValues]:
    flows: List[FlowOrderedChannelValues] = []

    flow = telemetry_config.flows[0]  # Packed into a single flow for this example
    flow_name = flow.name

    all_timestamps: List = []
    for channel in data:
        all_timestamps += channel.keys()

    for timestamp in sorted(list(set(all_timestamps))):
        channel_values = []
        for channel in data:
            channel_data = channel.get(timestamp)
            if channel_data:
                channel_values.append(double_value(float(channel_data)))
            else:
                channel_values.append(empty_value())

        flows.append(
            {
                "flow_name": flow_name,
                "timestamp": timestamp,
                "channel_values": channel_values,
            }
        )

    return flows


def load_telemetry_config(
    csv_paths: List[Path], asset_name: str, ingestion_client_key: str
) -> Tuple[TelemetryConfig, List[Dict]]:
    channels = []
    data: List[Dict] = []

    for path_to_csv in csv_paths:
        with open(path_to_csv, "r") as csv_file:
            reader = csv.reader(csv_file)
            header = next(reader)  # Grab header
            channel_name = header[1]  # Assuming only one channel per CSV

            channels.append(
                ChannelConfig(
                    name=channel_name,
                    data_type=ChannelDataType.DOUBLE,  # Assuming all channels are doubles for this example
                )
            )

            channel_data = {}
            for row in reader:
                 timestamp_str, value = row[0], row[1]  # Assuming only one channel per CSV
                 channel_data.update({
                      datetime.fromisoformat(timestamp_str):
                      value
                 })
            data.append(channel_data)

    telemetry_config = TelemetryConfig(
        asset_name=asset_name,
        ingestion_client_key=ingestion_client_key,
        flows=[FlowConfig(name="data", channels=channels)],
    )
    return telemetry_config, data


if __name__ == "__main__":
    """
    Example of ingesting data from a CSV file into Sift.
    """

    load_dotenv()

    sift_uri = os.getenv("SIFT_API_URI")
    assert sift_uri, "expected 'SIFT_API_URI' environment variable to be set"

    apikey = os.getenv("SIFT_API_KEY")
    assert apikey, "expected 'SIFT_API_KEY' environment variable to be set"

    asset_name = os.getenv("ASSET_NAME")
    assert asset_name, "expected 'ASSET_NAME' environment variable to be set"

    ingestion_client_key = os.getenv("INGESTION_CLIENT_KEY")
    assert ingestion_client_key, "expected 'INGESTION_CLIENT_KEY' environment variable to be set"

    csv_data = [Path("channel_a.csv"), Path("channel_b.csv"), Path("channel_c.csv")]

    telemetry_config, data = load_telemetry_config(csv_data, asset_name, ingestion_client_key)
    flows = parse_csv(data, telemetry_config)

    sift_channel_config = SiftChannelConfig(
        uri=sift_uri,
        apikey=apikey,
    )

    with use_sift_channel(sift_channel_config) as channel:
        # Create ingestion service using the telemetry config
        ingestion_service = IngestionService(
            channel=channel,
            config=telemetry_config,
        )

        # Create a new run as part of this ingestion
        run_name = f"{asset_name}-{datetime.now()}"
        ingestion_service.attach_run(channel, run_name, "example csv ingestion")

        with ingestion_service.buffered_ingestion() as buffered_ingestion:
            buffered_ingestion.ingest_flows(*flows)
