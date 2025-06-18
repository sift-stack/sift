import logging
import random
import time
from datetime import datetime, timezone
from queue import Queue
from typing import List

from google.protobuf.timestamp_pb2 import Timestamp
from sift.ingest.v1.ingest_pb2 import IngestWithConfigDataStreamRequest
from sift_py.ingestion.channel import (
    bit_field_value,
    double_value,
    enum_value,
    int32_value,
)
from sift_py.ingestion.service import IngestionService

READINGS_FREQUENCY_HZ = 100


class Simulator:
    """
    Generates data for 60 seconds.
    """

    ingestion_service: IngestionService
    data_queue: Queue
    sample_bit_field_values: List[bytes]
    logger: logging.Logger

    def __init__(self, ingestion_service: IngestionService, data_queue: Queue):
        self.ingestion_service = ingestion_service
        self.data_queue = data_queue

        logging.basicConfig(level=logging.DEBUG)
        self.logger = logging.getLogger(__name__)

        sample_bit_field_values = ["00001001", "00100011", "00001101", "11000001"]
        self.sample_bit_field_values = [bytes([int(byte, 2)]) for byte in sample_bit_field_values]

    def run(self):
        """
        Generate data.
        """
        asset_name = self.ingestion_service.asset_name
        run_id = self.ingestion_service.run_id

        if run_id is not None:
            self.logger.info(f"Beginning simulation for '{asset_name}' with run ({run_id})")
        else:
            self.logger.info(f"Beginning simulation for '{asset_name}'")

        start_time = time.time()
        end_time = start_time + 90

        last_reading_time = start_time
        readings_interval_s = 1 / READINGS_FREQUENCY_HZ

        while time.time() < end_time:
            current_time = time.time()

            if current_time - last_reading_time >= readings_interval_s:
                timestamp_pb = Timestamp()
                timestamp_pb.FromDatetime(datetime.now(timezone.utc))

                data = {
                    "mainmotor.velocity": double_value(random.randint(1, 10)),
                    "voltage": int32_value(random.randint(1, 50)),
                    "vehicle_state": enum_value(random.randint(0, 2)),
                    "gpio": bit_field_value(random.choice(self.sample_bit_field_values)),
                }

                # channel_values must be in the same order as channels in the telemetry config
                channel_values = []
                for channel in self.ingestion_service.flow_configs_by_name["readings"].channels:
                    channel_values.append(data[channel.name])

                self.data_queue.put(
                    IngestWithConfigDataStreamRequest(
                        ingestion_config_id=self.ingestion_service.ingestion_config.ingestion_config_id,
                        flow="readings",
                        timestamp=timestamp_pb,
                        run_id=self.ingestion_service.run_id or "",
                        channel_values=channel_values,
                        end_stream_on_validation_error=self.ingestion_service.end_stream_on_error,
                    )
                )

                last_reading_time = current_time

        # Signal ingest thread we are done.
        self.data_queue.put(None)
        self.logger.info("Completed simulation.")
