"""Import an HDF5 file with the two-dimensional schema into Sift.

In this layout, each dataset has shape ``[N, 2]`` where column 0 is time and
column 1 is the channel value. Channel name, units, and description are read
from the dataset's attributes when present.
"""

import os

from dotenv import load_dotenv
from sift_client import SiftClient
from sift_client.sift_types.data_import import DataTypeKey

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

    # Auto-detect the config and import the file. HDF5 requires the layout
    # (data_type) to be specified since the extension alone is ambiguous.
    # HDF5 timestamps aren't self-describing, so time_format defaults to
    # TimeFormat.ABSOLUTE_UNIX_NANOSECONDS; override it if your timestamps
    # are in a different format.
    import_job = client.data_import.import_from_path(
        "sample_data.h5",
        asset=asset_name,
        data_type=DataTypeKey.HDF5_TWO_D,
    )

    import_job.wait_until_complete()

    # If auto-detect doesn't quite match your file, inspect the config and patch
    # it before importing. Common fixes: change the time format, override a
    # channel's data type, or drop a channel that shouldn't be imported.
    #
    # from sift_client.sift_types.data_import import TimeFormat
    #
    # config = client.data_import.detect_config(
    #     "sample_data.h5",
    #     data_type=DataTypeKey.HDF5_TWO_D,
    # )
    # print(config)  # inspect what was auto-detected
    #
    # # Example: timestamps are unix seconds instead of unix nanoseconds
    # config.time_format = TimeFormat.ABSOLUTE_UNIX_SECONDS
    #
    # # Example: drop a channel from the import
    # config.data = [d for d in config.data if d.name != "sensor_0"]
    #
    # import_job = client.data_import.import_from_path(
    #     "sample_data.h5",
    #     asset=asset_name,
    #     config=config,
    # )
    # import_job.wait_until_complete()
