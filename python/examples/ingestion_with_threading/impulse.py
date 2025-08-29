import logging
import os
import threading
import time
import traceback
from datetime import datetime, timezone
from typing import List

from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.ingestion.channel import ChannelConfig, ChannelDataType, double_value
from sift_py.ingestion.config.telemetry import FlowConfig, TelemetryConfig
from sift_py.ingestion.service import IngestionService

# -------------------------------
# Config
# -------------------------------
LOG_LEVEL = logging.INFO
LOG_FORMAT = "%(asctime)s - %(levelname)s - %(message)s"

FLOW_NAME = "temperature_reading_multiprocessing"
CHANNEL_NAME = "temperature_multiprocessing"
ASSET_NAME = "NostromoLV426"
INGESTION_CLIENT_KEY = "nostromo_lv_426_test"

SIFT_BASE_URI = "https://grpc-api.development.siftstack.com"
SIFT_API_KEY = os.getenv("SIFT_DEV_API_KEY")

PROCESS_COUNT = 8
BUFFER_SIZE = 1000
FLUSH_INTERVAL_SEC = 1
INGEST_SLEEP_SEC = 1
FLOW_CREATE_BATCH = 100

logging.basicConfig(level=LOG_LEVEL, format=LOG_FORMAT)
log = logging.getLogger(__name__)


class Consumer:
    def __init__(self, ingestion_service: IngestionService):
        self.ingestion_service = ingestion_service

    def run(self, consumer_id: int) -> None:
        log.info("Consumer %d started", consumer_id)
        with self.ingestion_service.buffered_ingestion(
            buffer_size=BUFFER_SIZE, flush_interval_sec=FLUSH_INTERVAL_SEC
        ) as buffered:
            while True:
                try:
                    buffered.ingest_flows(
                        {
                            "flow_name": FLOW_NAME,
                            "timestamp": datetime.now(timezone.utc),
                            "channel_values": [double_value(123)],
                        }
                    )
                    time.sleep(INGEST_SLEEP_SEC)
                except Exception as e:
                    log.error("Error ingesting flows: %s", e)
                    log.error(traceback.format_exc())
                    time.sleep(INGEST_SLEEP_SEC)


def build_telemetry_config() -> TelemetryConfig:
    temperature_channel = ChannelConfig(
        name=CHANNEL_NAME,
        data_type=ChannelDataType.DOUBLE,
        description="temperature of thruster",
        unit="Kelvin",
    )

    return TelemetryConfig(
        asset_name=ASSET_NAME,
        ingestion_client_key=INGESTION_CLIENT_KEY,
        flows=[FlowConfig(name=FLOW_NAME, channels=[temperature_channel])],
    )


def create_missing_flows(
    svc: IngestionService, flows: List[FlowConfig], batch_size: int = FLOW_CREATE_BATCH
) -> None:
    existing = set(svc.flow_configs_by_name.keys())
    to_create = [f for f in flows if f.name not in existing]

    log.info("Existing flows: %d", len(existing))
    log.info("Creating %d flow(s): %s", len(to_create), [f.name for f in to_create])

    for i in range(0, len(to_create), batch_size):
        svc.try_create_flow(*to_create[i : i + batch_size])


def main() -> None:
    log.info("Loading telemetry config...")
    telemetry_config = build_telemetry_config()

    sift_channel_config = SiftChannelConfig(uri=SIFT_BASE_URI, apikey=SIFT_API_KEY)
    sift_channel = use_sift_channel(sift_channel_config)

    log.info("Initializing Sift Ingestion Service...")
    log.info("Asset name: %s", telemetry_config.asset_name)
    log.info("Ingestion client key: %s", telemetry_config.ingestion_client_key)

    ingestion_service = IngestionService(
        sift_channel,
        TelemetryConfig(
            asset_name=telemetry_config.asset_name,
            ingestion_client_key=telemetry_config.ingestion_client_key,
        ),
    )

    log.info("Flow registry: %s", list(ingestion_service.flow_configs_by_name.keys()))
    create_missing_flows(ingestion_service, telemetry_config.flows)

    processes: List[threading.Thread] = []
    try:
        consumer = Consumer(ingestion_service)
        for i in range(PROCESS_COUNT):
            p = threading.Thread(target=consumer.run, args=(i,))
            p.start()
            processes.append(p)

        for p in processes:
            p.join()

    except KeyboardInterrupt:
        log.info("Exiting...")
    finally:
        for p in processes:
            if p.is_alive():
                p.terminate()
        sift_channel.close()


if __name__ == "__main__":
    main()
