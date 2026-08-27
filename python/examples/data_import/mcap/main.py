"""Import an MCAP (.mcap) file into Sift.

MCAP files are self-describing, so the import needs no column mapping: every
channel of every supported topic (ros2msg schemas with cdr messages) is
imported. Channel names combine the topic and flattened field path; the
bundled sample_data.mcap produces "/imu/data.angular_velocity.x",
"/imu/data.linear_acceleration.x", "/battery.voltage", and their siblings.

Swap sample_data.mcap for your own .mcap recording.
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

    client = SiftClient(api_key=apikey, grpc_url=grpc_uri, rest_url=rest_uri)

    # Auto-detect the config and import the file.
    import_job = client.data_import.import_from_path(
        "sample_data.mcap",
        asset=asset_name,
    )

    import_job.wait_until_complete()

    # If auto-detect doesn't quite match your file, inspect the config and patch
    # it before importing. Common fixes: drop channels you don't need, rename or
    # retype a channel, skip undecodable topics, set a start time for logs on a
    # non-Unix epoch, or pick metadata records to import.
    #
    # from datetime import datetime, timezone
    #
    # from sift_client.sift_types.data_import import (
    #     McapComplexTypesImportMode,
    #     McapParseErrorPolicy,
    # )
    #
    # config = client.data_import.detect_config("sample_data.mcap")
    # print(config)  # inspect every detected channel
    #
    # # Example: import array fields only as JSON strings, instead of the
    # # default of both JSON and Arrow IPC bytes
    # config.complex_types_import_mode = McapComplexTypesImportMode.STRING
    #
    # # Example: import only the IMU topic
    # config.data = [d for d in config.data if d.topic == "/imu/data"]
    #
    # # Example: skip undecodable topics and records instead of failing
    # config.parse_error_policy = McapParseErrorPolicy.IGNORE_ERROR
    #
    # # Example: reinterpret log_time as elapsed nanoseconds from an explicit
    # # start; only for recorders whose clock did not track Unix time
    # config.relative_start_time = datetime(2026, 1, 1, tzinfo=timezone.utc)
    #
    # # Example: import every key of a named metadata record as run metadata
    # config.metadata_records = ["calibration"]
    #
    # import_job = client.data_import.import_from_path(
    #     "sample_data.mcap",
    #     asset=asset_name,
    #     config=config,
    # )
    # import_job.wait_until_complete()
