import os

from dotenv import load_dotenv
from rosbags.typesys import Stores
from sift_py.data_import.rosbags import RosbagsUploadService
from sift_py.rest import SiftRestConfig

if __name__ == "__main__":
    """
    Example of uploading a rosbag2 into Sift.
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

    ros2_upload_service = RosbagsUploadService(rest_config)
    import_service = ros2_upload_service.upload(
        "data/talker",
        ["data/builtin_interfaces", "data/rcl_interfaces", "data/std_msgs"],
        Stores.ROS2_HUMBLE,
        asset_name,
    )
    print(import_service.wait_until_complete())
    print("Upload example complete!")
