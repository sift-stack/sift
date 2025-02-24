"""
Service to upload ROS2 bag files.
"""

import csv
from glob import glob
from pathlib import Path
from typing import BinaryIO, Callable, Dict, List, Optional, Set, Union, Tuple

from rosbags.interfaces.typing import Typesdict
from rosbags.rosbag2.reader import Reader
from rosbags.typesys import Stores, get_types_from_msg, get_typestore
from rosbags.typesys.store import Typestore
from tqdm import tqdm

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
    ) -> DataImportService:
        """
        Uploads the ROS2 bag file pointed to by `path` to the specified asset.

        Arguments:
            path: Path to the ROS2 bag file.
            msg_dirs: List of directories containing custom message definitions. Each entry should be a path
                the root directory of the msg definitions (e.g, '/path/to/std_msgs').
            store: The Store type to use for the message definitions.
            asset_name: Name of the asset to upload the data to.
            ignore_errors: If True, will skip messages without definitions.
            run_name: Name of the run to create for this data.
            run_id: ID of the run to add this data to.
        """
        posix_path = Path(path) if isinstance(path, str) else path

        if not posix_path.is_dir():
            raise Exception(f"Provided path, '{path}', does not point to a directory.")

        with NamedTemporaryFile(mode="w", suffix=".csv") as temp_file:
            valid_channels = self._convert_to_csv(path, temp_file, msg_dirs, store, ignore_errors, handlers)
            if not valid_channels:
                raise Exception(f"No valid channels remaining in {path}")

            csv_config = self._create_csv_config(valid_channels, asset_name, run_name, run_id)
            print("Uploading file...")
            return self._csv_upload_service.upload(temp_file.name, csv_config)

    def _convert_to_csv(
        self,
        src_path: Union[str, Path],
        dst_file: BinaryIO,
        msg_dirs: List[Union[str, Path]],
        store: Stores,
        ignore_errors: bool,
        handlers: Optional[Dict[str, Callable]] = None ,
    ) -> List[RosChannel]:
        """Converts the ROS2 bag file to a temporary CSV on disk that we will upload.

        Returns the valid channels after parsing the ROS2 bag file.
        """
        handlers = handlers or {}
        typestore = get_typestore(store)
        msg_types = self._register_types(typestore, msg_dirs)

        ros_channels = {}

        def sanitize(name):
            result = "_".join(name.split("/"))
            if result.startswith("_"):
                result = result[1:]
            return result

        with Reader(src_path) as reader:
            # Collect all channel information from the connections.
            for connection in reader.connections:
                if connection.msgtype not in msg_types:
                    if ignore_errors:
                        print(f"WARNING: Skipping {connection.msgtype}. msg file not found.")
                        continue
                    else:
                        raise Exception(
                            f"Message type {connection.msgtype} not found in custom types."
                        )

                # Special types (typically sequences) are handled differently
                msg_def = typestore.get_msgdef(connection.msgtype)
                if connection.msgtype in handlers:
                    continue
                for field in msg_def.fields:
                    key = f"{connection.topic}:{msg_def.name}:{field}"
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
            pbar = tqdm(total=reader.message_count, unit="messages")
            for connection, timestamp, raw_data in reader.messages():
                pbar.update(1)
                if connection.msgtype not in msg_types:
                    if ignore_errors:
                        continue
                    else:
                        raise Exception(
                            f"Message type {connection.msgtype} not found in custom types."
                        )

                row = {}
                msg = typestore.deserialize_cdr(raw_data, connection.msgtype)
                msg_def = typestore.get_msgdef(connection.msgtype)
                row["time"] = timestamp

                if connection.msgtype in handlers:
                    handlers[connection.msgtype](connection.topic, timestamp, msg)
                else:
                    for field in msg_def.fields:
                        key = f"{connection.topic}:{msg_def.name}:{field}"
                        if key not in ros_channels:
                            if ignore_errors:
                                continue
                            else:
                                raise Exception(f"Message field {key} not found in custom types.")
                        channels = ros_channels[key]
                        for c in channels:
                            row[c.channel_name] = c.extract_value(msg)

                w.writerow(row)

        return [c for ros_channels in ros_channels.values() for c in ros_channels]

    def _register_types(self, typestore: Typestore, msg_dirs: List[Union[str, Path]]) -> Set[str]:
        """Register custom message types with the typestore."""
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
