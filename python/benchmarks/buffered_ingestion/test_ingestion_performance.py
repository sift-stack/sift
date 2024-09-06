import os
import uuid
from datetime import datetime
from pathlib import Path
from time import time
from typing import Dict, List, Tuple

from dotenv import load_dotenv
from pytest_benchmark.fixture import BenchmarkFixture
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.ingestion.channel import ChannelConfig, ChannelDataType, double_value
from sift_py.ingestion.config.telemetry import TelemetryConfig
from sift_py.ingestion.flow import FlowConfig, FlowOrderedChannelValues
from sift_py.ingestion.service import IngestionService

SAMPLE_DATA_DIR = "sample_data"


def test_buffered_ingestion_performance(benchmark: BenchmarkFixture):
    def target(svc: IngestionService, flows: List[FlowOrderedChannelValues], buffer_size: int):
        assert len(flows) == 100_000, "expected 100_000 flows"

        with svc.buffered_ingestion(buffer_size=buffer_size) as buffered_ingestion:
            buffered_ingestion.ingest_flows(*flows)

    benchmark.pedantic(target=target, setup=_setup, rounds=5)


def test_ingestion_performance(benchmark: BenchmarkFixture):
    def target(svc: IngestionService, flows: List[FlowOrderedChannelValues], stride: int):
        assert len(flows) == 100_000, "expected 100_000 flows"

        ingest_batch_times = []

        timestamp = time()
        for i in range(0, len(flows), stride):
            svc.ingest_flows(*flows[i : i + stride])
            ingest_batch_times.append(time() - timestamp)
            timestamp = time()

        average_time = sum(ingest_batch_times) / len(ingest_batch_times)

        print(f"unbuffered_ingestion | num_flows={len(flows)} batch_size={stride} avg_time_per_batch={average_time}s")

    benchmark.pedantic(target=target, setup=_setup, rounds=5)


def _setup() -> Tuple[Tuple[IngestionService, List[FlowOrderedChannelValues], int], Dict]:
    load_dotenv()

    sift_uri = os.getenv("SIFT_API_URI")
    assert sift_uri, "expected 'SIFT_API_URI' environment variable to be set"

    apikey = os.getenv("SIFT_API_KEY")
    assert apikey, "expected 'SIFT_API_KEY' environment variable to be set"

    sample_data_csv = os.getenv("SAMPLE_DATA_CSV")
    assert sample_data_csv, "expected 'SAMPLE_DATA_CSV' environment variable to be set"

    buffer_size = os.getenv("BUFFER_SIZE")
    assert buffer_size, "expected 'BUFFER_SIZE' environment variable to be set"

    asset_name = os.getenv("ASSET_NAME")
    assert asset_name, "expected 'ASSET_NAME' environment variable to be set"

    ingestion_client_key = os.getenv("INGESTION_CLIENT_KEY")
    assert ingestion_client_key, "expected 'INGESTION_CLIENT_KEY' environment variable to be set"

    sample_data = Path().joinpath(SAMPLE_DATA_DIR).joinpath(sample_data_csv)
    telemetry_config = _load_test_telemetry_config(asset_name, ingestion_client_key)
    flows = _parse_csv(sample_data, telemetry_config)

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
    ingestion_service.attach_run(sift_channel, run_name, "performance testing")

    return (ingestion_service, flows, int(buffer_size)), {}


def _parse_csv(
    path_to_csv: Path, telemetry_config: TelemetryConfig
) -> List[FlowOrderedChannelValues]:
    flows: List[FlowOrderedChannelValues] = []

    assert len(telemetry_config.flows) == 1, "expected only 1 flow for telemetry config"
    flow = telemetry_config.flows[0]
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


def _load_test_telemetry_config(asset_name: str, ingestion_client_key: str) -> TelemetryConfig:
    channels = []

    for i in range(1, 51):
        channels.append(
            ChannelConfig(
                name=f"col{i}",
                data_type=ChannelDataType.DOUBLE,
            )
        )

    return TelemetryConfig(
        asset_name=asset_name,
        ingestion_client_key=ingestion_client_key,
        flows=[FlowConfig(name="data", channels=channels)],
    )
