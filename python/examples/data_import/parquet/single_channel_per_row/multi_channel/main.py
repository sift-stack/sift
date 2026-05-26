"""Import a Parquet file with multiple channels (one channel per row) into Sift."""

import os

from dotenv import load_dotenv
from sift_client import SiftClient
from sift_client.sift_types.data_import import (
    ParquetMultiChannelConfig,
    ParquetSingleChannelPerRowImportConfig,
    ParquetTimeColumn,
)

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

    # SCPR requires declaring the channel layout explicitly. Here each row
    # identifies its channel via a name column, so set multi_channel with the
    # name and value column paths.
    config = ParquetSingleChannelPerRowImportConfig(
        asset_name=asset_name,
        time_column=ParquetTimeColumn(path="timestamp"),
        multi_channel=ParquetMultiChannelConfig(
            name_path="channel",
            data_path="value",
        ),
    )

    import_job = client.data_import.import_from_path(
        "sample_data.parquet",
        config=config,
    )

    import_job.wait_until_complete()
