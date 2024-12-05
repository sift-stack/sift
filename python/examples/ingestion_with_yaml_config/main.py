import os
from pathlib import Path

from dotenv import load_dotenv
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.ingestion.service import IngestionService
from sift_py.rule.service import RuleService
from sift_py.yaml.rule import load_sub_expressions
from telemetry_config import nostromos_lv_426

EXPRESSION_MODULES_DIR = Path().joinpath("expression_modules")
RULE_MODULES_DIR = Path().joinpath("rule_modules")


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
            end_stream_on_error=True,  # End stream if errors occur API-side.
        )

        sub_expressions = load_sub_expressions(
            rule_module_paths=[
                RULE_MODULES_DIR.joinpath("voltage.yml"),
                RULE_MODULES_DIR.joinpath("velocity.yml"),
                RULE_MODULES_DIR.joinpath("nostromo.yml"),
            ],
            named_module_paths=[
                EXPRESSION_MODULES_DIR.joinpath("kinematics.yml"),
                EXPRESSION_MODULES_DIR.joinpath("string.yml"),
            ],
        )
        print([sub_expression.__dict__ for sub_expression in sub_expressions])
        rule_service = RuleService(channel)
        rule_configs = rule_service.load_rules_from_yaml(
            paths=[
                RULE_MODULES_DIR.joinpath("voltage.yml"),
                RULE_MODULES_DIR.joinpath("velocity.yml"),
                RULE_MODULES_DIR.joinpath("nostromo.yml"),
            ],
            sub_expressions=sub_expressions,
        )

        # Create an optional run as part of this ingestion
#        current_ts = datetime.now(timezone.utc)
#        run_name = f"[{telemetry_config.asset_name}].{current_ts.timestamp()}"
#        ingestion_service.attach_run(channel, run_name, "Run simulation")
#
#        # Create our simulator
#        simulator = Simulator(ingestion_service)
#
#        # Run it
#        simulator.run()
