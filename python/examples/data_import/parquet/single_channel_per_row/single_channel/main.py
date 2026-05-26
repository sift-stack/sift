"""Import a Parquet file containing a single channel into Sift."""

import os

from dotenv import load_dotenv
from sift_client import SiftClient
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import (
    ParquetSingleChannelConfig,
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

    # SCPR requires declaring the channel layout explicitly. Here the whole file
    # is one channel, so set single_channel with its name, data type, and units.
    config = ParquetSingleChannelPerRowImportConfig(
        asset_name=asset_name,
        time_column=ParquetTimeColumn(path="timestamp"),
        single_channel=ParquetSingleChannelConfig(
            data_path="value",
            name="speed",
            data_type=ChannelDataType.DOUBLE,
            units="m/s",
        ),
    )

    import_job = client.data_import.import_from_path(
        "sample_data.parquet",
        config=config,
    )

    import_job.wait_until_complete()
