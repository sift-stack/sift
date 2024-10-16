import os

from dotenv import load_dotenv
from sift_py.data_import.tdms import TdmsUploadService
from sift_py.rest import SiftRestConfig

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
        "use_ssl": False,
    }

    tdms_upload_service = TdmsUploadService(rest_config)
    status = tdms_upload_service.upload("sample_data.tdms", asset_name, group_into_components=True)
    status.wait_until_complete()
    print("Upload example complete!")
