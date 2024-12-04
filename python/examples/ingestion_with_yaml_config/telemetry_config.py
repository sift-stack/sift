import os
from pathlib import Path

from sift_py.ingestion.service import TelemetryConfig

TELEMETRY_CONFIGS_DIR = Path().joinpath("telemetry_configs")

def nostromos_lv_426() -> TelemetryConfig:
    telemetry_config_name = os.getenv("TELEMETRY_CONFIG")

    if telemetry_config_name is None:
        raise Exception("Missing 'TELEMETRY_CONFIG' environment variable.")

    telemetry_config_path = TELEMETRY_CONFIGS_DIR.joinpath(telemetry_config_name)

    # Load your telemetry config with your reusable expressions modules and rule modules
    return TelemetryConfig.try_from_yaml(
        telemetry_config_path,
    )
