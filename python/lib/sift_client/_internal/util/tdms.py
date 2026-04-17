from __future__ import annotations

import json
from typing import TYPE_CHECKING

import numpy as np

if TYPE_CHECKING:
    from pathlib import Path
from nptdms import TdmsChannel, TdmsFile, TdmsGroup, types

from sift_client._internal.util.numpy_types import numpy_to_sift_type
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import (
    TdmsComplexComponent,
    TdmsDataColumn,
    TdmsFallbackMethod,
    TdmsImportConfig,
)

# Common property names used to detect the units of a channel in TDMS files.
COMMON_UNIT_PROPS = [
    "unit_string",
    "NI_UnitDescription",
]

# Common property names used to detect the description of a channel in TDMS files.
COMMON_DESCRIPTION_PROPS = ["description", "NI_Description", "Description"]

# Common unit strings that indicate a channel represents time (waveform x-axis).
COMMON_WAVEFORM_TIME_UNITS = [
    "s",
    "sec",
    "second",
    "seconds",
    "ms",
    "millisecond",
    "milliseconds",
    "us",
    "microsecond",
    "microseconds",
    "ns",
    "nanosecond",
    "nanoseconds",
]


def detect_properties(obj: TdmsChannel | TdmsGroup, possible_props: list, default: str = "") -> str:
    """Return the first matching property value from a list of possible property names."""
    for prop in possible_props:
        value = obj.properties.get(prop)
        if value:
            return value
    return default


def create_description(group_description: str, channel_description: str) -> str:
    """Combine TDMS group and channel descriptions into a single Sift description."""
    group_description = group_description.strip()
    channel_description = channel_description.strip()
    group_entry = f"Group: {group_description}" if group_description else ""
    channel_entry = f"Channel: {channel_description}" if channel_description else ""
    return "\n".join([group_entry, channel_entry]).strip()


def detect_enum_types(channel: TdmsChannel) -> dict[str, int] | None:
    """Check if the TDMS channel is embedded with enum configs.

    Returns a name-to-key mapping, or None if no enum config is present.
    """
    name = f"{channel.group_name}/{channel.name}"

    enum_config_data = channel.properties.get("enum_config")
    if not enum_config_data:
        return None
    try:
        enum_configs = json.loads(enum_config_data)
    except Exception as e:
        raise ValueError(f"Failed to decode JSON enum_configs for {name}: {e}") from e

    enum_types: dict[str, int] = {}
    for enum_key, enum_name in enum_configs.items():
        try:
            key = int(enum_key)
        except ValueError as e:
            raise ValueError(f"{enum_key} is not a valid enum integer for ({name})") from e
        if key < 0:
            raise ValueError(f"{enum_key} is not a valid unsigned enum integer ({name})")
        enum_types[enum_name] = key

    return enum_types if enum_types else None


def is_waveform_time_channel(channel: TdmsChannel) -> bool:
    """A waveform channel carries wf_start_offset and wf_increment properties."""
    return "wf_start_offset" in channel.properties and "wf_increment" in channel.properties


def find_time_channel(group: TdmsGroup) -> str | None:
    """Return the name of a dedicated time channel in the group, if one exists.

    Detection order:
    1. Group-level 'xchannel' property.
    2. Look for the time channel in the first index.

    https://www.ni.com/en/support/documentation/supplemental/12/writing-data-management-ready-tdms-files.html
    """
    channels = group.channels()
    channel_names = {ch.name for ch in channels}

    # 1. Explicit xchannel property set by the file author.
    xchannel = group.properties.get("xchannel")
    if xchannel and xchannel in channel_names:
        return xchannel

    # 2. Native datetime type in first index
    if channels and channels[0].data_type == types.TimeStamp:
        return channels[0].name

    return None


def detect_config(
    file_path: str | Path,
    asset_name: str = "",
    fallback_method: TdmsFallbackMethod = TdmsFallbackMethod.FAIL_ON_ERROR,
) -> TdmsImportConfig:
    """Detect a TDMS import config by inspecting the file's channels.

    Args:
        file_path: Path to the TDMS file.
        asset_name: The asset name to set on the config.
        fallback_method: How to handle channels with missing timing information.

    Returns:
        A TdmsImportConfig populated with detected channel configurations.
    """
    data: list[TdmsDataColumn] = []

    with TdmsFile.open(file_path) as tdms_file:
        for group in tdms_file.groups():
            group_name = group.name
            time_channel_name = find_time_channel(group)
            group_description = detect_properties(group, COMMON_DESCRIPTION_PROPS)

            for channel in group.channels():
                tdms_channel_name = channel.name

                # Skip channels that are used as a time axis
                if tdms_channel_name == time_channel_name:
                    continue

                # Channel name will always be <group>.<channel>
                channel_name = f"{group_name}.{tdms_channel_name}"

                units = detect_properties(channel, COMMON_UNIT_PROPS)
                channel_description = detect_properties(channel, COMMON_DESCRIPTION_PROPS)
                description = create_description(group_description, channel_description)
                enum_types = detect_enum_types(channel)

                candidates: list[tuple[str, ChannelDataType, TdmsComplexComponent | None]] = []
                if np.issubdtype(channel.dtype, np.complexfloating):
                    # Split complex channel into separate .real and .imag channels.
                    sift_type = numpy_to_sift_type(channel.dtype)
                    candidates.append(
                        (f"{channel_name}.real", sift_type, TdmsComplexComponent.REAL)
                    )
                    candidates.append(
                        (f"{channel_name}.imag", sift_type, TdmsComplexComponent.IMAGINARY)
                    )
                else:
                    sift_type = (
                        ChannelDataType.ENUM if enum_types else numpy_to_sift_type(channel.dtype)
                    )
                    candidates.append((channel_name, sift_type, None))

                for name, data_type, complex_component in candidates:
                    # If a time channel is present, that takes priority.
                    # Some applications will generate invalid waveform
                    # properties that are not meant to be used.
                    if time_channel_name is not None:
                        data.append(
                            TdmsDataColumn(
                                group_name=group_name,
                                channel_name=tdms_channel_name,
                                name=name,
                                data_type=data_type,
                                units=units,
                                description=description,
                                time_channel_name=time_channel_name,
                                complex_component=complex_component,
                                enum_types=enum_types,
                            )
                        )
                    elif is_waveform_time_channel(channel):
                        data.append(
                            TdmsDataColumn(
                                group_name=group_name,
                                channel_name=tdms_channel_name,
                                name=name,
                                data_type=data_type,
                                units=units,
                                description=description,
                                time_channel_name=None,
                                complex_component=complex_component,
                                enum_types=enum_types,
                            )
                        )
                    # Non time series data (e.g, binary blob, spectrum data, etc.)
                    else:
                        if fallback_method == TdmsFallbackMethod.IGNORE_ERROR:
                            continue
                        raise ValueError(f"No timing information for {channel_name}")

    return TdmsImportConfig(
        asset_name=asset_name,
        data=data,
        fallback_method=fallback_method,
    )
