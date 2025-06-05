import os
import logging
from datetime import datetime, timezone

from dotenv import load_dotenv
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.ingestion.service import IngestionService
from simulator import Simulator
from telemetry_config import get_new_flow_config, nostromos_lv_426

if __name__ == "__main__":
    """
    Example of telemetering data for the asset of name 'NostromoLV426' with various channels
    and rules. Also shows how to dynamically add new flows to your configuration at run time.
    The simulator will be sending data for various flows at various frequencies.
    """
    logging.basicConfig(level=logging.DEBUG)
    logger = logging.getLogger(__name__)

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
        # Create ingestion service using the telemetry config we loaded in
        ingestion_service = IngestionService(
            channel,
            telemetry_config,
            end_stream_on_error=True,  # End stream if errors occur API-side.
        )

        # Create an optional run as part of this ingestion
        current_ts = datetime.now(timezone.utc)
        run_name = f"[{telemetry_config.asset_name}].{current_ts.timestamp()}"
        ingestion_service.attach_run(channel, run_name, "Run simulation")

        # Dynamically add a new flow. This is done before starting the simulator
        # for this example, but can be done at any time after creating the IngestionService.
        new_flow = get_new_flow_config()
        if new_flow.name in ingestion_service.flow_configs_by_name:
            logger.info("New flow already exists, not adding.")
        else:
            logger.info("New flow does not exists, adding.")
            ingestion_service.try_create_flow(new_flow)

        # Create our simulator
        simulator = Simulator(ingestion_service)

        # Run it
        simulator.run()
