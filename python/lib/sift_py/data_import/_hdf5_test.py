from typing import Dict

import h5py  # type: ignore
import numpy as np
import polars as pl
import pytest
from pytest_mock import MockFixture

from sift_py.data_import._config import Hdf5DataCfg
from sift_py.data_import.config import Hdf5Config
from sift_py.data_import.hdf5 import (
    Hdf5UploadService,
    _convert_hdf5_to_dataframes,
    _create_csv_config,
    _extract_hdf5_data_to_dataframe,
    _parse_hdf5_data_cfg,
    _split_hdf5_configs,
)


class MockHdf5File:
    def __init__(self, data_dict: Dict):
        self.data_dict = data_dict

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        pass

    def __getitem__(self, key):
        return MockHdf5Dataset(self.data_dict[key])

    def __contains__(self, key):
        return key in self.data_dict


class MockHdf5Dataset:
    def __init__(self, data):
        self.data = data

    def __getitem__(self, key):
        return self.data[key]


@pytest.fixture
def rest_config():
    return {
        "uri": "some_uri.com",
        "apikey": "123456789",
    }


@pytest.fixture
def hdf5_config():
    return Hdf5Config(
        {
            "asset_name": "TestAsset",
            "time": {
                "format": "TIME_FORMAT_RELATIVE_SECONDS",
                "relative_start_time": "2025-01-01T01:00:00Z",
            },
            "data": [
                {
                    "name": "DoubleChannel",
                    "time_dataset": "/DoubleChannel",
                    "value_dataset": "/DoubleChannel",
                    "time_column": 1,
                    "value_column": 2,
                    "data_type": "CHANNEL_DATA_TYPE_DOUBLE",
                },
                {
                    "name": "DoubleChannelInGroup",
                    "time_dataset": "/testgrp/DoubleChannelInGroup",
                    "value_dataset": "/testgrp/DoubleChannelInGroup",
                    "time_column": 1,
                    "value_column": 2,
                    "data_type": "CHANNEL_DATA_TYPE_DOUBLE",
                },
                {
                    "name": "StringChannel1",
                    "time_dataset": "/StringChannel1",
                    "value_dataset": "/StringChannel1",
                    "time_column": 1,
                    "value_column": 2,
                    "data_type": "CHANNEL_DATA_TYPE_STRING",
                },
                {
                    "name": "BinaryStringChannel2",
                    "time_dataset": "/BinaryStringChannel2",
                    "value_dataset": "/BinaryStringChannel2",
                    "time_column": 1,
                    "value_column": 2,
                    "data_type": "CHANNEL_DATA_TYPE_STRING",
                },
                {
                    "name": "EnumChannel",
                    "time_dataset": "/EnumChannel",
                    "value_dataset": "/EnumChannel",
                    "time_column": 1,
                    "value_column": 2,
                    "data_type": "CHANNEL_DATA_TYPE_ENUM",
                    "enum_types": [
                        {"key": 1, "name": "On"},
                        {"key": 0, "name": "Off"},
                    ],
                },
                {
                    "name": "BitFieldChannel",
                    "time_dataset": "/BitFieldChannel",
                    "value_dataset": "/BitFieldChannel",
                    "time_column": 1,
                    "value_column": 2,
                    "data_type": "CHANNEL_DATA_TYPE_BIT_FIELD",
                    "bit_field_elements": [
                        {"index": 0, "name": "flag1", "bit_count": 4},
                        {"index": 4, "name": "flag2", "bit_count": 4},
                    ],
                },
                {
                    "name": "BoolChannel",
                    "time_dataset": "/BoolChannel",
                    "value_dataset": "/BoolChannel",
                    "time_column": 1,
                    "value_column": 2,
                    "data_type": "CHANNEL_DATA_TYPE_BOOL",
                },
                {
                    "name": "FloatChannel",
                    "time_dataset": "/FloatChannel",
                    "value_dataset": "/FloatChannel",
                    "time_column": 1,
                    "value_column": 2,
                    "data_type": "CHANNEL_DATA_TYPE_FLOAT",
                },
                {
                    "name": "Int32Channel",
                    "time_dataset": "/Int32Channel",
                    "value_dataset": "/Int32Channel",
                    "time_column": 1,
                    "value_column": 2,
                    "data_type": "CHANNEL_DATA_TYPE_INT_32",
                },
                {
                    "name": "Int64Channel",
                    "time_dataset": "/Int64Channel",
                    "value_dataset": "/Int64Channel",
                    "time_column": 1,
                    "value_column": 2,
                    "data_type": "CHANNEL_DATA_TYPE_INT_64",
                },
                {
                    "name": "UInt32Channel",
                    "time_dataset": "/UInt32Channel",
                    "value_dataset": "/UInt32Channel",
                    "time_column": 1,
                    "value_column": 2,
                    "data_type": "CHANNEL_DATA_TYPE_UINT_32",
                },
                {
                    "name": "UInt64Channel",
                    "time_dataset": "/UInt64Channel",
                    "value_dataset": "/UInt64Channel",
                    "time_column": 1,
                    "value_column": 2,
                    "data_type": "CHANNEL_DATA_TYPE_UINT_64",
                },
            ],
        }
    )


@pytest.fixture
def hdf5_data_dict():
    return {
        "/DoubleChannel": np.array(
            list(zip([0, 1, 2], [1.0, 2.0, 3.0])), dtype=[("time", np.int64), ("value", np.float64)]
        ),
        "/testgrp/DoubleChannelInGroup": np.array(
            list(zip([4, 5, 6], [-1.0, -2.0, -3.0])),
            dtype=[("time", np.int64), ("value", np.float64)],
        ),
        "/StringChannel1": np.array(
            list(zip([0, 1, 2], ["a", "b", "c"])),
            dtype=[("time", np.int64), ("value", h5py.string_dtype("utf-8"))],
        ),
        "/BinaryStringChannel2": np.array(
            list(zip([0, 1, 2], [b"a", b"b", b"c"])),
            dtype=[("time", np.int64), ("value", h5py.string_dtype("ascii"))],
        ),
        "/EnumChannel": np.array(
            list(zip([0, 1, 2], [1, 0, 1])), dtype=[("time", np.int64), ("value", np.int32)]
        ),
        "/BitFieldChannel": np.array(
            list(zip([0, 1, 2], [15, 240, 15])), dtype=[("time", np.int64), ("value", np.int32)]
        ),
        "/BoolChannel": np.array(
            list(zip([0, 1, 2], [True, False, True])),
            dtype=[("time", np.int64), ("value", np.bool_)],
        ),
        "/FloatChannel": np.array(
            list(zip([0, 1, 2], [1.1, 2.2, 3.3])), dtype=[("time", np.int64), ("value", np.float32)]
        ),
        "/Int32Channel": np.array(
            list(zip([0, 1, 2], [10, 20, 30])), dtype=[("time", np.int64), ("value", np.int32)]
        ),
        "/Int64Channel": np.array(
            list(zip([0, 1, 2], [10000000000, 20000000000, 30000000000])),
            dtype=[("time", np.int64), ("value", np.int64)],
        ),
        "/UInt32Channel": np.array(
            list(zip([0, 1, 2], [1000, 2000, 3000])),
            dtype=[("time", np.int64), ("value", np.uint32)],
        ),
        "/UInt64Channel": np.array(
            list(zip([0, 1, 2], [1000000000000, 2000000000000, 3000000000000])),
            dtype=[("time", np.int64), ("value", np.uint64)],
        ),
    }


def test_hdf5_upload_service_valid_path(mocker: MockFixture, rest_config, hdf5_config):
    mock_path_is_file = mocker.patch("pathlib.Path.is_file")
    mock_path_is_file.return_value = False

    with pytest.raises(Exception, match="does not point to a regular file"):
        svc = Hdf5UploadService(rest_config)
        svc.upload(path="badpath.h5", hdf5_config=hdf5_config)


def test_split_hdf5_configs_splits_strings(hdf5_config):
    configs = _split_hdf5_configs(hdf5_config)
    # Should split into 1 non-string and 2 string configs (StringChannel1 and StringChannel2)
    string_configs = [
        cfg for cfg in configs if cfg._hdf5_config.data[0].data_type == "CHANNEL_DATA_TYPE_STRING"
    ]
    non_string_configs = [
        cfg for cfg in configs if cfg._hdf5_config.data[0].data_type != "CHANNEL_DATA_TYPE_STRING"
    ]
    assert len(configs) == 3
    assert len(string_configs) == 2
    assert len(non_string_configs) == 1


def test_create_csv_config(hdf5_config):
    csv_cfg = _create_csv_config(hdf5_config)
    csv_cfg_dict = csv_cfg.to_dict()
    assert "time_column" in csv_cfg_dict
    assert "data_columns" in csv_cfg_dict
    assert len(csv_cfg_dict["data_columns"]) == 12


def test_parse_hdf5_data_cfg():
    data_cfg = Hdf5DataCfg(
        name="TestChannel",
        time_dataset="/TestChannel",
        value_dataset="/TestChannel",
        data_type="CHANNEL_DATA_TYPE_DOUBLE",
        units="m/s",
        description="Test channel",
        enum_types=[],
        bit_field_elements=[],
    )
    parsed_cfg = _parse_hdf5_data_cfg(data_cfg)
    assert parsed_cfg["name"] == "TestChannel"
    assert parsed_cfg["data_type"] == "CHANNEL_DATA_TYPE_DOUBLE"
    assert parsed_cfg["units"] == "m/s"
    assert parsed_cfg["description"] == "Test channel"
    assert not parsed_cfg["enum_types"]
    assert not parsed_cfg["bit_field_elements"]


def test_convert_hdf5_to_dataframes(mocker: MockFixture, hdf5_config, hdf5_data_dict):
    mocker.patch("h5py.File", return_value=MockHdf5File(hdf5_data_dict))

    expected_col_count = len(hdf5_data_dict) + 1
    time_stamps = []
    for data in hdf5_data_dict.values():
        for row in data:
            time_stamps.append(row[0])
    expected_row_count = len(set(time_stamps))

    df = _convert_hdf5_to_dataframes("mock.h5", hdf5_config)

    # Dataframe should have cols == parameter count + 1 (timestamps) and rows == unique timestamps
    assert df.shape == (expected_row_count, expected_col_count)


def test_two_dataset_extraction():
    hdf5_config = Hdf5Config(
        {
            "asset_name": "TestAsset",
            "time": {
                "format": "TIME_FORMAT_RELATIVE_SECONDS",
                "relative_start_time": "2025-01-01T01:00:00Z",
            },
            "data": [
                {
                    "name": "Channel1",
                    "time_dataset": "/Channel1_Time",
                    "value_dataset": "/Channel1_Value",
                    "data_type": "CHANNEL_DATA_TYPE_DOUBLE",
                },
            ],
        }
    )

    data_dict = {
        "/Channel1_Time": np.array([0, 1, 2], dtype=np.int64),
        "/Channel1_Value": np.array([1.0, 2.0, 3.0], dtype=np.float64),
    }

    mock_file = MockHdf5File(data_dict)

    for data_cfg in hdf5_config._hdf5_config.data:
        df = _extract_hdf5_data_to_dataframe(mock_file, data_cfg)
        assert df.shape == (3, 2)
        assert df.columns[1] == data_cfg.name
        assert (np.array(df[df.columns[0]]) == data_dict["/Channel1_Time"]).all()
        assert (np.array(df[df.columns[1]]) == data_dict["/Channel1_Value"]).all()


def test_multi_col_dataset_extraction():
    hdf5_config = Hdf5Config(
        {
            "asset_name": "TestAsset",
            "time": {
                "format": "TIME_FORMAT_RELATIVE_SECONDS",
                "relative_start_time": "2025-01-01T01:00:00Z",
            },
            "data": [
                {
                    "name": "Channel1",
                    "time_dataset": "/Channel1",
                    "value_dataset": "/Channel1",
                    "time_column": 4,
                    "value_column": 3,
                    "data_type": "CHANNEL_DATA_TYPE_DOUBLE",
                },
            ],
        }
    )

    data_dict = {
        "/Channel1": [
            np.array([9, 9, 9], dtype=np.int64),
            np.array([9, 9, 9], dtype=np.int64),
            np.array([1.0, 2.0, 3.0], dtype=np.float64),
            np.array([0, 1, 2], dtype=np.int64),
        ],
    }

    mock_file = MockHdf5File(data_dict)

    for data_cfg in hdf5_config._hdf5_config.data:
        df = _extract_hdf5_data_to_dataframe(mock_file, data_cfg)
        assert df.shape == (3, 2)
        assert df.columns[1] == data_cfg.name
        assert (np.array(df[df.columns[0]]) == data_dict["/Channel1"][3]).all()
        assert (np.array(df[df.columns[1]]) == data_dict["/Channel1"][2]).all()


def test_string_conversion():
    hdf5_config = Hdf5Config(
        {
            "asset_name": "TestAsset",
            "time": {
                "format": "TIME_FORMAT_RELATIVE_SECONDS",
                "relative_start_time": "2025-01-01T01:00:00Z",
            },
            "data": [
                {
                    "name": "StringChannel1",
                    "time_dataset": "/StringChannel1",
                    "value_dataset": "/StringChannel1",
                    "time_column": 1,
                    "value_column": 2,
                    "data_type": "CHANNEL_DATA_TYPE_STRING",
                },
                {
                    "name": "BinaryStringChannel2",
                    "time_dataset": "/BinaryStringChannel2",
                    "value_dataset": "/BinaryStringChannel2",
                    "time_column": 1,
                    "value_column": 2,
                    "data_type": "CHANNEL_DATA_TYPE_STRING",
                },
            ],
        }
    )

    data_dict = {
        "/StringChannel1": np.array(
            list(zip([0, 1, 2], ["a", "b", "cat"])),
            dtype=[("time", np.int64), ("value", h5py.string_dtype("utf-8"))],
        ),
        "/BinaryStringChannel2": np.array(
            list(zip([0, 1, 2], [b"a", b"b", b"cat"])),
            dtype=[("time", np.int64), ("value", h5py.string_dtype("ascii"))],
        ),
    }

    mock_file = MockHdf5File(data_dict)

    for data_cfg in hdf5_config._hdf5_config.data:
        df = _extract_hdf5_data_to_dataframe(mock_file, data_cfg)
        assert (np.array(df[data_cfg.name]) == np.array(["a", "b", "cat"])).all()


def test_hdf5_to_dataframe_conversion(mocker: MockFixture, hdf5_config, hdf5_data_dict):
    mocker.patch("h5py.File", return_value=MockHdf5File(hdf5_data_dict))
    name_dataframe_map = {data.name: data.value_dataset for data in hdf5_config._hdf5_config.data}

    df: pl.DataFrame = _convert_hdf5_to_dataframes("mock.h5", hdf5_config)

    for name, value_dataset in name_dataframe_map.items():
        assert name in df.columns

        # Remove nulls since they won't be in original data
        data = df[name].filter(df[name].is_not_null())
        assert len(data) == len(hdf5_data_dict[value_dataset])


def test_bad_time_col(mocker: MockFixture):
    hdf5_config = Hdf5Config(
        {
            "asset_name": "TestAsset",
            "time": {
                "format": "TIME_FORMAT_RELATIVE_SECONDS",
                "relative_start_time": "2025-01-01T01:00:00Z",
            },
            "data": [
                {
                    "name": "DoubleChannel",
                    "time_dataset": "/DoubleChannel",
                    "value_dataset": "/DoubleChannel",
                    "time_column": 2,
                    "value_column": 1,
                    "data_type": "CHANNEL_DATA_TYPE_DOUBLE",
                },
            ],
        }
    )

    data_dict = {
        "/DoubleChannel": np.array([0, 1, 2], dtype=np.int64),
    }

    mocker.patch("h5py.File", return_value=MockHdf5File(data_dict))

    with pytest.raises(Exception, match="time_column=2 out of range"):
        _convert_hdf5_to_dataframes("mock.h5", hdf5_config)


def test_bad_val_col(mocker: MockFixture):
    hdf5_config = Hdf5Config(
        {
            "asset_name": "TestAsset",
            "time": {
                "format": "TIME_FORMAT_RELATIVE_SECONDS",
                "relative_start_time": "2025-01-01T01:00:00Z",
            },
            "data": [
                {
                    "name": "DoubleChannel",
                    "time_dataset": "/DoubleChannel",
                    "value_dataset": "/DoubleChannel",
                    "time_column": 1,
                    "value_column": 2,
                    "data_type": "CHANNEL_DATA_TYPE_DOUBLE",
                },
            ],
        }
    )

    data_dict = {
        "/DoubleChannel": np.array([0, 1, 2], dtype=np.int64),
    }

    mocker.patch("h5py.File", return_value=MockHdf5File(data_dict))

    with pytest.raises(Exception, match="value_column=2 out of range"):
        _convert_hdf5_to_dataframes("mock.h5", hdf5_config)


def test_missing_time_data(mocker: MockFixture):
    hdf5_config = Hdf5Config(
        {
            "asset_name": "TestAsset",
            "time": {
                "format": "TIME_FORMAT_RELATIVE_SECONDS",
                "relative_start_time": "2025-01-01T01:00:00Z",
            },
            "data": [
                {
                    "name": "DoubleChannel",
                    "time_dataset": "/DoubleChannelTime",
                    "value_dataset": "/DoubleChannelValue",
                    "time_column": 1,
                    "value_column": 1,
                    "data_type": "CHANNEL_DATA_TYPE_DOUBLE",
                },
            ],
        }
    )

    data_dict = {
        "/DoubleChannelValue": np.array([0, 1, 2], dtype=np.int64),
    }

    mocker.patch("h5py.File", return_value=MockHdf5File(data_dict))

    with pytest.raises(Exception, match="HDF5 file does not contain dataset"):
        _convert_hdf5_to_dataframes("mock.h5", hdf5_config)


def test_missing_value_data(mocker: MockFixture):
    hdf5_config = Hdf5Config(
        {
            "asset_name": "TestAsset",
            "time": {
                "format": "TIME_FORMAT_RELATIVE_SECONDS",
                "relative_start_time": "2025-01-01T01:00:00Z",
            },
            "data": [
                {
                    "name": "DoubleChannel",
                    "time_dataset": "/DoubleChannelTime",
                    "value_dataset": "/DoubleChannelValue",
                    "time_column": 1,
                    "value_column": 1,
                    "data_type": "CHANNEL_DATA_TYPE_DOUBLE",
                },
            ],
        }
    )

    data_dict = {
        "/DoubleChannelTime": np.array([0, 1, 2], dtype=np.int64),
    }

    mocker.patch("h5py.File", return_value=MockHdf5File(data_dict))

    with pytest.raises(Exception, match="HDF5 file does not contain dataset"):
        _convert_hdf5_to_dataframes("mock.h5", hdf5_config)


def test_hdf5_upload(mocker: MockFixture, hdf5_config, hdf5_data_dict, rest_config):
    mock_path_is_file = mocker.patch("pathlib.Path.is_file")
    mock_path_is_file.return_value = True

    mocker.patch("h5py.File", return_value=MockHdf5File(hdf5_data_dict))

    mock_csv_upload = mocker.patch("sift_py.data_import.csv.CsvUploadService.upload")

    svc = Hdf5UploadService(rest_config)
    import_services = svc.upload(
        "mock.h5",
        hdf5_config,
    )

    mock_csv_upload.assert_called()
    assert len(import_services) == 3


def test_hdf5_upload_string_timestamps(mocker: MockFixture, hdf5_config, rest_config):
    mock_path_is_file = mocker.patch("pathlib.Path.is_file")
    mock_path_is_file.return_value = True

    data_dict = {
        "/timestamps": np.array(
            [
                b"2024-10-07 17:00:09.982126",
                b"2024-10-07 17:00:10.022126",
                b"2024-10-07 17:00:10.062126",
            ]
        ),
        "/DoubleChannel": np.array([0, 1, 2], dtype=np.int64),
    }

    hdf5_config = Hdf5Config(
        {
            "asset_name": "TestAsset",
            "time": {
                "format": "TIME_FORMAT_ABSOLUTE_DATETIME",
            },
            "data": [
                {
                    "name": "DoubleChannel",
                    "time_dataset": "/timestamps",
                    "value_dataset": "/DoubleChannel",
                    "time_column": 1,
                    "value_column": 1,
                    "data_type": "CHANNEL_DATA_TYPE_DOUBLE",
                },
            ],
        }
    )

    mocker.patch("h5py.File", return_value=MockHdf5File(data_dict))

    mock_csv_upload = mocker.patch("sift_py.data_import.csv.CsvUploadService.upload")

    svc = Hdf5UploadService(rest_config)
    svc.upload(
        "mock.h5",
        hdf5_config,
    )

    mock_csv_upload.assert_called()
