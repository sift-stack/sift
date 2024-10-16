import pytest

from sift_py.data_import.config import CsvConfig
from sift_py.data_import.time_format import TimeFormatType
from sift_py.ingestion.channel import ChannelDataType


@pytest.fixture
def csv_config_data():
    return {
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
            }
        },
    }


def test_empty_data_columns(csv_config_data: dict):
    csv_config_data["data_columns"] = {}
    with pytest.raises(Exception, match="Empty 'data_columns'"):
        CsvConfig(csv_config_data)


def test_data_column_validation(csv_config_data: dict):
    csv_config_data["data_columns"] = {
        1: {
            "name": "channel",
            "data_type": "INVALID_DATA_TYPE",
        }
    }
    with pytest.raises(Exception, match="Invalid data_type:"):
        CsvConfig(csv_config_data)

    csv_config_data["data_columns"] = {1: {"name": "channel", "data_type": complex}}
    with pytest.raises(Exception, match="Invalid data_type:"):
        CsvConfig(csv_config_data)

    csv_config_data["data_columns"] = {
        1: {"name": "channel_bool", "data_type": ChannelDataType.BOOL},
        2: {"name": "channel_double", "data_type": ChannelDataType.DOUBLE},
        3: {"name": "channel_int", "data_type": ChannelDataType.INT_64},
        4: {"name": "channel_str", "data_type": ChannelDataType.STRING},
    }
    CsvConfig(csv_config_data)


def test_enums(csv_config_data: dict):
    csv_config_data["data_columns"] = {
        1: {
            "name": "channel",
            "data_type": "CHANNEL_DATA_TYPE_INT_32",
            "enum_types": [
                {"key": 1, "name": "value_1"},
                {"key": 2, "name": "value_2"},
            ],
        }
    }
    with pytest.raises(Exception, match="Enums can only be specified"):
        CsvConfig(csv_config_data)

    csv_config_data["data_columns"] = {
        1: {
            "name": "channel",
            "data_type": "CHANNEL_DATA_TYPE_ENUM",
            "enum_types": [
                {"key": 1, "name": "value_1", "extra_key": "value"},
                {"key": 2, "name": "value_2"},
            ],
        }
    }
    with pytest.raises(Exception, match="validation error"):
        CsvConfig(csv_config_data)

    csv_config_data["data_columns"] = {
        1: {
            "name": "channel",
            "data_type": "CHANNEL_DATA_TYPE_ENUM",
            "enum_types": [
                {"key": 1, "name": "value_1"},
                {"key": 2, "name": "value_2"},
            ],
        }
    }
    CsvConfig(csv_config_data)


def test_bit_field(csv_config_data: dict):
    csv_config_data["data_columns"] = {
        1: {
            "name": "channel",
            "data_type": "CHANNEL_DATA_TYPE_INT_32",
            "bit_field_elements": [
                {"index": 1, "name": "bit_field_name_1", "bit_count": 4},
            ],
        }
    }
    with pytest.raises(Exception, match="Bit fields can only be specified"):
        CsvConfig(csv_config_data)

    csv_config_data["data_columns"] = {
        1: {
            "name": "channel",
            "data_type": "CHANNEL_DATA_TYPE_INT_32",
            "bit_field_elements": [
                {
                    "index": 1,
                    "name": "bit_field_name_1",
                    "bit_count": 4,
                    "extra_key": "value",
                },
            ],
        }
    }
    with pytest.raises(Exception, match="validation error"):
        CsvConfig(csv_config_data)

    csv_config_data["data_columns"] = {
        1: {
            "name": "channel",
            "data_type": "CHANNEL_DATA_TYPE_BIT_FIELD",
            "bit_field_elements": [
                {"index": 1, "name": "bit_field_name_1", "bit_count": 4},
            ],
        }
    }
    CsvConfig(csv_config_data)


def test_time_column(csv_config_data: dict):
    csv_config_data["time_column"] = {
        "format": "INVALID_TIME_FORMAT",
        "column_number": 1,
    }
    with pytest.raises(Exception, match="Invalid time format"):
        CsvConfig(csv_config_data)

    csv_config_data["time_column"] = {
        "format": "TIME_FORMAT_RELATIVE_SECONDS",
        "column_number": 1,
    }
    with pytest.raises(Exception, match="Missing 'relative_start_time'"):
        CsvConfig(csv_config_data)

    csv_config_data["time_column"] = {
        "format": "TIME_FORMAT_ABSOLUTE_UNIX_SECONDS",
        "column_number": 1,
        "relative_start_time": "100",
    }
    with pytest.raises(
        Exception, match="'relative_start_time' specified for non relative time format."
    ):
        CsvConfig(csv_config_data)

    csv_config_data["time_column"] = {
        "format": TimeFormatType.ABSOLUTE_DATETIME,
        "column_number": 1,
    }
    CsvConfig(csv_config_data)
