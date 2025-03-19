"""
Service to upload ROS2 bag files.


ROS organizes data exchange through topics, messages, and fields:
    * topics: Named messages that facilitate communication between ROS nodes.
    * messages: Data structures published and subscribed to on topics. Each message type defines a specific schema.
    * fields: Individual data elements within a message, such as integers, floats, strings, or nested structures.

This class extracts messages from a ROS bag, flattens their fields, and prepares them for uploading to Sift.
"""

import csv
import struct
from glob import glob
from pathlib import Path
from typing import Callable, Dict, List, Optional, Set, TextIO, Union

from alive_progress import alive_it  # type: ignore

try:
    from rosbags.interfaces.typing import Typesdict
    from rosbags.rosbag2.reader import Reader
    from rosbags.typesys import Stores, get_types_from_msg, get_typestore
    from rosbags.typesys.store import Typestore
except ImportError as e:
    raise RuntimeError(
        "The rosbags package is required to use the rosbag upload service. "
        "Please include this dependency in your project by specifying `sift-stack-py[rosbags]`."
    ) from e

from sift_py.data_import._config import DataColumn, TimeColumn
from sift_py.data_import._ros_channel import RosChannel
from sift_py.data_import.config import CsvConfig
from sift_py.data_import.csv import CsvUploadService
from sift_py.data_import.status import DataImportService
from sift_py.data_import.tempfile import NamedTemporaryFile
from sift_py.data_import.time_format import TimeFormatType
from sift_py.rest import SiftRestConfig


class RosbagsUploadService:
    """
    Service to upload ROS2 bag files.
    """

    _csv_upload_service: CsvUploadService

    def __init__(self, rest_conf: SiftRestConfig):
        self._csv_upload_service = CsvUploadService(rest_conf)

    def upload(
        self,
        path: Union[str, Path],
        msg_dirs: List[Union[str, Path]],
        store: Stores,
        asset_name: str,
        ignore_errors: bool = False,
        run_name: Optional[str] = None,
        run_id: Optional[str] = None,
        handlers: Optional[Dict[str, Callable]] = None,
        show_progress: bool = True,
    ) -> DataImportService:
        """
        Uploads the ROS2 bag file pointed to by `path` to the specified asset.

        Arguments:
            path: Path to the ROS2 bag file.
            msg_dirs: List of directories containing message definitions.
                Each entry should be a path the root directory of the msg definitions (e.g, `/path/to/std_msgs`).
                Inspect your topics and verify that the 'type' matches the directory structure of your
                message definitions. For example if the type is `custom_msgs/msg/MyCustomMessage` your
                directory structure should match that and you should include `/path/to/custom_msgs`
                in the `msg_dirs` list passed into this function.
            store: The Store type to use for the message definitions.
            asset_name: Name of the asset to upload the data to.
            ignore_errors: If True, will skip messages without definitions.
            run_name: Name of the run to create for this data.
            run_id: ID of the run to add this data to.
            handlers: Dictionary of messages to callbacks for custom processing or sequence data
                (e.g, images or videos). Keys should be the ROS topic, value is a callable with
                the following signature:
                    def callback(topic: str, timestamp: int, msg: object)
                        ...
            show_progress: Whether to show the status bar or not.
        """
        posix_path = Path(path) if isinstance(path, str) else path

        if not posix_path.exists():
            raise Exception(f"Provided path, '{path}', does not exists")

        if not posix_path.is_dir():
            raise Exception(f"Provided path, '{path}', does not point to a directory.")

        with NamedTemporaryFile(mode="wt", suffix=".csv.gz") as temp_file:
            try:
                valid_channels = self._convert_to_csv(
                    path,
                    temp_file,
                    msg_dirs,
                    store,
                    ignore_errors,
                    handlers,
                    show_progress,
                )
                if not valid_channels:
                    raise Exception(f"No valid channels remaining in {path}")
            except struct.error as e:
                raise Exception(
                    f"Failed to parse the rosbag. Ensure you're using the correct type store. Original error: {e}"
                )

            csv_config = self._create_csv_config(valid_channels, asset_name, run_name, run_id)

            print("Uploading file")
            return self._csv_upload_service.upload(temp_file.name, csv_config, show_progress)

    def _convert_to_csv(
        self,
        src_path: Union[str, Path],
        dst_file: TextIO,
        msg_dirs: List[Union[str, Path]],
        store: Stores,
        ignore_errors: bool,
        handlers: Optional[Dict[str, Callable]] = None,
        show_progress: bool = True,
    ) -> List[RosChannel]:
        """Converts the ROS2 bag file to a temporary CSV on disk that we will upload.


        Args:
            src_path: The path of the rosbag.
            dst_file: The path to save the CSV.
            msg_dirs: The list of directories containing rosbag message definitions.
            store: The rosbag type store to use.
            ignore_errors: Whether to ignore errors (e.g, unknown message definitions).
            handlers: Dictionary of messages to callbacks for custom processing or sequence data
                (e.g, images or videos). Keys should be the ROS topic, value is a callable with
                the following signature:
                    def callback(topic: str, timestamp: int, msg: object)
                        ...
            show_progress: Whether to show the status bar or not.
        Returns:
            The list valid channels after parsing the ROS2 bag file.
        """
        handlers = handlers or {}
        typestore = get_typestore(store)
        registered_msg_types = self._register_types(typestore, msg_dirs)

        # Map each (topic, message, field) combination to a list of RosChannels
        ros_channels: Dict[str, List[RosChannel]] = {}

        def sanitize(name):
            result = "_".join(name.split("/"))
            if result.startswith("_"):
                result = result[1:]
            return result

        def get_key(connection, msg_def, field):
            return f"{connection.topic}:{msg_def.name}:{field}"

        with Reader(src_path) as reader:
            # Collect all channel information from the connections.
            for connection in reader.connections:
                if connection.msgtype not in registered_msg_types:
                    if ignore_errors:
                        print(f"WARNING: Skipping {connection.msgtype}. msg file not found.")
                        continue
                    else:
                        raise Exception(
                            f"Message type {connection.msgtype} not found in custom types."
                        )

                # Flatten and collect all underlying fields in this message as RosChannels
                msg_def = typestore.get_msgdef(connection.msgtype)
                for field in msg_def.fields:
                    key = get_key(connection, msg_def, field)
                    if key in ros_channels:
                        raise Exception(f"Duplicate key: {key}")
                    ros_channels[key] = RosChannel.get_underlying_fields(
                        sanitize(connection.topic), field, typestore
                    )

            headers = ["time"] + [
                c.channel_name for channels in ros_channels.values() for c in channels
            ]
            w = csv.DictWriter(dst_file, headers)
            w.writeheader()

            print("Processing rosbag messages")
            for connection, timestamp, raw_data in alive_it(
                reader.messages(),
                total=reader.message_count,
                unit=" messages",
                disable=not show_progress,
            ):
                if connection.msgtype not in registered_msg_types:
                    if ignore_errors:
                        continue
                    else:
                        raise Exception(
                            f"Message type {connection.msgtype} not found in custom types."
                        )

                row: Dict[str, Union[int, float, bool, str]] = {}
                msg = typestore.deserialize_cdr(raw_data, connection.msgtype)
                msg_def = typestore.get_msgdef(connection.msgtype)
                row["time"] = timestamp

                if connection.topic in handlers:
                    handlers[connection.topic](connection.topic, timestamp, msg)

                for field in msg_def.fields:
                    key = get_key(connection, msg_def, field)
                    if key not in ros_channels:
                        if ignore_errors:
                            continue
                        else:
                            raise Exception(f"Message field {key} not found in custom types.")
                    channels = ros_channels[key]
                    for c in channels:
                        row[c.channel_name] = c.extract_value(msg)

                w.writerow(row)

        # Close the file to make sure all contents are written.
        # Required if using gzip compression to ensure all data is flushed: https://bugs.python.org/issue1110242
        dst_file.close()

        return [c for ros_channels in ros_channels.values() for c in ros_channels]

    def _register_types(self, typestore: Typestore, msg_dirs: List[Union[str, Path]]) -> Set[str]:
        """Register custom message types with the typestore.

        Args:
            typestore: The type store to register messages against.
            msg_dirs: The list of directories containing message definitions.

        Returns:
            Set of all registered message definitions.
        """
        msg_types: Typesdict = {}
        for dir_pathname in msg_dirs:
            dir_path = Path(dir_pathname)
            for msg_pathname in glob(str(dir_path / "**" / "*.msg")):
                relative_msg_path = Path(msg_pathname).relative_to(dir_pathname)
                msg_path_from_root = dir_path.name / relative_msg_path
                msg_types.update(
                    get_types_from_msg(
                        Path(msg_pathname).read_text(),
                        str(msg_path_from_root).replace("\\", "/").replace(".msg", ""),
                    )
                )

        typestore.register(msg_types)

        return set(msg_types.keys())

    def _create_csv_config(
        self,
        channels: List[RosChannel],
        asset_name: str,
        run_name: Optional[str] = None,
        run_id: Optional[str] = None,
    ) -> CsvConfig:
        """Construct a CsvConfig based on metadata within the ROS2 bag file."""
        data_config: Dict[int, DataColumn] = {}
        # Data columns start in column 2 (1-indexed)
        first_data_column = 2
        for i, channel in enumerate(channels):
            data_type = channel.data_type

            channel_config = DataColumn(
                name=channel.channel_name,
                data_type=data_type,
                description="",
                units="",
            )

            data_config[first_data_column + i] = channel_config

        config_info = {
            "asset_name": asset_name,
            "first_data_row": first_data_column,
            "time_column": TimeColumn(
                format=TimeFormatType.ABSOLUTE_UNIX_NANOSECONDS,
                column_number=1,
            ),
            "data_columns": data_config,
        }

        if run_name is not None:
            config_info["run_name"] = run_name

        if run_id is not None:
            config_info["run_id"] = run_id

        return CsvConfig(config_info)
