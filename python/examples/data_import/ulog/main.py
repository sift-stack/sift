"""Import a PX4 ULog (.ulg) file into Sift.

ULog files are self-describing, so detection enumerates every channel from the
embedded schema and you supply no column mapping. Channel names follow PX4's
convention of message, multi-instance index, and field, e.g. "sensor_accel_0.x".

Point ULOG_PATH at your own .ulg flight log.
"""

import os

from dotenv import load_dotenv
from sift_client import SiftClient

if __name__ == "__main__":
    load_dotenv()

    grpc_uri = os.getenv("SIFT_GRPC_URI")
    assert grpc_uri, "expected 'SIFT_GRPC_URI' environment variable to be set"

    rest_uri = os.getenv("SIFT_REST_URI")
    assert rest_uri, "expected 'SIFT_REST_URI' environment variable to be set"

    apikey = os.getenv("SIFT_API_KEY")
    assert apikey, "expected 'SIFT_API_KEY' environment variable to be set"

    asset_name = os.getenv("ASSET_NAME")
    assert asset_name, "expected 'ASSET_NAME' environment variable to be set"

    ulog_path = os.getenv("ULOG_PATH", "sample_data.ulg")

    client = SiftClient(api_key=apikey, grpc_url=grpc_uri, rest_url=rest_uri)

    # Auto-detect the config and import the file.
    import_job = client.data_import.import_from_path(
        ulog_path,
        asset=asset_name,
    )

    import_job.wait_until_complete()

    # If auto-detect doesn't quite match your file, inspect the config and patch
    # it before importing. Common fixes: drop channels you don't need, rename or
    # retype a channel, set a start time for logs without a GPS fix, or pick run
    # metadata to import.
    #
    # from datetime import datetime, timezone
    #
    # config = client.data_import.detect_config(ulog_path)
    # print(config)  # inspect every detected channel
    #
    # # Example: import only the accelerometer channels
    # config.data = [d for d in config.data if d.channel.startswith("sensor_accel_0.")]
    #
    # # Example: anchor the timeline when the log has no GPS fix
    # config.relative_start_time = datetime(2026, 1, 1, tzinfo=timezone.utc)
    #
    # # Example: import firmware version and a parameter as run metadata
    # config.info_keys = ["ver_sw"]
    # config.param_keys = ["BAT1_CAPACITY"]
    #
    # import_job = client.data_import.import_from_path(
    #     ulog_path,
    #     asset=asset_name,
    #     config=config,
    # )
    # import_job.wait_until_complete()
