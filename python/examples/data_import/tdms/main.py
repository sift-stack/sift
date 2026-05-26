"""Import a TDMS file into Sift."""

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

    # Auto-detect the config and import the file. The same call works on both
    # TDMS timing layouts: waveform timing (wf_start_time/wf_increment) and
    # explicit time channels. Swap in `sample_data_time_channel.tdms` to try
    # the other layout. Channel names are emitted as <group>.<channel>.
    import_job = client.data_import.import_from_path(
        "sample_data_waveform.tdms",
        asset=asset_name,
    )

    import_job.wait_until_complete()

    # If auto-detect doesn't quite match your file, inspect the config and patch
    # it before importing. Common fixes: skip channels without timing info,
    # drop a channel that shouldn't be imported, or override the time format.
    #
    # from sift_client.sift_types.data_import import TdmsFallbackMethod
    #
    # config = client.data_import.detect_config("sample_data_waveform.tdms")
    # print(config)  # inspect what was auto-detected
    #
    # # Example: skip channels that lack timing information instead of failing
    # config.fallback_method = TdmsFallbackMethod.IGNORE_ERROR
    #
    # # Example: drop a channel from the import
    # config.data = [d for d in config.data if d.name != "Measurements.channel_0"]
    #
    # import_job = client.data_import.import_from_path(
    #     "sample_data_waveform.tdms",
    #     asset=asset_name,
    #     config=config,
    # )
    # import_job.wait_until_complete()
