from pathlib import Path
from tempfile import NamedTemporaryFile
from typing import Dict, List, Optional, Union

try:
    from nptdms import (  # type: ignore
        ChannelObject,
        RootObject,
        TdmsChannel,
        TdmsFile,
        TdmsWriter,
        types,
    )
except ImportError as e:
    raise RuntimeError(
        "The npTDMS package is required to use the TDMS upload service. "
        "Please include this dependency in your project by specifying `sift-stack-py[tdms]`."
    ) from e

from sift_py.data_import._config import DataColumn, TimeColumn
from sift_py.data_import.config import CsvConfig
from sift_py.data_import.csv import CsvUploadService
from sift_py.data_import.status import DataImportService
from sift_py.data_import.time_format import TimeFormatType
from sift_py.ingestion.channel import ChannelDataType
from sift_py.rest import SiftRestConfig

TDMS_TO_SIFT_TYPES = {
    types.Boolean: ChannelDataType.BOOL,
    types.Int8: ChannelDataType.INT_32,
    types.Int16: ChannelDataType.INT_32,
    types.Int32: ChannelDataType.INT_32,
    types.Int64: ChannelDataType.INT_64,
    types.Uint8: ChannelDataType.UINT_32,
    types.Uint16: ChannelDataType.UINT_32,
    types.Uint32: ChannelDataType.UINT_32,
    types.Uint64: ChannelDataType.UINT_64,
    types.SingleFloat: ChannelDataType.FLOAT,
    types.DoubleFloat: ChannelDataType.DOUBLE,
}


class TdmsUploadService:
    """
    Service to upload TDMS files.
    """

    _csv_upload_service: CsvUploadService

    def __init__(self, rest_conf: SiftRestConfig):
        self._csv_upload_service = CsvUploadService(rest_conf)

    def upload(
        self,
        path: Union[str, Path],
        asset_name: str,
        group_into_components: bool = False,
        ignore_errors: bool = False,
        run_name: Optional[str] = None,
        run_id: Optional[str] = None,
    ) -> DataImportService:
        """
        Uploads the TDMS file pointed to by `path` to the specified asset.

        Set `group_into_components` to True if you want to upload the TDMS groups as
        a Sift Component.

        If `ignore_errors` is True will skip channels without timing information.

        Override `run_name` to specify the name of the run to create for this data. Default is None.
        Override `run_id` to specify the id of the run to add this data to. Default is None.
        """
        posix_path = Path(path) if isinstance(path, str) else path

        if not posix_path.is_file():
            raise Exception(f"Provided path, '{path}', does not point to a regular file.")

        with NamedTemporaryFile(mode="w", suffix=".csv") as temp_file:
            valid_channels = self._convert_to_csv(path, temp_file.name, ignore_errors)
            if not valid_channels:
                raise Exception(f"No valid channels remaining in {path}")

            csv_config = self._create_csv_config(
                valid_channels, asset_name, group_into_components, run_name, run_id
            )
            return self._csv_upload_service.upload(temp_file.name, csv_config)

    def _convert_to_csv(
        self, src_path: Union[str, Path], dst_path: Union[str, Path], ignore_errors: bool
    ) -> List[TdmsChannel]:
        """Converts the TDMS file to a temporary CSV on disk that we will upload.

        Returns the valid channels after parsing the TDMS file. Valid channels contain
        timing information.
        """

        def contains_timing(channel: TdmsChannel) -> bool:
            """Returns true if the TDMS Channel contains timing information."""
            return all(
                [
                    "wf_increment" in channel.properties,
                    "wf_start_time" in channel.properties,
                    "wf_start_offset" in channel.properties,
                ]
            )

        def normalize_name(channel_name: str) -> str:
            """Normalize channel names by invalid characters."""
            return " ".join(channel_name.replace("/", " ").split())

        src_file = TdmsFile(src_path)

        original_groups = src_file.groups()
        valid_channels: List[ChannelObject] = []
        for group in original_groups:
            for channel in group.channels():
                if contains_timing(channel):
                    new_channel = ChannelObject(
                        group=normalize_name(channel.group_name),
                        channel=normalize_name(channel.name),
                        data=channel.data,
                        properties=channel.properties,
                    )
                    valid_channels.append(new_channel)
                else:
                    if ignore_errors:
                        print(
                            f"{group.name}:{channel.name} does not contain timing information. Skipping."
                        )
                    else:
                        raise Exception(
                            f"{group.name}:{channel.name} does not contain timing information. "
                            "Set `ignore_errors` to True to skip channels without timing information."
                        )

        # Write out the new TDMS file with invalid channels removed, then convert to csv.
        with NamedTemporaryFile(mode="w") as f:
            with TdmsWriter(f.name) as tdms_writer:
                root_object = RootObject(src_file.properties)
                tdms_writer.write_segment([root_object] + original_groups + valid_channels)

            filtered_tdms_file = TdmsFile(f.name)
            df = filtered_tdms_file.as_dataframe(time_index=True, absolute_time=True)
            df.to_csv(dst_path, encoding="utf-8")

        return [channel for group in filtered_tdms_file.groups() for channel in group.channels()]

    def _create_csv_config(
        self,
        channels: List[TdmsChannel],
        asset_name: str,
        group_into_components: bool,
        run_name: Optional[str] = None,
        run_id: Optional[str] = None,
    ) -> CsvConfig:
        """Construct a CsvConfig based on metadata within the TDMS file."""
        data_config: Dict[int, DataColumn] = {}
        # Data columns start in column 2 (1-indexed)
        first_data_column = 2
        for i, channel in enumerate(channels):
            try:
                data_type = TDMS_TO_SIFT_TYPES[channel.data_type].as_human_str(api_format=True)
            except KeyError:
                data_type = None

            if data_type is None:
                raise Exception(f"{channel.name} data type not supported: {channel.data_type}")

            extra_info = ""
            for k, v in channel.properties.items():
                # Skip these since the csv config has dedicated fields for them.
                if k in ["description", "unit_string"]:
                    continue
                # Must convert datetime to a string
                elif k == "wf_start_time":
                    v = str(v)
                extra_info += f"{k}: {v}\n"

            channel_config = DataColumn(
                name=channel.name,
                data_type=data_type,
                description=f"{channel.properties.get('description')}\n{extra_info}",
                units=channel.properties.get("unit_string") or "",
            )
            if group_into_components and channel.group_name:
                channel_config.component = channel.group_name

            data_config[first_data_column + i] = channel_config

        config_info = {
            "asset_name": asset_name,
            "first_data_row": first_data_column,
            "time_column": TimeColumn(
                format=TimeFormatType.ABSOLUTE_DATETIME,
                column_number=1,
            ),
            "data_columns": data_config,
        }

        if run_name is not None:
            config_info["run_name"] = run_name

        if run_id is not None:
            config_info["run_id"] = run_id

        return CsvConfig(config_info)
