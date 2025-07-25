import os

import h5py
from dotenv import load_dotenv
from sift_py.data_import.config import Hdf5Config
from sift_py.data_import.hdf5 import Hdf5UploadService
from sift_py.rest import SiftRestConfig

if __name__ == "__main__":
    """
    Example of uploading an hdf5 into Sift.
    """

    load_dotenv()

    sift_uri = os.getenv("SIFT_API_URI")
    assert sift_uri, "expected 'SIFT_API_URI' environment variable to be set"

    apikey = os.getenv("SIFT_API_KEY")
    assert apikey, "expected 'SIFT_API_KEY' environment variable to be set"

    asset_name = os.getenv("ASSET_NAME")
    assert asset_name, "expected 'ASSET_NAME' environment variable to be set"

    # Create an HDF5 configuration file to define the data to be ingested
    hdf5_config_dict = {
        "asset_name": asset_name,
        "time": {
            "format": "TIME_FORMAT_ABSOLUTE_DATETIME",
        },
        "data": [],
    }

    # For this example, each HDF5 dataset uses the common '/timestamp' dataset
    # Each is of type double and contains its channel name in the 'Name' attribute
    with h5py.File("sample_data.h5", "r") as f:
        for dset in f.values():
            # Skip adding the timestamp dataset
            if dset.name == "/timestamp":
                continue

            hdf5_config_dict["data"].append(
                {
                    "name": dset.attrs["Name"],
                    "time_dataset": "/timestamp",
                    "value_dataset": dset.name,
                    "data_type": "CHANNEL_DATA_TYPE_DOUBLE",
                }
            )

    hdf5_config = Hdf5Config(hdf5_config_dict)

    rest_config: SiftRestConfig = {
        "uri": sift_uri,
        "apikey": apikey,
    }

    hdf5_upload_service = Hdf5UploadService(rest_config)
    import_service = hdf5_upload_service.upload(
        "sample_data.h5",
        hdf5_config,
    )

    # Wait until the data import is completed.
    # The hdf5 upload service may split the upload into multiple parts
    data_imports = import_service.wait_until_all_complete()

    # Print the data import details and final status.
    for data_import in data_imports:
        print(data_import.model_dump_json(indent=1))

    print("Upload example complete!")
