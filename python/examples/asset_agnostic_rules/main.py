import os
from datetime import datetime, timezone
from pathlib import Path

from dotenv import load_dotenv
from python.examples.asset_agnostic_rules.simulator import Simulator
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.ingestion.service import IngestionService, TelemetryConfig
from sift_py.rule.config import ExpressionChannelReference
from sift_py.rule.service import RuleService, SubExpression

TELEMETRY_CONFIGS_DIR = Path().joinpath("telemetry_configs")
RULE_MODULES_DIR = Path().joinpath("rule_modules")


if __name__ == "__main__":
    load_dotenv()

    apikey = os.getenv("SIFT_API_KEY")
    assert apikey, "Missing 'SIFT_API_KEY' environment variable."

    base_uri = os.getenv("BASE_URI")
    assert base_uri, "Missing 'BASE_URI' environment variable."

    telemetry_config_name = os.getenv("TELEMETRY_CONFIG")
    assert telemetry_config_name, "Missing 'TELEMETRY_CONFIG' environment variable."

    # Load your telemetry config
    telemetry_config_path = TELEMETRY_CONFIGS_DIR.joinpath(telemetry_config_name)
    telemetry_config = TelemetryConfig.try_from_yaml(telemetry_config_path)

    # Paths to your rules
    voltage_rules_src = RULE_MODULES_DIR.joinpath("voltage.yml")
    velocity_rules_src = RULE_MODULES_DIR.joinpath("velocity.yml")

    # Create a gRPC transport channel configured specifically for the Sift API
    sift_channel_config = SiftChannelConfig(uri=base_uri, apikey=apikey)

    with use_sift_channel(sift_channel_config) as channel:
        # Create ingestion service using the telemetry config we loaded in.
        # This step must come before loading rules.
        ingestion_service = IngestionService(
            channel,
            telemetry_config,
            # End stream if errors occur API-side. Good for debugging but remove
            # in production for increased ingestion performance.
            end_stream_on_error=True,
        )

        # Create/update and configure your rules
        rule_service = RuleService(channel)
        rule_configs = rule_service.load_rules_from_yaml(
            paths=[
                voltage_rules_src,
                velocity_rules_src,
            ],
            sub_expressions=[
                SubExpression("voltage.overvoltage", {"$1": 75}),
                SubExpression("voltage.undervoltage", {"$1": 30}),
                SubExpression("velocity.vehicle_not_stopped", {"$2": 10}),
            ],
            channel_references_map={
                "overvoltage": [
                    ExpressionChannelReference(
                        channel_reference="$2", channel_identifier="vehicle_state"
                    ),
                ],
                "undervoltage": [
                    ExpressionChannelReference(
                        channel_reference="$2", channel_identifier="vehicle_state"
                    ),
                ],
                "vehicle_stuck": [
                    ExpressionChannelReference(
                        channel_reference="$1", channel_identifier="vehicle_state"
                    ),
                    ExpressionChannelReference(
                        channel_reference="$2", channel_identifier="mainmotor.velocity"
                    ),
                ],
                "vehicle_not_stopped": [
                    ExpressionChannelReference(
                        channel_reference="$1", channel_identifier="vehicle_state"
                    ),
                ],
            },
        )

        # Create an optional run as part of this ingestion
        current_ts = datetime.now(timezone.utc)
        run_name = f"[{telemetry_config.asset_name}].{current_ts.timestamp()}"
        ingestion_service.attach_run(channel, run_name, "Run simulation")

        # Create our simulator
        simulator = Simulator(ingestion_service)

        # Run it
        simulator.run()
