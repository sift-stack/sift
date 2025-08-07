import os
import threading
from datetime import datetime, timezone
from queue import Empty, Queue

from dotenv import load_dotenv
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.ingestion.service import IngestionService
from simulator import Simulator
from telemetry_config import nostromos_lv_426


def ingestion_thread(data_queue: Queue):
    """
    This thread is responsible for consuming data from the queue and sending
    it to Sift.
    """
    # Can tune ingestion performance with buffer_size and flush_interval_sec
    with ingestion_service.buffered_ingestion() as buffered_ingestion:
        while True:
            try:
                item = data_queue.get(timeout=1)
                # None signals the Simulator thread is done.
                if item is None:
                    return
            except Empty:
                continue
            buffered_ingestion.try_ingest_flows(item)


if __name__ == "__main__":
    """
    Threaded example of telemetering data for the asset of name 'NostromoLV426' with various channels
    and rules. The simulator will be generating data for various flows at various frequencies.
    The ingestion_thread will ingest this data into Sift.
    """

    load_dotenv()

    apikey = os.getenv("SIFT_API_KEY")

    if apikey is None:
        raise Exception("Missing 'SIFT_API_KEY' environment variable.")

    base_uri = os.getenv("BASE_URI")
    if not base_uri.startswith("http"):
        base_uri = f"http://{base_uri}"

    if base_uri is None:
        raise Exception("Missing 'BASE_URI' environment variable.")

    # Load your telemetry config
    telemetry_config = nostromos_lv_426()

    # Create a gRPC transport channel configured specifically for the Sift API
    sift_channel_config = SiftChannelConfig(uri=base_uri, apikey=apikey)
    sift_channel_config["use_ssl"] = False

    with use_sift_channel(sift_channel_config) as channel:
        # Create ingestion service using the telemetry config we loaded in
        ingestion_service = IngestionService(
            channel,
            telemetry_config,
            end_stream_on_error=True,  # End stream if errors occur API-side.
        )

        # Create an optional run as part of this ingestion
        current_ts = datetime.now(timezone.utc)
        run_name = f"[{telemetry_config.asset_name}].{current_ts.timestamp()} (Threaded)"
        ingestion_service.attach_run(channel, run_name, "Run simulation")

        data_queue = Queue()
        thread = threading.Thread(target=ingestion_thread, args=(data_queue,))
        thread.start()

        # Create our simulator
        simulator = Simulator(data_queue, ingestion_service.asset_name, ingestion_service.run_id)

        # Run it
        simulator.run()
        thread.join()
