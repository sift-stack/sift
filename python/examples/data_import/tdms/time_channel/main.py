import os

from dotenv import load_dotenv
from sift_py.data_import.tdms import TdmsTimeFormat, TdmsUploadService
from sift_py.rest import SiftRestConfig

if __name__ == "__main__":
    """
    Example of uploading a TDMS file into Sift using the TDMS
    format that contains time information as a separate channel
    within each TDMS group.
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

    tdms_upload_service = TdmsUploadService(rest_config)
    import_service = tdms_upload_service.upload(
        "sample_data.tdms",
        asset_name,
        tdms_time_format=TdmsTimeFormat.TIME_CHANNEL,
        prefix_channel_with_group=True,
    )
    print(import_service.wait_until_complete())
    print("Upload example complete!")
