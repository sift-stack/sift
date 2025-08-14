import logging
import random
import time
from datetime import datetime, timezone
from pathlib import Path
from queue import Queue
from typing import List, Optional

from sift_py.ingestion.channel import (
    bit_field_value,
    double_value,
    enum_value,
    int32_value,
)

READINGS_FREQUENCY_HZ = 100


class Simulator:
    """
    Generates data for 60 seconds.
    """

    data_queue: Queue
    asset_name: str
    run_id: Optional[str]
    sample_bit_field_values: List[bytes]
    sample_logs: List[str]
    logger: logging.Logger

    def __init__(self, data_queue: Queue, asset_name: str, run_id: Optional[str]):
        self.data_queue = data_queue
        self.asset_name = asset_name
        self.run_id = run_id

        logging.basicConfig(level=logging.DEBUG)
        self.logger = logging.getLogger(__name__)

        sample_bit_field_values = ["00001001", "00100011", "00001101", "11000001"]
        self.sample_bit_field_values = [bytes([int(byte, 2)]) for byte in sample_bit_field_values]

        dir_path = Path(__file__).parent
        sample_logs = dir_path.joinpath("sample_data").joinpath("sample_logs.txt")

        with open(sample_logs, "r") as file:
            self.sample_logs = file.readlines()

    def run(self):
        """
        Generate data.
        """
        if self.run_id is not None:
            self.logger.info(
                f"Beginning simulation for '{self.asset_name}' with run ({self.run_id})"
            )
        else:
            self.logger.info(f"Beginning simulation for '{self.asset_name}'")

        start_time = time.time()
        end_time = start_time + 60

        last_reading_time = start_time
        readings_interval_s = 1 / READINGS_FREQUENCY_HZ

        last_reporting_time = start_time
        reporting_interval_1 = 1

        n = 0
        while time.time() < end_time:
            current_time = time.time()

            # Send date for readings flow
            timestamp = datetime.now(timezone.utc)
            if current_time - last_reading_time >= readings_interval_s:
                n += 1
                self.data_queue.put(
                    {
                        "flow_name": "readings",
                        "timestamp": timestamp,
                        "channel_values": [
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
                                "value": bit_field_value(
                                    random.choice(self.sample_bit_field_values)
                                ),
                            },
                        ],
                    }
                )
                last_reading_time = current_time

            if current_time - last_reporting_time >= reporting_interval_1:
                logging.info(f"{timestamp} Pushed {n} data points for 'readings' flow")
                last_reporting_time = current_time
                n = 0

        # Signal ingest thread we are done.
        self.data_queue.put(None)
        self.logger.info("Completed simulation.")
