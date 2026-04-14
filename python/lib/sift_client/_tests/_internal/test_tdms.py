"""Tests for detect_config (TDMS)."""

import numpy as np
import pytest
from nptdms import ChannelObject, GroupObject, RootObject, TdmsWriter

from sift_client._internal.util.tdms import detect_config
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import TdmsComplexComponent, TdmsFallbackMethod


@pytest.fixture
def create_tdms_file(tmp_path):
    """Return a helper that writes a TDMS file and returns its path."""
    file_path = tmp_path / "test.tdms"

    def _create(root_props=None, groups=None):
        """Write a TDMS file.

        Args:
            root_props: dict of root-level file properties.
            groups: list of (group_name, channels) tuples where channels is a list of
                ChannelObject instances.
        """
        root = RootObject(properties=root_props or {})
        with TdmsWriter(file_path) as writer:
            for group_name, channels in groups or []:
                group = GroupObject(group_name)
                writer.write_segment([root, group, *channels])
        return file_path

    return _create


class TestDetectConfig:
    def test_waveform_channels(self, create_tdms_file):
        """Channels with wf_start_offset and wf_increment are detected as waveform channels."""
        path = create_tdms_file(
            groups=[
                (
                    "sensors",
                    [
                        ChannelObject(
                            "sensors",
                            "voltage",
                            np.array([1.0, 2.0, 3.0], dtype="float64"),
                            properties={
                                "wf_start_offset": 0.0,
                                "wf_increment": 0.001,
                                "wf_start_time": np.datetime64("2024-01-01T00:00:00"),
                            },
                        ),
                    ],
                )
            ]
        )

        config = detect_config(path)

        assert len(config.data) == 1
        assert config.data[0].name == "sensors.voltage"
        assert config.data[0].data_type == ChannelDataType.DOUBLE
        assert config.data[0].time_channel_name is None
        assert config.data[0].group_name == "sensors"
        assert config.data[0].channel_name == "voltage"

    def test_time_channel_detection(self, create_tdms_file):
        """A channel with TimeStamp type is used as the time source and excluded from data."""
        path = create_tdms_file(
            groups=[
                (
                    "group1",
                    [
                        ChannelObject(
                            "group1",
                            "timestamp",
                            np.array(
                                ["2024-01-01", "2024-01-02"],
                                dtype="datetime64[ns]",
                            ),
                        ),
                        ChannelObject(
                            "group1",
                            "temperature",
                            np.array([20.5, 21.0], dtype="float32"),
                        ),
                    ],
                )
            ]
        )

        config = detect_config(path)

        channel_names = [d.name for d in config.data]
        assert "group1.timestamp" not in channel_names
        assert "group1.temperature" in channel_names
        assert config.data[0].time_channel_name == "timestamp"
        assert config.data[0].data_type == ChannelDataType.FLOAT

    def test_common_time_name_detection(self, create_tdms_file):
        """Channels named 'time', 'Time', etc. are detected as time channels."""
        path = create_tdms_file(
            groups=[
                (
                    "data",
                    [
                        ChannelObject(
                            "data",
                            "time",
                            np.array([0.0, 0.1, 0.2], dtype="float64"),
                        ),
                        ChannelObject(
                            "data",
                            "pressure",
                            np.array([101.3, 101.4, 101.5], dtype="float64"),
                        ),
                    ],
                )
            ]
        )

        config = detect_config(path)

        channel_names = [d.name for d in config.data]
        assert "data.time" not in channel_names
        assert "data.pressure" in channel_names
        assert config.data[0].time_channel_name == "time"

    def test_complex_channels_split(self, create_tdms_file):
        """Complex-valued channels are split into .real and .imag entries."""
        path = create_tdms_file(
            groups=[
                (
                    "rf",
                    [
                        ChannelObject(
                            "rf",
                            "signal",
                            np.array([1 + 2j, 3 + 4j], dtype="complex128"),
                            properties={
                                "wf_start_offset": 0.0,
                                "wf_increment": 0.001,
                                "wf_start_time": np.datetime64("2024-01-01T00:00:00"),
                            },
                        ),
                    ],
                )
            ]
        )

        config = detect_config(path)

        assert len(config.data) == 2
        names = [d.name for d in config.data]
        assert "rf.signal.real" in names
        assert "rf.signal.imag" in names

        real_col = next(d for d in config.data if d.name == "rf.signal.real")
        imag_col = next(d for d in config.data if d.name == "rf.signal.imag")
        assert real_col.complex_component == TdmsComplexComponent.REAL
        assert imag_col.complex_component == TdmsComplexComponent.IMAGINARY
        assert real_col.data_type == ChannelDataType.DOUBLE
        assert imag_col.data_type == ChannelDataType.DOUBLE

    def test_unit_and_description_detection(self, create_tdms_file):
        """Units and descriptions are read from TDMS channel properties."""
        path = create_tdms_file(
            groups=[
                (
                    "sensors",
                    [
                        ChannelObject(
                            "sensors",
                            "voltage",
                            np.array([1.0, 2.0], dtype="float64"),
                            properties={
                                "wf_start_offset": 0.0,
                                "wf_increment": 0.001,
                                "wf_start_time": np.datetime64("2024-01-01T00:00:00"),
                                "unit_string": "V",
                                "description": "Supply voltage",
                            },
                        ),
                    ],
                )
            ]
        )

        config = detect_config(path)

        assert config.data[0].units == "V"
        assert config.data[0].description == "Supply voltage"

    def test_fallback_fail_on_error(self, create_tdms_file):
        """Channels without timing info raise ValueError when fallback is FAIL_ON_ERROR."""
        path = create_tdms_file(
            groups=[
                (
                    "data",
                    [
                        ChannelObject(
                            "data",
                            "orphan",
                            np.array([1.0, 2.0], dtype="float64"),
                        ),
                    ],
                )
            ]
        )

        with pytest.raises(ValueError, match="No timing information"):
            detect_config(path, fallback_method=TdmsFallbackMethod.FAIL_ON_ERROR)

    def test_fallback_ignore_error(self, create_tdms_file):
        """Channels without timing info are silently skipped when fallback is IGNORE_ERROR."""
        path = create_tdms_file(
            groups=[
                (
                    "data",
                    [
                        ChannelObject(
                            "data",
                            "orphan",
                            np.array([1.0, 2.0], dtype="float64"),
                        ),
                    ],
                )
            ]
        )

        config = detect_config(path, fallback_method=TdmsFallbackMethod.IGNORE_ERROR)

        assert len(config.data) == 0
        assert config.fallback_method == TdmsFallbackMethod.IGNORE_ERROR

    def test_multiple_groups(self, create_tdms_file):
        """Channels from multiple groups are all detected with correct group_name."""
        path = create_tdms_file(
            groups=[
                (
                    "group_a",
                    [
                        ChannelObject(
                            "group_a",
                            "ch1",
                            np.array([1.0, 2.0], dtype="float64"),
                            properties={
                                "wf_start_offset": 0.0,
                                "wf_increment": 0.001,
                                "wf_start_time": np.datetime64("2024-01-01T00:00:00"),
                            },
                        ),
                    ],
                ),
                (
                    "group_b",
                    [
                        ChannelObject(
                            "group_b",
                            "ch2",
                            np.array([3, 4], dtype="int32"),
                            properties={
                                "wf_start_offset": 0.0,
                                "wf_increment": 0.001,
                                "wf_start_time": np.datetime64("2024-01-01T00:00:00"),
                            },
                        ),
                    ],
                ),
            ]
        )

        config = detect_config(path)

        assert len(config.data) == 2
        assert config.data[0].group_name == "group_a"
        assert config.data[0].name == "group_a.ch1"
        assert config.data[0].data_type == ChannelDataType.DOUBLE
        assert config.data[1].group_name == "group_b"
        assert config.data[1].name == "group_b.ch2"
        assert config.data[1].data_type == ChannelDataType.INT_32

    def test_enum_channel_detection(self, create_tdms_file):
        """Channels with enum_config property are detected as ENUM type with enum_types populated."""
        import json

        enum_config = json.dumps({"0": "Off", "1": "On", "2": "Error"})
        path = create_tdms_file(
            groups=[
                (
                    "status",
                    [
                        ChannelObject(
                            "status",
                            "state",
                            np.array([0, 1, 2], dtype="uint32"),
                            properties={
                                "wf_start_offset": 0.0,
                                "wf_increment": 1.0,
                                "wf_start_time": np.datetime64("2024-01-01T00:00:00"),
                                "enum_config": enum_config,
                            },
                        ),
                    ],
                )
            ]
        )

        config = detect_config(path)

        assert len(config.data) == 1
        assert config.data[0].data_type == ChannelDataType.ENUM
        assert config.data[0].enum_types == {"Off": 0, "On": 1, "Error": 2}

    def test_asset_name_passthrough(self, create_tdms_file):
        """The asset_name parameter is set on the returned config."""
        path = create_tdms_file(
            groups=[
                (
                    "g",
                    [
                        ChannelObject(
                            "g",
                            "ch",
                            np.array([1.0], dtype="float64"),
                            properties={
                                "wf_start_offset": 0.0,
                                "wf_increment": 0.001,
                                "wf_start_time": np.datetime64("2024-01-01T00:00:00"),
                            },
                        ),
                    ],
                )
            ]
        )

        config = detect_config(path, asset_name="my-asset")

        assert config.asset_name == "my-asset"

    def test_xchannel_property(self, create_tdms_file):
        """Group-level 'xchannel' property overrides time channel detection."""
        path = create_tdms_file(
            groups=[
                (
                    "data",
                    [
                        ChannelObject(
                            "data",
                            "custom_time",
                            np.array([0.0, 1.0, 2.0], dtype="float64"),
                        ),
                        ChannelObject(
                            "data",
                            "value",
                            np.array([10.0, 20.0, 30.0], dtype="float64"),
                        ),
                    ],
                )
            ]
        )

        # nptdms TdmsWriter doesn't support group-level properties directly in segments,
        # so we write the file and then patch the group property by re-reading/writing.
        # Instead, test via the find_time_channel helper.
        from nptdms import TdmsFile

        from sift_client._internal.util.tdms import find_time_channel

        with TdmsFile.open(path) as tdms_file:
            group = tdms_file["data"]
            # Simulate xchannel property
            group.properties["xchannel"] = "custom_time"
            result = find_time_channel(group)

        assert result == "custom_time"
