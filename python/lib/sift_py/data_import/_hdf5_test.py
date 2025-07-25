from typing import Dict

import h5py  # type: ignore
import numpy as np
import polars as pl  # type: ignore
import pytest
from pytest_mock import MockFixture

from sift_py.data_import._config import Hdf5DataCfg
from sift_py.data_import.config import Hdf5Config
from sift_py.data_import.hdf5 import (
    Hdf5UploadService,
    _convert_hdf5_to_dataframes,
    _convert_signed_enums,
    _create_csv_config,
    _extract_hdf5_data_to_dataframe,
    _merge_timeseries_dataframes,
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


def test_create_csv_config(mocker: MockFixture, hdf5_config):
    # Use a reverse list to make sure the order has changed
    data_cols = [d_cfg.name for d_cfg in hdf5_config._hdf5_config.data][::-1]
    columns = ["timestamp"] + data_cols
    merged_df = pl.DataFrame({col: [] for col in columns})

    csv_cfg = _create_csv_config(hdf5_config, merged_df)
    csv_cfg_dict = csv_cfg.to_dict()
    assert "time_column" in csv_cfg_dict
    assert "data_columns" in csv_cfg_dict
    assert len(csv_cfg_dict["data_columns"]) == 12

    for csv_col, df_col in zip(csv_cfg_dict["data_columns"].values(), merged_df.columns[1:]):
        assert csv_col["name"] == df_col


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
        df = _extract_hdf5_data_to_dataframe(
            mock_file, data_cfg.time_dataset, data_cfg.time_column, [data_cfg]
        )
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
        df = _extract_hdf5_data_to_dataframe(
            mock_file, data_cfg.time_dataset, data_cfg.time_column, [data_cfg]
        )
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
        df = _extract_hdf5_data_to_dataframe(
            mock_file, data_cfg.time_dataset, data_cfg.time_column, [data_cfg]
        )
        assert (np.array(df[data_cfg.name]) == np.array(["a", "b", "cat"])).all()


def test_bitfield_conversion():
    hdf5_config = Hdf5Config(
        {
            "asset_name": "TestAsset",
            "time": {
                "format": "TIME_FORMAT_RELATIVE_SECONDS",
                "relative_start_time": "2025-01-01T01:00:00Z",
            },
            "data": [
                {
                    "name": "bitfield1",
                    "time_dataset": "/bitChannel1",
                    "value_dataset": "/bitChannel1",
                    "time_column": 1,
                    "value_column": 2,
                    "data_type": "CHANNEL_DATA_TYPE_BIT_FIELD",
                    "bit_field_elements": [
                        {"index": 0, "name": "flag1", "bit_count": 4},
                        {"index": 4, "name": "flag2", "bit_count": 4},
                    ],
                }
            ],
        }
    )

    data_dict = {
        "/bitChannel1": np.array(
            list(zip([0, 1, 2], [0, 2_147_483_647, 15])),
            dtype=[("time", np.int64), ("value", np.int32)],
        ),
    }

    mock_file = MockHdf5File(data_dict)

    for data_cfg in hdf5_config._hdf5_config.data:
        df = _extract_hdf5_data_to_dataframe(
            mock_file, data_cfg.time_dataset, data_cfg.time_column, [data_cfg]
        )
        assert (np.array(df["timestamp"]) == np.array([0, 1, 2])).all()
        assert (np.array(df[data_cfg.name]) == np.array([0, 2_147_483_647, 15])).all()


def test_enum_conversion():
    hdf5_config = Hdf5Config(
        {
            "asset_name": "TestAsset",
            "time": {
                "format": "TIME_FORMAT_RELATIVE_SECONDS",
                "relative_start_time": "2025-01-01T01:00:00Z",
            },
            "data": [
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
                        {"key": 2_147_483_647, "name": "Invalid"},
                    ],
                },
            ],
        }
    )

    data_dict = {
        "/EnumChannel": np.array(
            list(zip([0, 1, 2], [1, 0, 2_147_483_647])),
            dtype=[("time", np.int64), ("value", np.int32)],
        ),
    }

    mock_file = MockHdf5File(data_dict)

    for data_cfg in hdf5_config._hdf5_config.data:
        df = _extract_hdf5_data_to_dataframe(
            mock_file, data_cfg.time_dataset, data_cfg.time_column, [data_cfg]
        )
        assert (np.array(df["timestamp"]) == np.array([0, 1, 2])).all()
        assert (np.array(df[data_cfg.name]) == np.array([1, 0, 2_147_483_647])).all()


def test_time_value_len_diff():
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
                    "time_dataset": "/time",
                    "value_dataset": "/data",
                    "time_column": 1,
                    "value_column": 1,
                    "data_type": "CHANNEL_DATA_TYPE_DOUBLE",
                },
            ],
        }
    )

    data_dict = {
        "/time": np.array([0, 1, 2], dtype=np.int64),
        "/data": np.array([1.0, 2.0, 3.0, 4.0], dtype=np.float64),
    }

    mock_file = MockHdf5File(data_dict)

    for data_cfg in hdf5_config._hdf5_config.data:
        with pytest.raises(Exception, match="time and value columns have different lengths"):
            _extract_hdf5_data_to_dataframe(
                mock_file, data_cfg.time_dataset, data_cfg.time_column, [data_cfg]
            )


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
    svc.upload(
        "mock.h5",
        hdf5_config,
    )

    assert mock_csv_upload.call_count == 3


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


def test_merge_timeseries_dataframes_no_duplicates():
    """Test merging dataframes with no duplicate channels"""
    df1 = pl.DataFrame({"timestamp": [0, 1, 2], "channel1": [1.0, 2.0, 3.0]})
    df2 = pl.DataFrame({"timestamp": [1, 2, 3], "channel2": [4.0, 5.0, 6.0]})

    result = _merge_timeseries_dataframes(df1, df2)

    assert result.shape == (4, 3)
    assert "timestamp" in result.columns
    assert "channel1" in result.columns
    assert "channel2" in result.columns
    result = result.sort("timestamp")
    assert result["timestamp"].to_list() == [0, 1, 2, 3]
    assert result["channel1"].to_list() == [1.0, 2.0, 3.0, None]
    assert result["channel2"].to_list() == [None, 4.0, 5.0, 6.0]


def test_merge_timeseries_dataframes_with_duplicates():
    """Test merging dataframes with duplicate channel names"""
    df1 = pl.DataFrame(
        {"timestamp": [0, 1, 2], "channel1": [1.0, 2.0, 3.0], "common_channel": [10.0, 20.0, 30.0]}
    )
    df2 = pl.DataFrame(
        {"timestamp": [1, 2, 3], "channel2": [4.0, 5.0, 6.0], "common_channel": [40.0, 50.0, 60.0]}
    )

    result = _merge_timeseries_dataframes(df1, df2)

    assert result.shape == (4, 4)
    assert "timestamp" in result.columns
    assert "channel1" in result.columns
    assert "channel2" in result.columns
    assert "common_channel" in result.columns

    result = result.sort("timestamp")

    # Check that values are coalesced properly
    common_values = result["common_channel"].to_list()
    assert common_values == [10.0, 20.0, 30.0, 60.0]


def test_merge_timeseries_dataframes_with_nulls():
    """Test merging dataframes where one has null values"""
    df1 = pl.DataFrame(
        {"timestamp": [0, 1, 2], "channel1": [1.0, None, 3.0], "common_channel": [10.0, None, 30.0]}
    )
    df2 = pl.DataFrame(
        {"timestamp": [1, 2, 3], "channel2": [4.0, 5.0, 6.0], "common_channel": [40.0, 50.0, 60.0]}
    )

    result = _merge_timeseries_dataframes(df1, df2)

    assert result.shape == (4, 4)

    timestamps = result["timestamp"].to_list()
    common_values = result["common_channel"].to_list()

    # At timestamp 1: df1 has null, so should use df2 value (40.0)
    assert common_values[timestamps.index(1)] == 40.0
    # At timestamp 2: df1 has 30.0, so should use df1 value
    assert common_values[timestamps.index(2)] == 30.0


def test_merge_timeseries_dataframes_empty_dataframes():
    """Test merging empty dataframes"""
    df1 = pl.DataFrame({"timestamp": [], "channel1": []})
    df2 = pl.DataFrame({"timestamp": [], "channel2": []})

    result = _merge_timeseries_dataframes(df1, df2)

    assert result.shape == (0, 3)
    assert "timestamp" in result.columns
    assert "channel1" in result.columns
    assert "channel2" in result.columns


def test_merge_timeseries_dataframes_multiple_duplicates():
    """Test merging dataframes with multiple duplicate channel names"""
    df1 = pl.DataFrame(
        {
            "timestamp": [0, 1, 2],
            "channel1": [1.0, 2.0, 3.0],
            "dup1": [10.0, 20.0, 30.0],
            "dup2": [100.0, 200.0, 300.0],
        }
    )
    df2 = pl.DataFrame(
        {
            "timestamp": [1, 2, 3],
            "channel2": [4.0, 5.0, 6.0],
            "dup1": [40.0, 50.0, 60.0],
            "dup2": [400.0, 500.0, 600.0],
        }
    )

    result = _merge_timeseries_dataframes(df1, df2)

    assert result.shape == (4, 5)
    expected_columns = {"timestamp", "channel1", "channel2", "dup1", "dup2"}
    assert set(result.columns) == expected_columns

    # At timestamp 0: should have df1 values only
    assert result.filter(pl.col("timestamp") == 0)["dup1"].item() == 10.0
    assert result.filter(pl.col("timestamp") == 0)["dup2"].item() == 100.0

    # At timestamp 3: should have df2 values only
    assert result.filter(pl.col("timestamp") == 3)["dup1"].item() == 60.0
    assert result.filter(pl.col("timestamp") == 3)["dup2"].item() == 600.0


def test_merge_timeseries_dataframes_different_dtypes():
    """Test merging dataframes with different data types"""
    df1 = pl.DataFrame(
        {"timestamp": [0, 1, 2], "int_channel": [1, 2, 3], "common_channel": [10.0, 20.0, 30.0]}
    )
    df2 = pl.DataFrame(
        {
            "timestamp": [1, 2, 3],
            "string_channel": ["a", "b", "c"],
            "common_channel": [40.0, 50.0, 60.0],
        }
    )

    result = _merge_timeseries_dataframes(df1, df2)

    assert result.shape == (4, 4)
    assert "int_channel" in result.columns
    assert "string_channel" in result.columns
    assert "common_channel" in result.columns
    result = result.sort("timestamp")
    assert result["string_channel"].to_list() == [None, "a", "b", "c"]
    assert result["common_channel"].to_list() == [10.0, 20.0, 30.0, 60.0]


def test_convert_signed_enums():
    data_cfg = Hdf5DataCfg(
        name="TestEnum",
        time_dataset="/time",
        value_dataset="/values",
        data_type="CHANNEL_DATA_TYPE_ENUM",
        enum_types=[
            {"name": "Off", "key": -1, "is_signed": True},
            {"name": "On", "key": 1, "is_signed": True},
        ],
    )

    # Create test data with signed enum values
    test_data = pl.Series("test", [-1, 1, -1])

    result = _convert_signed_enums(data_cfg, test_data)

    # Check that the signed enum key was converted: -1 + 2^32 = 4294967295
    assert data_cfg.enum_types[0].key == 4294967295
    assert data_cfg.enum_types[1].key == 1  # Positive key unchanged

    # Check that the data was converted to uint32
    assert result.dtype == pl.UInt32
    expected_values = np.array([4294967295, 1, 4294967295], dtype=np.uint32)
    assert np.array_equal(result.to_numpy(), expected_values)


def test_convert_signed_enums_no_signed_keys():
    """Test _convert_signed_enums with no signed enum keys"""

    data_cfg = Hdf5DataCfg(
        name="TestEnum",
        time_dataset="/time",
        value_dataset="/values",
        data_type="CHANNEL_DATA_TYPE_ENUM",
        enum_types=[
            {"name": "Off", "key": 0, "is_signed": True},
            {"name": "On", "key": 1, "is_signed": True},
        ],
    )

    test_data = pl.Series("test", [0, 1, 0])

    result = _convert_signed_enums(data_cfg, test_data)

    # Keys should remain unchanged
    assert data_cfg.enum_types[0].key == 0
    assert data_cfg.enum_types[1].key == 1

    assert result.dtype == pl.UInt32
    assert np.array_equal(result.to_numpy(), test_data.to_numpy())


def test_convert_signed_enums_collision_error():
    """Test _convert_signed_enums raises error when conversion would cause collision"""

    # Create a scenario where converting -1 to unsigned (4294967295) would collide
    data_cfg = Hdf5DataCfg(
        name="TestEnum",
        time_dataset="/time",
        value_dataset="/values",
        data_type="CHANNEL_DATA_TYPE_ENUM",
        enum_types=[
            {"name": "Negative", "key": -1, "is_signed": True},
            {"name": "Collision", "key": 4294967295, "is_signed": True},  # This would collide
        ],
    )

    test_data = pl.Series("test", [-1, 4294967295])

    with pytest.raises(
        Exception, match="Converting key -1 to unsigned int collides with existing key 4294967295"
    ):
        _convert_signed_enums(data_cfg, test_data)


def test_convert_signed_enums_multiple_negative_keys():
    """Test _convert_signed_enums with multiple negative signed enum keys"""

    data_cfg = Hdf5DataCfg(
        name="TestEnum",
        time_dataset="/time",
        value_dataset="/values",
        data_type="CHANNEL_DATA_TYPE_ENUM",
        enum_types=[
            {"name": "NegOne", "key": -1, "is_signed": True},
            {"name": "NegTwo", "key": -2, "is_signed": True},
            {"name": "Zero", "key": 0, "is_signed": True},
            {"name": "PosOne", "key": 1, "is_signed": True},
        ],
    )

    test_data = pl.Series("test", [-1, -2, 0, 1])

    result = _convert_signed_enums(data_cfg, test_data)

    # Check conversions: -1 -> 4294967295, -2 -> 4294967294
    assert data_cfg.enum_types[0].key == 4294967295
    assert data_cfg.enum_types[1].key == 4294967294
    assert data_cfg.enum_types[2].key == 0  # Unchanged
    assert data_cfg.enum_types[3].key == 1  # Unchanged

    # Data should be converted to uint32
    assert result.dtype == pl.UInt32
    expected_values = np.array([4294967295, 4294967294, 0, 1])
    assert np.array_equal(result.to_numpy(), expected_values)


def test_convert_signed_enums_edge_case_min_int32():
    """Test _convert_signed_enums with minimum int32 value"""

    min_int32 = -2147483648

    data_cfg = Hdf5DataCfg(
        name="TestEnum",
        time_dataset="/time",
        value_dataset="/values",
        data_type="CHANNEL_DATA_TYPE_ENUM",
        enum_types=[
            {"name": "MinInt32", "key": min_int32, "is_signed": True},
        ],
    )

    test_data = pl.Series("test", [min_int32])

    result = _convert_signed_enums(data_cfg, test_data)

    # min_int32 + 2^32 = -2147483648 + 4294967296 = 2147483648
    expected_unsigned_key = min_int32 + (1 << 32)
    assert data_cfg.enum_types[0].key == expected_unsigned_key

    # Data should be converted to uint32
    assert result.dtype == pl.UInt32
    expected_values = np.array([expected_unsigned_key])
    assert np.array_equal(result.to_numpy(), expected_values)


def test_convert_signed_enums_overflow():
    data_cfg = Hdf5DataCfg(
        name="TestEnum",
        time_dataset="/time",
        value_dataset="/values",
        data_type="CHANNEL_DATA_TYPE_ENUM",
        enum_types=[
            # Min int32 is -2_147_483_648
            {"name": "Off", "key": -2_147_483_649, "is_signed": True},
            {"name": "On", "key": 1, "is_signed": True},
        ],
    )

    # Create test data with signed enum values
    test_data = pl.Series("test", [-2_147_483_649, 1, -2_147_483_649])

    with pytest.raises(Exception, match="below valid int32 range"):
        _convert_signed_enums(data_cfg, test_data)
