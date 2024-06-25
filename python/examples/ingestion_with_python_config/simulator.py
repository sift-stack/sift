import logging
import random
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import List

from sift_py.ingestion.channel import (
    bit_field_value,
    double_value,
    enum_value,
    int32_value,
    string_value,
)
from sift_py.ingestion.service import IngestionService

READINGS_FREQUENCY_HZ = 1.5
LOGS_FREQUENCY_HZ = 2
PARTIAL_READINGS_WITH_LOG_FREQUENCY_HZ = 0.5


class Simulator:
    """
    Telemeters sample data for 60 seconds for various combinations of flows
    at various frequencies.
    """

    sample_bit_field_values: List[bytes]
    sample_logs: List[str]
    ingestion_service: IngestionService
    logger: logging.Logger

    def __init__(self, ingestion_service: IngestionService):
        self.ingestion_service = ingestion_service

        logging.basicConfig(level=logging.DEBUG)
        self.logger = logging.getLogger(__name__)

        sample_bit_field_values = ["00001001", "00100011", "00001101", "11000001"]
        self.sample_bit_field_values = [bytes([int(byte, 2)]) for byte in sample_bit_field_values]

        sample_logs = Path().joinpath("sample_data").joinpath("sample_logs.txt")

        with open(sample_logs, "r") as file:
            self.sample_logs = file.readlines()

    def run(self):
        """
        Send data for different combination of flows at different frequencies.
        """

        asset_name = self.ingestion_service.asset_name
        run_id = self.ingestion_service.run_id

        if run_id is not None:
            self.logger.info(f"Beginning simulation for '{asset_name}' with run ({run_id})")
        else:
            self.logger.info(f"Beginning simulation for '{asset_name}'")

        start_time = time.time()
        end_time = start_time + 60

        last_reading_time = start_time
        last_log_time = start_time
        last_partial_readings_time = start_time

        readings_interval_s = 1 / READINGS_FREQUENCY_HZ
        logs_interval_s = 1 / LOGS_FREQUENCY_HZ
        partial_readings_with_log_interval_s = 1 / PARTIAL_READINGS_WITH_LOG_FREQUENCY_HZ

        while time.time() < end_time:
            current_time = time.time()

            if current_time - last_reading_time >= readings_interval_s:
                self.send_reading()
                last_reading_time = current_time

            if current_time - last_log_time >= logs_interval_s:
                self.send_log()
                last_log_time = current_time

            if current_time - last_partial_readings_time >= partial_readings_with_log_interval_s:
                self.send_partial_reading_with_log()
                last_partial_readings_time = current_time

        self.logger.info("Completed simulation.")

    def send_reading(self):
        """
        Sends data for 'readings' flow.
        """

        timestamp = datetime.now(timezone.utc)

        request = self.ingestion_service.try_create_ingestion_request(
            flow_name="readings",
            timestamp=timestamp,
            channel_values=[
                {
                    "channel_name": "velocity",
                    "component": "mainmotor",
                    "value": double_value(random.randint(1, 10)),
                },
                {
                    "channel_name": "voltage",
                    "value": int32_value(random.randint(1, 10)),
                },
                {
                    "channel_name": "vehicle_state",
                    "value": enum_value(random.randint(0, 2)),
                },
                {
                    "channel_name": "gpio",
                    "value": bit_field_value(random.choice(self.sample_bit_field_values)),
                },
            ],
        )
        self.ingestion_service.ingest(request)

        logging.info(f"{timestamp} Emitted data for 'readings' flow")

    def send_log(self):
        """
        Sends data for 'log' flow
        """

        timestamp = datetime.now(timezone.utc)

        request = self.ingestion_service.try_create_ingestion_request(
            flow_name="logs",
            timestamp=timestamp,
            channel_values=[
                {
                    "channel_name": "log",
                    "value": string_value(random.choice(self.sample_logs).strip()),
                },
            ],
        )
        self.ingestion_service.ingest(request)

        logging.info(f"{timestamp} Emitted data for 'logs' flow")

    def send_partial_reading_with_log(self):
        """
        Sends partial data for 'readings' flow and sends data for 'log' flow.
        """

        timestamp = datetime.now(timezone.utc)

        partial_reading = self.ingestion_service.try_create_ingestion_request(
            flow_name="readings",
            timestamp=timestamp,
            channel_values=[
                {
                    "channel_name": "velocity",
                    "component": "mainmotor",
                    "value": double_value(random.randint(1, 10)),
                },
                {
                    "channel_name": "voltage",
                    "value": int32_value(random.randint(1, 10)),
                },
            ],
        )

        log = self.ingestion_service.try_create_ingestion_request(
            flow_name="logs",
            timestamp=timestamp,
            channel_values=[
                {
                    "channel_name": "log",
                    "value": string_value(random.choice(self.sample_logs).strip()),
                },
            ],
        )

        self.ingestion_service.ingest(partial_reading, log)

        logging.info(
            f"{timestamp} Emitted log for 'logs' flow and partial data for 'readings' flow"
        )
