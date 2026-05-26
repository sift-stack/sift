"""Import a CSV file into Sift."""

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
        "sample_data.csv",
        asset=asset_name,
    )

    import_job.wait_until_complete()

    # If auto-detect doesn't quite match your file, inspect the config and patch
    # it before importing. Common fixes: override a column's data type, change
    # the time column or format, or drop a column that shouldn't be imported.
    #
    # config = client.data_import.detect_config("sample_data.csv")
    # print(config)  # inspect what was auto-detected
    #
    # # Example: drop a column from the import
    # config.data_columns = [dc for dc in config.data_columns if dc.name != "channel_0"]
    #
    # import_job = client.data_import.import_from_path(
    #     "sample_data.csv",
    #     asset=asset_name,
    #     config=config,
    # )
    # import_job.wait_until_complete()
