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


def _parse_csv(
    path_to_csv: Path, telemetry_config: TelemetryConfig
) -> List[FlowOrderedChannelValues]:
    flows: List[FlowOrderedChannelValues] = []

    flow = telemetry_config.flows[0]  # Packed into a single flow for this example
    flow_name = flow.name

    with open(path_to_csv, "r") as csv:
        body = iter(csv.readlines())
        next(body)  # skip header

        for row in body:
            columns = row.strip().split(",")
            timestamp_str, values = columns[0], columns[1:]
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


def _load_telemetry_config(
    path_to_csv: Path, asset_name: str, ingestion_client_key: str
) -> TelemetryConfig:
    channels = []

    with open(path_to_csv, "r") as csv:
        body = csv.readlines()
        header_row = body[0]
        header = header_row.strip().split(",")
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

    telemetry_config = _load_telemetry_config(sample_data_csv, asset_name, ingestion_client_key)
    flows = _parse_csv(sample_data_csv, telemetry_config)

    config = SiftChannelConfig(
        uri=sift_uri,
        apikey=apikey,
        use_ssl=False,
    )

    sift_channel = use_sift_channel(config)

    ingestion_service = IngestionService(
        channel=sift_channel,
        config=telemetry_config,
    )

    run_name = f"{telemetry_config.ingestion_client_key}-{uuid.uuid4()}"
    ingestion_service.attach_run(sift_channel, run_name, "example csv ingestion")
