import csv
import os
import uuid
from datetime import datetime
from pathlib import Path
from typing import List

from dotenv import load_dotenv
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.ingestion.channel import ChannelConfig, ChannelDataType, double_value
from sift_py.ingestion.config.telemetry import TelemetryConfig
from sift_py.ingestion.flow import FlowConfig, FlowOrderedChannelValues
from sift_py.ingestion.service import IngestionService


def parse_csv(
    path_to_csv: Path, telemetry_config: TelemetryConfig
) -> List[FlowOrderedChannelValues]:
    flows: List[FlowOrderedChannelValues] = []

    flow = telemetry_config.flows[0]  # Packed into a single flow for this example
    flow_name = flow.name

    with open(path_to_csv, "r") as csv_file:
        reader = csv.reader(csv_file)
        next(reader)  # skip header

        for row in reader:
            timestamp_str, values = row[0], row[1:]
            assert len(values) == len(
                flow.channels
            ), "number of channels don't match number of data points in row"

            flows.append(
                {
                    "flow_name": flow_name,
                    "timestamp": datetime.fromisoformat(timestamp_str),
                    "channel_values": [double_value(float(raw_value)) for raw_value in values],
                }
            )

    return flows


def load_telemetry_config(
    path_to_csv: Path, asset_name: str, ingestion_client_key: str
) -> TelemetryConfig:
    channels = []

    with open(path_to_csv, "r") as csv_file:
        reader = csv.reader(csv_file)
        header = next(reader)  # grab only header
        channel_names = header[1:]

        for name in channel_names:
            channels.append(
                ChannelConfig(
                    name=name,
                    data_type=ChannelDataType.DOUBLE,  # Assuming all channels are doubles for this example
                )
            )

    return TelemetryConfig(
        asset_name=asset_name,
        ingestion_client_key=ingestion_client_key,
        flows=[FlowConfig(name="data", channels=channels)],
    )


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

    sample_data_csv = Path("sample_data.csv")

    telemetry_config = load_telemetry_config(sample_data_csv, asset_name, ingestion_client_key)
    flows = parse_csv(sample_data_csv, telemetry_config)

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
        run_name = f"{telemetry_config.ingestion_client_key}-{uuid.uuid4()}"
        ingestion_service.attach_run(channel, run_name, "example csv ingestion")
