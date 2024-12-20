import os
from datetime import datetime, timezone

from dotenv import load_dotenv
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.ingestion.service import IngestionService
from simulator import Simulator
from telemetry_config import nostromos_lv_426

if __name__ == "__main__":
    """
    Example of telemetering data for the asset of name 'NostromoLV426' with various channels
    and rules. The simulator will be sending data for various flows at various frequencies.
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
        # Create ingestion service using the telemetry config we loaded in
        ingestion_service = IngestionService(
            channel,
            telemetry_config,
            overwrite_rules=True,  # Overwrite any rules created in the Sift UI that isn't in the config
            end_stream_on_error=True,  # End stream if errors occur API-side.
        )

        # Create an optional run as part of this ingestion
        current_ts = datetime.now(timezone.utc)
        run_name = f"[{telemetry_config.asset_name}].{current_ts.timestamp()}"
        ingestion_service.attach_run(channel, run_name, "Run simulation")

        # Create our simulator
        simulator = Simulator(ingestion_service)

        # Run it
        simulator.run()
