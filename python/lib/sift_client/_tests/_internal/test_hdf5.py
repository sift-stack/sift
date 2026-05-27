"""Tests for HDF5 schema detection."""

from __future__ import annotations

import h5py
import numpy as np
import pytest

from sift_client._internal.util.hdf5 import detect_hdf5_config
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import Hdf5Schema


@pytest.fixture
def create_hdf5_file(tmp_path):
    """Return a helper that writes an HDF5 file and returns its path."""
    file_path = tmp_path / "test.h5"

    def _create(populate):
        with h5py.File(file_path, "w") as hdf5_file:
            populate(hdf5_file)
        return file_path

    return _create


def test_one_d_root_time_and_value_datasets(create_hdf5_file):
    """A root '/time' dataset pairs with sibling 1D value datasets."""

    def populate(f):
        f.create_dataset("time", data=np.arange(10, dtype="<i8"))
        f.create_dataset("voltage", data=np.random.rand(10).astype("<f8"))
        f.create_dataset("current", data=np.random.rand(10).astype("<f4"))

    config = detect_hdf5_config(create_hdf5_file(populate), Hdf5Schema.ONE_D)

    assert len(config.data) == 2
    names = {col.name for col in config.data}
    assert names == {"voltage", "current"}
    for col in config.data:
        assert col.time_dataset == "time"
        assert col.value_dataset == col.name
        assert col.time_field is None
        assert col.value_field is None


def test_one_d_ancestor_time_walkup(create_hdf5_file):
    """A value in a sub-group with no own time falls back to an ancestor's time."""

    def populate(f):
        f.create_dataset("time", data=np.arange(10, dtype="<i8"))
        f.create_dataset("group_a/value", data=np.random.rand(10).astype("<f8"))

    config = detect_hdf5_config(create_hdf5_file(populate), Hdf5Schema.ONE_D)

    assert len(config.data) == 1
    assert config.data[0].value_dataset == "group_a/value"
    assert config.data[0].time_dataset == "time"


def test_one_d_per_group_time_overrides_ancestor(create_hdf5_file):
    """A group's own time dataset is preferred over an ancestor's."""

    def populate(f):
        f.create_dataset("time", data=np.arange(10, dtype="<i8"))
        f.create_dataset("group_a/time", data=np.arange(10, dtype="<i8"))
        f.create_dataset("group_a/value", data=np.random.rand(10).astype("<f8"))

    config = detect_hdf5_config(create_hdf5_file(populate), Hdf5Schema.ONE_D)

    value_col = next(c for c in config.data if c.value_dataset == "group_a/value")
    assert value_col.time_dataset == "group_a/time"
    # The root /time still pairs with no values (no siblings besides itself),
    # so its absence from the columns is expected.


def test_one_d_heuristic_time_dataset_names(create_hdf5_file):
    """A dataset literally named 'timestamp' is treated as the group time."""

    def populate(f):
        f.create_dataset("timestamp", data=np.arange(10, dtype="<i8"))
        f.create_dataset("voltage", data=np.random.rand(10).astype("<f8"))

    config = detect_hdf5_config(create_hdf5_file(populate), Hdf5Schema.ONE_D)

    voltage = next(c for c in config.data if c.value_dataset == "voltage")
    assert voltage.time_dataset == "timestamp"


def test_one_d_drops_compound_datasets(create_hdf5_file):
    """A compound dataset alongside 1D datasets is not included."""
    compound_dtype = np.dtype([("ts", "<i8"), ("v", "<f8")])

    def populate(f):
        f.create_dataset("time", data=np.arange(10, dtype="<i8"))
        f.create_dataset("voltage", data=np.random.rand(10).astype("<f8"))
        f.create_dataset("compound_thing", shape=(10,), dtype=compound_dtype)

    config = detect_hdf5_config(create_hdf5_file(populate), Hdf5Schema.ONE_D)

    assert {c.value_dataset for c in config.data} == {"voltage"}


def test_one_d_no_time_anywhere(create_hdf5_file):
    """Without any heuristic time dataset, value columns get empty time_dataset."""

    def populate(f):
        f.create_dataset("voltage", data=np.random.rand(10).astype("<f8"))

    config = detect_hdf5_config(create_hdf5_file(populate), Hdf5Schema.ONE_D)

    assert len(config.data) == 1
    assert config.data[0].time_dataset == ""


def test_two_d_happy_path(create_hdf5_file):
    """[N, 2] datasets become one channel each, col 0 = time, col 1 = value."""

    def populate(f):
        f.create_dataset("sensor_a", data=np.random.rand(10, 2).astype("<f8"))
        f.create_dataset("sensor_b", data=np.random.rand(10, 2).astype("<f4"))

    config = detect_hdf5_config(create_hdf5_file(populate), Hdf5Schema.TWO_D)

    assert len(config.data) == 2
    for col in config.data:
        assert col.time_dataset == col.value_dataset
        assert col.time_index == 0
        assert col.value_index == 1


def test_two_d_drops_non_n_by_2_shapes(create_hdf5_file):
    """1D, wider 2D, and compound datasets are not included."""

    def populate(f):
        f.create_dataset("good", data=np.random.rand(10, 2).astype("<f8"))
        f.create_dataset("one_d", data=np.random.rand(10).astype("<f8"))
        f.create_dataset("wide_2d", data=np.random.rand(10, 3).astype("<f8"))

    config = detect_hdf5_config(create_hdf5_file(populate), Hdf5Schema.TWO_D)

    assert {c.value_dataset for c in config.data} == {"good"}


def test_compound_multi_field(create_hdf5_file):
    """First field is time, remaining fields each become a channel."""
    compound_dtype = np.dtype([("ts", "<i8"), ("voltage", "<f8"), ("current", "<f4")])

    def populate(f):
        f.create_dataset("sensors", shape=(10,), dtype=compound_dtype)

    config = detect_hdf5_config(create_hdf5_file(populate), Hdf5Schema.COMPOUND)

    assert len(config.data) == 2
    voltage = next(c for c in config.data if c.value_field == "voltage")
    current = next(c for c in config.data if c.value_field == "current")
    assert voltage.time_field == "ts"
    assert voltage.data_type == ChannelDataType.DOUBLE
    assert voltage.name == "sensors.voltage"
    assert current.data_type == ChannelDataType.FLOAT
    assert current.name == "sensors.current"


def test_compound_single_value_field_uses_bare_dataset_name(create_hdf5_file):
    """A compound with one value field uses the dataset name verbatim."""
    compound_dtype = np.dtype([("ts", "<i8"), ("voltage", "<f8")])

    def populate(f):
        f.create_dataset("sensor", shape=(10,), dtype=compound_dtype)

    config = detect_hdf5_config(create_hdf5_file(populate), Hdf5Schema.COMPOUND)

    assert len(config.data) == 1
    assert config.data[0].name == "sensor"
    assert config.data[0].value_field == "voltage"


def test_compound_drops_non_compound(create_hdf5_file):
    """1D datasets are not included when the COMPOUND schema is requested."""
    compound_dtype = np.dtype([("ts", "<i8"), ("v", "<f8")])

    def populate(f):
        f.create_dataset("sensor", shape=(10,), dtype=compound_dtype)
        f.create_dataset("voltage", data=np.random.rand(10).astype("<f8"))

    config = detect_hdf5_config(create_hdf5_file(populate), Hdf5Schema.COMPOUND)

    assert {c.value_dataset for c in config.data} == {"sensor"}


def test_attribute_discovery_reads_alternate_names_and_units(create_hdf5_file):
    """Channel name (Title, Sensor, ...) and unit (Units) attributes are honored."""

    def populate(f):
        f.create_dataset("time", data=np.arange(5, dtype="<i8"))
        a = f.create_dataset("raw_a", data=np.random.rand(5).astype("<f8"))
        a.attrs["Title"] = "voltage"
        a.attrs["Units"] = "V"
        a.attrs["Description"] = "Supply voltage"
        b = f.create_dataset("raw_b", data=np.random.rand(5).astype("<f8"))
        b.attrs["Sensor"] = "current_sensor"
        b.attrs["unit"] = "A"

    config = detect_hdf5_config(create_hdf5_file(populate), Hdf5Schema.ONE_D)

    by_dataset = {c.value_dataset: c for c in config.data}
    assert by_dataset["raw_a"].name == "voltage"
    assert by_dataset["raw_a"].units == "V"
    assert by_dataset["raw_a"].description == "Supply voltage"
    assert by_dataset["raw_b"].name == "current_sensor"
    assert by_dataset["raw_b"].units == "A"


def test_list_valued_attribute_uses_first_element(create_hdf5_file):
    """A list-valued Name attribute resolves to its first non-empty element."""

    def populate(f):
        f.create_dataset("time", data=np.arange(5, dtype="<i8"))
        ds = f.create_dataset("raw", data=np.random.rand(5).astype("<f8"))
        ds.attrs.create("Name", data=["voltage", "secondary"], dtype=h5py.string_dtype())

    config = detect_hdf5_config(create_hdf5_file(populate), Hdf5Schema.ONE_D)

    voltage = next(c for c in config.data if c.value_dataset == "raw")
    assert voltage.name == "voltage"


def test_duplicate_names_get_disambiguated(create_hdf5_file):
    """Two datasets resolving to the same channel name get disambiguated."""

    def populate(f):
        f.create_dataset("time", data=np.arange(10, dtype="<i8"))
        a = f.create_dataset("group1/sensor", data=np.random.rand(10).astype("<f8"))
        a.attrs["Name"] = "pressure"
        b = f.create_dataset("group2/sensor", data=np.random.rand(10).astype("<f8"))
        b.attrs["Name"] = "pressure"

    config = detect_hdf5_config(create_hdf5_file(populate), Hdf5Schema.ONE_D)

    names = [c.name for c in config.data]
    assert len(names) == len(set(names))
    assert "pressure" in names
    assert any(n.startswith("pressure.") for n in names)


def test_unsupported_dtype_raises(create_hdf5_file):
    """Unsupported numpy dtypes still raise ValueError rather than silently dropping data."""

    def populate(f):
        f.create_dataset("time", data=np.arange(5, dtype="<i8"))
        f.create_dataset("data", data=np.zeros(5, dtype=np.float16))

    with pytest.raises(ValueError, match="Unsupported numpy dtype"):
        detect_hdf5_config(create_hdf5_file(populate), Hdf5Schema.ONE_D)
