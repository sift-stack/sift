from __future__ import annotations

from typing import Dict, List

from pydantic import BaseModel, ConfigDict, model_validator
from pydantic_core import PydanticCustomError
from sift_py.ingestion.channel import ChannelBitFieldElement, ChannelDataType, ChannelEnumType

VALID_TIME_FORMATS = [
    "TIME_FORMAT_ABSOLUTE_RFC3339",
    "TIME_FORMAT_ABSOLUTE_DATETIME",
    "TIME_FORMAT_ABSOLUTE_UNIX_SECONDS",
    "TIME_FORMAT_ABSOLUTE_UNIX_MILLISECONDS",
    "TIME_FORMAT_ABSOLUTE_UNIX_MICROSECONDS",
    "TIME_FORMAT_ABSOLUTE_UNIX_NANOSECONDS",
    "TIME_FORMAT_RELATIVE_NANOSECONDS",
    "TIME_FORMAT_RELATIVE_MICROSECONDS",
    "TIME_FORMAT_RELATIVE_MILLISECONDS",
    "TIME_FORMAT_RELATIVE_SECONDS",
    "TIME_FORMAT_RELATIVE_MINUTES",
    "TIME_FORMAT_RELATIVE_HOURS",
]


class CsvConfig:
    def __init__(self, config_info) -> None:
        self._config_info = config_info
        self._csv_config = _CsvConfigImpl(**self._config_info)

    def to_json(self):
        return self._csv_config.model_dump_json()

    def to_dict(self):
        return self._csv_config.model_dump()


class _BaseModel(BaseModel):
    model_config = ConfigDict(extra="forbid")


class _CsvConfigImpl(_BaseModel):
    """"""

    asset_name: str
    run_name: str = ""
    run_id: str = ""
    first_data_row: int
    time_column: _TimeColumn
    data_columns: Dict[int, _DataColumn]

    @model_validator(mode="after")
    def validate_config(self):
        if not self.data_columns:
            raise PydanticCustomError("invalid_config_error", "Empty 'data_columns'")


class _EnumType(_BaseModel, ChannelEnumType):
    pass


class _BitFieldElement(_BaseModel, ChannelBitFieldElement):
    pass


class _TimeColumn(_BaseModel):
    format: str
    column_number: int
    relative_start_time: str = None

    @model_validator(mode="after")
    def validate_format(self):
        if self.format not in VALID_TIME_FORMATS:
            raise PydanticCustomError(
                "invalid_config_error",
                "Invalid time format: {format}.\nValid options: {valid}",
                {"format": self.format, "valid": ", ".join(VALID_TIME_FORMATS)},
            )

        return self

    @model_validator(mode="after")
    def validate_relative_time(self):
        if self.format.startswith("TIME_FORMAT_RELATIVE_"):
            if self.relative_start_time is None:
                raise PydanticCustomError("invalid_config_error", "Missing 'relative_start_time'")
        else:
            if self.relative_start_time:
                raise PydanticCustomError(
                    "invalid_config_error",
                    "'relative_start_time' specified for non relative time format.",
                )

        return self


class _DataColumn(_BaseModel):
    name: str
    data_type: str
    component: str = ""
    units: str = ""
    description: str = ""
    enum_types: List[_EnumType] = []
    bit_field_elements: List[_BitFieldElement] = []

    @model_validator(mode="after")
    def validate_data_type(self):
        if ChannelDataType.from_str(self.data_type) is None:
            raise PydanticCustomError(
                "invalid_config_error", f"Invalid data_type: {self.data_type}."
            )

        return self

    @model_validator(mode="after")
    def validate_enums(self):
        if self.enum_types:
            if self.data_type != "CHANNEL_DATA_TYPE_ENUM":
                raise PydanticCustomError(
                    "invalid_config_error",
                    f"Enums can only be specified with the CHANNEL_DATA_TYPE_ENUM data type. {self.name} is {self.data_type}",
                )

        return self

    @model_validator(mode="after")
    def validate_bit_fields(self):
        if self.bit_field_elements:
            if self.data_type != "CHANNEL_DATA_TYPE_BIT_FIELD":
                raise PydanticCustomError(
                    "invalid_config_error",
                    f"Bit fields can only be specified with the CHANNEL_DATA_TYPE_BIT_FIELD data type. {self.name} is {self.data_type}",
                )

        return self
