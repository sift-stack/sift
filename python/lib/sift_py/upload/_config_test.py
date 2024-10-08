import pytest
from sift_py.upload.config import CsvConfig


def test_empty_data_columns():
    with pytest.raises(Exception, match="Empty 'data_columns'"):
        CsvConfig(
            {
                "asset_name": "test_asset",
                "first_data_row": 2,
                "time_column": {
                    "format": "TIME_FORMAT_ABSOLUTE_DATETIME",
                    "column_number": 1,
                },
                "data_columns": {},
            }
        )


def test_data_column_validation():
    with pytest.raises(Exception, match="Invalid data_type:"):
        CsvConfig(
            {
                "asset_name": "test_asset",
                "first_data_row": 2,
                "time_column": {
                    "format": "TIME_FORMAT_ABSOLUTE_DATETIME",
                    "column_number": 1,
                },
                "data_columns": {
                    1: {
                        "name": "channel",
                        "data_type": "INVALID_DATA_TYPE",
                    }
                },
            }
        )


def test_enums():
    with pytest.raises(Exception, match="Enums can only be specified"):
        CsvConfig(
            {
                "asset_name": "test_asset",
                "first_data_row": 2,
                "time_column": {
                    "format": "TIME_FORMAT_ABSOLUTE_DATETIME",
                    "column_number": 1,
                },
                "data_columns": {
                    1: {
                        "name": "channel",
                        "data_type": "CHANNEL_DATA_TYPE_INT_32",
                        "enum_types": [
                            {"key": 1, "name": "value_1"},
                            {"key": 2, "name": "value_2"},
                        ],
                    }
                },
            }
        )

    CsvConfig(
        {
            "asset_name": "test_asset",
            "first_data_row": 2,
            "time_column": {
                "format": "TIME_FORMAT_ABSOLUTE_DATETIME",
                "column_number": 1,
            },
            "data_columns": {
                1: {
                    "name": "channel",
                    "data_type": "CHANNEL_DATA_TYPE_ENUM",
                    "enum_types": [
                        {"key": 1, "name": "value_1"},
                        {"key": 2, "name": "value_2"},
                    ],
                }
            },
        }
    )


def test_bit_field():
    with pytest.raises(Exception, match="Enums can only be specified"):
        CsvConfig(
            {
                "asset_name": "test_asset",
                "first_data_row": 2,
                "time_column": {
                    "format": "TIME_FORMAT_ABSOLUTE_DATETIME",
                    "column_number": 1,
                },
                "data_columns": {
                    1: {
                        "name": "channel",
                        "data_type": "CHANNEL_DATA_TYPE_BIT_FIELD",
                        "enum_types": [
                            {"key": 1, "name": "value_1"},
                            {"key": 2, "name": "value_2"},
                        ],
                    }
                },
            }
        )

    CsvConfig(
        {
            "asset_name": "test_asset",
            "first_data_row": 2,
            "time_column": {
                "format": "TIME_FORMAT_ABSOLUTE_DATETIME",
                "column_number": 1,
            },
            "data_columns": {
                1: {
                    "name": "channel",
                    "data_type": "CHANNEL_DATA_TYPE_BIT_FIELD",
                    "bit_field_elements": [
                        {"index": 1, "name": "bit_field_name_1", "bit_count": 4},
                    ],
                }
            },
        }
    )


def test_time_column():
    with pytest.raises(Exception, match="Invalid time format"):
        CsvConfig(
            {
                "asset_name": "test_asset",
                "first_data_row": 2,
                "time_column": {
                    "format": "INVALID_TIME_FORMAT",
                    "column_number": 1,
                },
                "data_columns": {
                    1: {
                        "name": "channel",
                        "data_type": "CHANNEL_DATA_TYPE_BIT_FIELD",
                    }
                },
            }
        )

    with pytest.raises(Exception, match="Missing 'relative_start_time'"):
        CsvConfig(
            {
                "asset_name": "test_asset",
                "first_data_row": 2,
                "time_column": {
                    "format": "TIME_FORMAT_RELATIVE_SECONDS",
                    "column_number": 1,
                },
                "data_columns": {
                    1: {
                        "name": "channel",
                        "data_type": "CHANNEL_DATA_TYPE_BIT_FIELD",
                    }
                },
            }
        )

    with pytest.raises(
        Exception, match="'relative_start_time' specified for non relative time format."
    ):
        CsvConfig(
            {
                "asset_name": "test_asset",
                "first_data_row": 2,
                "time_column": {
                    "format": "TIME_FORMAT_ABSOLUTE_UNIX_SECONDS",
                    "column_number": 1,
                    "relative_start_time": "100",
                },
                "data_columns": {
                    1: {
                        "name": "channel",
                        "data_type": "CHANNEL_DATA_TYPE_BIT_FIELD",
                    }
                },
            }
        )
