import logging
import os
import threading
import time
from datetime import datetime, timezone
from queue import Queue
from typing import Iterable

import grpc
from dotenv import load_dotenv
from sift.ingest.v1.ingest_pb2 import IngestWithConfigDataStreamRequest
from sift.ingest.v1.ingest_pb2_grpc import IngestServiceStub
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.ingestion.service import IngestionService
from simulator import Simulator
from telemetry_config import nostromos_lv_426


def ingestion_thread(sift_channel_config: SiftChannelConfig, data_queue: Queue):
    """
    This thread will use a generator to stream data to Sift with no buffering/little latency.
    """
    logger = logging.getLogger(__name__)
    stop = threading.Event()
    backup_queue = []

    def data_generator() -> Iterable[IngestWithConfigDataStreamRequest]:
        # Yield data for 60s
        connection_duration = 60
        start_time = time.time()
        while time.time() - start_time < connection_duration:
            item = data_queue.get()
            # None signals the Simulator thread is done.
            if item is None:
                stop.set()
                logger.info("Simulation thread completed.")
                return
            # Store each item in a backup queue in case we lose the connection.
            backup_queue.append(item)
            yield item

    with use_sift_channel(sift_channel_config) as channel:
        # Use the raw grpc Stub to stream data with without buffering.
        ingestion_service = IngestServiceStub(channel)

        while True:
            try:
                logger.info("Opening connection and streaming for 60s")
                ingestion_service.IngestWithConfigDataStream(data_generator())
                logger.info("Connection closed. Data streamed successfully")
                backup_queue.clear()
            except grpc.RpcError as e:
                logger.warning(e)
                #  Add the data back to the queue so that we can stream it again.
                for item in backup_queue:
                    data_queue.put(item)

            if stop.is_set():
                return


if __name__ == "__main__":
    """
    Threaded example of telemetering data for the asset of name 'NostromoLV426' with various channels
    and rules with no buffering and little latency. The simulator will be generating data.
    The ingestion_thread will ingest this data into Sift.
    """

    load_dotenv()

    apikey = os.getenv("SIFT_API_KEY")

    if apikey is None:
        raise Exception("Missing 'SIFT_API_KEY' environment variable.")

    base_uri = os.getenv("BASE_URI")

    if base_uri is None:
        raise Exception("Missing 'BASE_URI' environment variable.")

    # Load your telemetry config
    telemetry_config = nostromos_lv_426()

    # Create a gRPC transport channel configured specifically for the Sift API
    sift_channel_config = SiftChannelConfig(uri=base_uri, apikey=apikey)

    with use_sift_channel(sift_channel_config) as channel:
        # Create ingestion service using the telemetry config we loaded in.
        ingestion_service = IngestionService(
            channel,
            telemetry_config,
        )

        # Create an optional run as part of this ingestion
        current_ts = datetime.now(timezone.utc)
        run_name = f"[{telemetry_config.asset_name}].{current_ts.timestamp()}"
        ingestion_service.attach_run(channel, run_name, "Run simulation")

    # Start the ingestion thread
    data_queue = Queue()
    thread = threading.Thread(
        target=ingestion_thread,
        args=(
            sift_channel_config,
            data_queue,
        ),
    )
    thread.start()

    # Create our simulator
    simulator = Simulator(ingestion_service, data_queue)

    # Run it
    simulator.run()
    thread.join()
