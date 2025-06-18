import warnings
from collections import namedtuple
from csv import DictWriter
from enum import Enum
from pathlib import Path
from typing import Dict, List, Optional, Sequence, TextIO, Union

from pandas import to_datetime

try:
    from nptdms import (  # type: ignore
        ChannelObject,
        RootObject,
        TdmsChannel,
        TdmsFile,
        TdmsGroup,
        TdmsWriter,
        types,
    )
except ImportError as e:
    raise RuntimeError(
        "The npTDMS package is required to use the TDMS upload service. "
        "Please include this dependency in your project by specifying `sift-stack-py[tdms]`."
    ) from e

from sift_py._internal.channel import channel_fqn as _channel_fqn
from sift_py.data_import._config import DataColumn, TimeColumn
from sift_py.data_import.config import CsvConfig
from sift_py.data_import.csv import CsvUploadService
from sift_py.data_import.status import DataImportService
from sift_py.data_import.tempfile import NamedTemporaryFile
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
    types.String: ChannelDataType.STRING,
}


class TdmsTimeFormat(Enum):
    # Time information is encoded as a waveform.
    WAVEFORM = "waveform"
    # Time information is encoded as a separate TDMS channel.
    TIME_CHANNEL = "time_channel"


# The common time channel name to use with TdmsTimeFormat.TIME_CHANNEL.
TIME_CHANNEL_NAME = "Time"

# Implements the same interface as TdmsChannel. Allows us to create
# TdmsChannel like objects without having to save and read the channels to
# a file.
_TdmsChannel = namedtuple("_TdmsChannel", ["group_name", "name", "data_type", "data", "properties"])


CHARACTER_REPLACEMENTS = {
    '"': "_",
    "\\": "_",
    "`": "_",
    "~": "_",
    "|": "_",
}


def sanitize_string(input_string: str) -> str:
    """
    Removes the characters ", \\, `, ~, and | from the input string.

    See https://docs.siftstack.com/docs/data-model/assets-channels-runs#assets-and-channels

    Args:
        input_string: The string to sanitize.

    Returns:
        The sanitized string.
    """
    return input_string.translate(str.maketrans(CHARACTER_REPLACEMENTS))  # type: ignore


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
        prefix_channel_with_group: bool = False,
        group_into_components: bool = False,  # Deprecated
        ignore_errors: bool = False,
        run_name: Optional[str] = None,
        run_id: Optional[str] = None,
        tdms_time_format: TdmsTimeFormat = TdmsTimeFormat.WAVEFORM,
    ) -> DataImportService:
        """
        Uploads the TDMS file pointed to by `path` to the specified asset.

        Args:
            path: The path to the file to upload.
            asset_name: The name of the asset to upload to.
            prefix_channel_with_group: Set to True if you want to prefix the channel name with TDMS group.
                This can later be used to group into folders in the Sift UI. Default is False.
            ignore_errors: If True will skip channels without timing information. Default is False.
            run_name: The name of the run to create for this data. Default is None.
            run_id: The id of the run to add this data to. Default is None.
            tdms_time_format: Specify how timing information is encoded in the file. Default is WAVEFORM.
                If using the TIME_CHANNEL format, timestamps should use the LabVIEW/TDMS epoch (number of
                seconds since 01/01/1904 00:00:00.00 UTC).

        Returns:
            The DataImportService used to get the status of the import.
        """
        if group_into_components:
            warnings.warn(
                "`group_into_components` has been renamed to `prefix_channel_with_group` to reflect the"
                " deprecation of Sift Channel components. `component` will be removed in 1.0.0. "
                "See docs for more details: https://docs.siftstack.com/docs/glossary#component",
                FutureWarning,
            )
            prefix_channel_with_group = group_into_components

        posix_path = Path(path) if isinstance(path, str) else path

        if not posix_path.is_file():
            raise Exception(f"Provided path, '{path}', does not point to a regular file.")

        with NamedTemporaryFile(mode="wt", suffix=".csv.gz") as temp_file:
            csv_config = self._convert_to_csv(
                path,
                temp_file,
                asset_name,
                prefix_channel_with_group,
                ignore_errors,
                run_name,
                run_id,
                tdms_time_format,
            )
            return self._csv_upload_service.upload(temp_file.name, csv_config)

    def _convert_to_csv(
        self,
        src_path: Union[str, Path],
        dst_file: TextIO,
        asset_name: str,
        prefix_channel_with_group: bool,
        ignore_errors: bool,
        run_name: Optional[str],
        run_id: Optional[str],
        tdms_time_format: TdmsTimeFormat,
    ) -> CsvConfig:
        """Converts the TDMS file to a temporary CSV on disk that we will upload.

        Args:
            src_path: The source path to the TDMS file.
            dst_file: The output CSV file.
            asset_name: The name of the asset to upload to.
            prefix_channel_with_group: Set to True if you want to prefix the channel name with TDMS group.
                This can later be used to group into folders in the Sift UI.
            ignore_errors: If True will skip channels without timing information.
            run_name: The name of the run to create for this data.
            run_id: The id of the run to add this data to.
            tdms_time_format: Specify how timing information is encoded in the file.

        Returns:
            The CSV config for the import.
        """
        if tdms_time_format == TdmsTimeFormat.WAVEFORM:
            convert_func = self._convert_waveform_tdms_to_csv
        elif tdms_time_format == TdmsTimeFormat.TIME_CHANNEL:
            convert_func = self._convert_time_channel_tdms_to_csv
        else:
            raise Exception(f"Unknown TDMS time format: {tdms_time_format}")

        return convert_func(
            src_path,
            dst_file,
            asset_name,
            prefix_channel_with_group,
            ignore_errors,
            run_name,
            run_id,
        )

    def _convert_waveform_tdms_to_csv(
        self,
        src_path: Union[str, Path],
        dst_file: TextIO,
        asset_name: str,
        prefix_channel_with_group: bool,
        ignore_errors: bool,
        run_name: Optional[str],
        run_id: Optional[str],
    ) -> CsvConfig:
        """Converts the TDMS file to a temporary CSV on disk using channel waveform properties.

        Args:
            src_path: The source path to the TDMS file.
            dst_file: The output CSV file.
            asset_name: The name of the asset to upload to.
            prefix_channel_with_group: Set to True if you want to prefix the channel name with TDMS group.
                This can later be used to group into folders in the Sift UI.
            ignore_errors: If True will skip channels without timing information.
            run_name: The name of the run to create for this data.
            run_id: The id of the run to add this data to.

        Returns:
            The CSV config for the import.
        """

        def contains_timing(channel: TdmsChannel) -> bool:
            """Returns True if the TDMS Channel contains timing information."""
            return all(
                [
                    "wf_increment" in channel.properties,
                    "wf_start_time" in channel.properties,
                    "wf_start_offset" in channel.properties,
                ]
            )

        src_file = TdmsFile(src_path)

        original_groups = src_file.groups()
        valid_channels: List[ChannelObject] = []
        for group in original_groups:
            for channel in group.channels():
                if contains_timing(channel):
                    new_channel = ChannelObject(
                        group=sanitize_string(channel.group_name),
                        channel=sanitize_string(channel.name),
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

        if not valid_channels:
            raise Exception(f"No valid channels found in {src_path}")

        # Write out the new TDMS file with invalid channels removed, then convert to csv.
        with NamedTemporaryFile(mode="w") as f:
            with TdmsWriter(f.name) as tdms_writer:
                root_object = RootObject(src_file.properties)
                tdms_writer.write_segment([root_object] + original_groups + valid_channels)

            filtered_tdms_file = TdmsFile(f.name)
            df = filtered_tdms_file.as_dataframe(time_index=True, absolute_time=True)
            df.to_csv(dst_file, encoding="utf-8")

            # Close the file to make sure all contents are written.
            # Required if using gzip compression to ensure all data
            # is flushed: https://bugs.python.org/issue1110242
            dst_file.close()

        valid_tdms_channels = [
            channel for group in filtered_tdms_file.groups() for channel in group.channels()
        ]

        return self._create_csv_config(
            channels=valid_tdms_channels,
            asset_name=asset_name,
            prefix_channel_with_group=prefix_channel_with_group,
            run_name=run_name,
            run_id=run_id,
        )

    def _convert_time_channel_tdms_to_csv(
        self,
        src_path: Union[str, Path],
        dst_file: TextIO,
        asset_name: str,
        prefix_channel_with_group: bool,
        ignore_errors: bool,
        run_name: Optional[str],
        run_id: Optional[str],
    ) -> CsvConfig:
        """Converts the TDMS file to a temporary CSV using time channels in each group.

        Args:
            src_path: The source path to the TDMS file.
            dst_file: The output CSV file.
            asset_name: The name of the asset to upload to.
            prefix_channel_with_group: Set to True if you want to prefix the channel name with TDMS group.
                This can later be used to group into folders in the Sift UI.
            ignore_errors: If True will skip channels without timing information.
            run_name: The name of the run to create for this data.
            run_id: The id of the run to add this data to.

        Returns:
            The CSV config for the import.
        """

        def get_time_channels(group: TdmsGroup) -> List[TdmsChannel]:
            """Returns the time channels."""
            return [channel for channel in group.channels() if channel.data_type == types.TimeStamp]

        src_file = TdmsFile(src_path)

        # Process each group by setting the Time channel within each group
        # to have a common name (i.e, "Time").
        valid_groups: Dict[str, List[_TdmsChannel]] = {}
        all_tdms_channels: List[_TdmsChannel] = []
        for group in src_file.groups():
            updated_group_name = sanitize_string(group.name)
            time_channels = get_time_channels(group)
            if len(time_channels) != 1:
                msg = (
                    f"{group.name} contains more than one time channel"
                    if len(time_channels) > 1
                    else "no time channels"
                )
                if ignore_errors:
                    print(f"{msg}. Skipping.")
                    continue
                else:
                    raise Exception(f"{msg}. Set `ignore_errors` to True to skip this group.")

            time_channel = time_channels[0]
            updated_channels = []
            for channel in group.channels():
                if channel == time_channel:
                    updated_channel_name = TIME_CHANNEL_NAME
                    data = to_datetime(channel.data).tz_localize("UTC")
                    data = (
                        data.strftime("%Y-%m-%dT%H:%M:%S.%f")
                        + data.nanosecond.map(lambda ns: f"{ns % 1000:03d}")
                        + "Z"
                    )
                else:
                    if len(time_channel.data) != len(channel.data):
                        msg = f"Length mismatch between {time_channel.name} and {channel.name}"
                        if ignore_errors:
                            print(f"{msg}. Skipping.")
                            continue
                        else:
                            raise Exception(
                                f"{msg}. Set `ignore_errors` to True to skip this channel."
                            )

                    updated_channel_name = sanitize_string(channel.name)
                    data = channel.data

                updated_channel = _TdmsChannel(
                    group_name=updated_group_name,
                    name=updated_channel_name,
                    data_type=channel.data_type,
                    data=data,
                    properties=channel.properties,
                )
                updated_channels.append(updated_channel)

                if channel != time_channel:
                    all_tdms_channels.append(updated_channel)

            if len(updated_channels) > 1:
                valid_groups[updated_group_name] = updated_channels
            else:
                msg = f"{group.name} does not contain any valid channels"
                if ignore_errors:
                    print(f"{msg}. Skipping.")
                    continue
                else:
                    raise Exception(f"{msg}. Set `ignore_errors` to True to skip this group.")

        if not valid_groups:
            raise Exception(f"No valid groups remaining in {src_path}")

        # Write the CSV manually instead of calling pandas.concat
        # in order to preserve the data types. Calling pandas.concat will end up casting
        # everything to a double when the channels have different number of points
        # since it has to fill the empty cells with NaN. By writing the CSV manually
        # we can write out empty cells.
        headers = [TIME_CHANNEL_NAME] + [channel.name for channel in all_tdms_channels]
        csv_writer = DictWriter(dst_file, headers)
        csv_writer.writeheader()
        rows = []
        for updated_channels in valid_groups.values():
            n_points = len(updated_channels[0].data)
            for i in range(n_points):
                rows.append({channel.name: channel.data[i] for channel in updated_channels})
        csv_writer.writerows(rows)

        # Close the file to make sure all contents are written.
        # Required if using gzip compression to ensure all data
        # is flushed: https://bugs.python.org/issue1110242
        dst_file.close()

        return self._create_csv_config(
            channels=all_tdms_channels,
            asset_name=asset_name,
            prefix_channel_with_group=prefix_channel_with_group,
            run_name=run_name,
            run_id=run_id,
            time_format=TimeFormatType.ABSOLUTE_RFC3339,
        )

    def _create_csv_config(
        self,
        channels: Sequence[Union[TdmsChannel, _TdmsChannel]],
        asset_name: str,
        prefix_channel_with_group: bool,
        run_name: Optional[str] = None,
        run_id: Optional[str] = None,
        time_format: TimeFormatType = TimeFormatType.ABSOLUTE_DATETIME,
    ) -> CsvConfig:
        """Construct a CsvConfig based on metadata within the TDMS file.

        Args:
            channels: The collection of channels.
            asset_name: The name of the asset.
            prefix_channel_with_group: Set to True if you want to prefix the channel name with TDMS group.
                This can later be used to group into folders in the Sift UI.
            run_name: The name of the run to create for this data. Default is None.
            run_id: The id of the run to add this data to. Default is None.
            time_format: The CSV time format. Default is ABSOLUTE_DATETIME.

        Returns:
            The CSV config.
        """
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

            channel_config = DataColumn(
                name=_channel_fqn(name=channel.name, component=channel.group_name)
                if prefix_channel_with_group and channel.group_name
                else channel.name,
                data_type=data_type,
                description=channel.properties.get("description", ""),
                units=channel.properties.get("unit_string") or "",
            )

            data_config[first_data_column + i] = channel_config

        config_info = {
            "asset_name": asset_name,
            "first_data_row": first_data_column,
            "time_column": TimeColumn(
                format=time_format,
                column_number=1,
            ),
            "data_columns": data_config,
        }

        if run_name is not None:
            config_info["run_name"] = run_name

        if run_id is not None:
            config_info["run_id"] = run_id

        return CsvConfig(config_info)
