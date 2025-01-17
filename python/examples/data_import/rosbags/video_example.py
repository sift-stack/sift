"""
This example will not run as-is. It is meant to be a reference for how to extract video from a rosbag2
"""

import os

import ffmpeg
from dotenv import load_dotenv
from rosbags.typesys import Stores
from sift_py.data_import.rosbags import RosbagsUploadService
from sift_py.rest import SiftRestConfig

if __name__ == "__main__":
    """
    Example of uploading a rosbag2 into Sift and extracting video.
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

    # Initialize video
    output_video_filename = "output_video.mp4"
    width, height, channels = 1280, 720, 3
    video_processor = (
        ffmpeg.input("pipe:", s=f"{width}x{height}", framerate=30)
        .output(output_video_filename, pix_fmt="yuv420p")
        .run_async(pipe_stdin=True)
    )
    video_start_ns = None
    video_end_ns = None

    # Callback handler to write video frames.
    # Assumes only one video topic.
    def write_video_frame_handler(topic, timestamp, msg):
        global video_start_ns, video_end_ns
        if video_start_ns is None:
            video_start_ns = timestamp
        video_end_ns = timestamp
        video_processor.stdin.write(msg.data)

    ros2_upload_service = RosbagsUploadService(rest_config)
    import_service = ros2_upload_service.upload(
        "data/with/video",
        ["common_interfaces/sensor_msgs"],
        Stores.ROS2_HUMBLE,
        asset_name,
        handlers={"sensor_msgs/msg/CompressedImage": write_video_frame_handler},
    )
    video_processor.stdin.close()
    video_processor.wait()

    print(import_service.wait_until_complete())
    print("CSV upload complete complete!")

    print("Use the following info to upload the video to Sift:")
    print(f"output_video_filename: {output_video_filename}")
    print(f"video_start_ns: {video_start_ns}")
    print(f"video_end_ns: {video_end_ns}")
