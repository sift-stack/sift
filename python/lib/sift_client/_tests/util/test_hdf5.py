"""Tests for detect_hdf5_config.

These tests verify that the client-side detect_hdf5_config matches the
backend hdf5.py detect_config 1-to-1. Any client-specific heuristics
(e.g. sibling "timestamps" resolution, 2D dataset handling, "values"
leaf naming) are intentionally NOT present and should not be added.
"""

import h5py
import numpy as np
import pytest

from sift_client._internal.util.hdf5 import detect_hdf5_config
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import TimeFormat


@pytest.fixture
def create_hdf5_file(tmp_path):
    """Return a helper that writes an HDF5 file and returns its path."""
    file_path = tmp_path / "test.h5"

    def _create(populate):
        with h5py.File(file_path, "w") as hdf5_file:
            populate(hdf5_file)
        return file_path

    return _create


class TestDetectHdf5Config:
    def test_compound_dataset(self, create_hdf5_file):
        """Compound type: first field is time, remaining fields become value channels."""
        compound_dtype = np.dtype([("timestamp_ns", "<i8"), ("voltage", "<f8"), ("current", "<f4")])

        def populate(hdf5_file):
            hdf5_file.create_dataset("sensors", shape=(10,), dtype=compound_dtype)

        config = detect_hdf5_config(create_hdf5_file(populate))

        assert len(config.data) == 2
        assert config.data[0].time_field == "timestamp_ns"
        assert config.data[0].value_field == "voltage"
        assert config.data[0].data_type == ChannelDataType.DOUBLE
        assert config.data[0].time_dataset == "sensors"
        assert config.data[0].value_dataset == "sensors"

        assert config.data[1].time_field == "timestamp_ns"
        assert config.data[1].value_field == "current"
        assert config.data[1].data_type == ChannelDataType.FLOAT

    def test_single_column_with_root_time(self, create_hdf5_file):
        """Single-column datasets use root 'time' as time source when present."""

        def populate(hdf5_file):
            hdf5_file.create_dataset("time", data=np.arange(100, dtype="<i8"))
            hdf5_file.create_dataset("voltage", data=np.random.rand(100).astype("<f8"))
            hdf5_file.create_dataset("current", data=np.random.rand(100).astype("<f4"))

        config = detect_hdf5_config(create_hdf5_file(populate))

        assert len(config.data) == 2
        for col in config.data:
            assert col.time_dataset == "time"
            assert col.time_field is None
            assert col.value_field is None

    def test_single_column_without_root_time(self, create_hdf5_file):
        """Without root 'time', time_dataset is empty string."""

        def populate(hdf5_file):
            hdf5_file.create_dataset("voltage", data=np.random.rand(10).astype("<f8"))

        config = detect_hdf5_config(create_hdf5_file(populate))

        assert len(config.data) == 1
        assert config.data[0].time_dataset == ""
        assert config.data[0].name == "voltage"

    def test_root_time_skipped_as_value_channel(self, create_hdf5_file):
        """The root 'time' dataset must not appear as a value channel."""

        def populate(hdf5_file):
            hdf5_file.create_dataset("time", data=np.arange(10, dtype="<i8"))
            hdf5_file.create_dataset("voltage", data=np.random.rand(10).astype("<f8"))

        config = detect_hdf5_config(create_hdf5_file(populate))

        channel_names = [col.name for col in config.data]
        assert "time" not in channel_names
        assert "voltage" in channel_names

    def test_duplicate_name_deduplication(self, create_hdf5_file):
        """Duplicate channel names get a .{dataset_name} suffix."""

        def populate(hdf5_file):
            hdf5_file.create_dataset("time", data=np.arange(10, dtype="<i8"))
            sensor_1 = hdf5_file.create_dataset(
                "group1/sensor", data=np.random.rand(10).astype("<f8")
            )
            sensor_1.attrs["Name"] = "pressure"
            sensor_2 = hdf5_file.create_dataset(
                "group2/sensor", data=np.random.rand(10).astype("<f8")
            )
            sensor_2.attrs["Name"] = "pressure"

        config = detect_hdf5_config(create_hdf5_file(populate))

        channel_names = [col.name for col in config.data]
        assert len(channel_names) == 2
        assert len(set(channel_names)) == 2  # all unique
        assert "pressure" in channel_names

    def test_attribute_detection(self, create_hdf5_file):
        """Channel name, units, and description are read from HDF5 attributes."""

        def populate(hdf5_file):
            hdf5_file.create_dataset("time", data=np.arange(5, dtype="<i8"))
            dataset = hdf5_file.create_dataset("raw_voltage", data=np.random.rand(5).astype("<f8"))
            dataset.attrs["Name"] = "voltage"
            dataset.attrs["Units"] = "V"
            dataset.attrs["Description"] = "Supply voltage"

        config = detect_hdf5_config(create_hdf5_file(populate))

        assert len(config.data) == 1
        assert config.data[0].name == "voltage"
        assert config.data[0].units == "V"
        assert config.data[0].description == "Supply voltage"

    def test_returns_correct_wrapper_type(self, create_hdf5_file):
        """Config wrapper uses correct time format and empty asset_name."""

        def populate(hdf5_file):
            hdf5_file.create_dataset("x", data=np.array([1.0, 2.0]))

        config = detect_hdf5_config(create_hdf5_file(populate))

        assert config.asset_name == ""
        assert config.time_format == TimeFormat.ABSOLUTE_UNIX_NANOSECONDS
