"""
Example ch10 upload.

This example will not work out of the box. Replace ExampleCh10File with
an implementation of the class that can parse your ch10 files.
"""

import os

from dotenv import load_dotenv
from sift_py.data_import.ch10 import BaseCh10File, Ch10UploadService
from sift_py.data_import.status import DataImportService
from sift_py.rest import SiftRestConfig


class ExampleCh10File(BaseCh10File):
    pass


if __name__ == "__main__":
    """
    Example of uploading a TDMS file into Sift.
    """
    load_dotenv()

    sift_uri = os.getenv("SIFT_API_URI")
    assert sift_uri, "expected 'SIFT_API_URI' environment variable to be set"

    apikey = os.getenv("SIFT_API_KEY")
    assert apikey, "expected 'SIFT_API_KEY' environment variable to be set"

    asset_name = os.getenv("ASSET_NAME")
    assert asset_name, "expected 'ASSET_NAME' environment variable to be set"

    rest_config: SiftRestConfig = {
        "uri": sift_uri,
        "apikey": apikey,
    }

    ch10_file = ExampleCh10File("sample_data.ch10")
    ch10_upload_service = Ch10UploadService(rest_config)

    import_service: DataImportService = ch10_upload_service.upload(
        ch10_file,
        asset_name,
    )
    print(import_service.get_data_import())

    print("Waiting for upload to complete...")
    import_service.wait_until_complete()
    print("Upload example complete!")
