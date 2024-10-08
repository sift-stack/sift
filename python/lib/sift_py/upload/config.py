from typing import Dict, List, Optional

from pydantic import BaseModel, ConfigDict, model_validator
from pydantic_core import PydanticCustomError

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


VALID_DATA_TYPES = [
    "CHANNEL_DATA_TYPE_DOUBLE",
    "CHANNEL_DATA_TYPE_FLOAT",
    "CHANNEL_DATA_TYPE_STRING",
    "CHANNEL_DATA_TYPE_BOOL",
    "CHANNEL_DATA_TYPE_INT_32",
    "CHANNEL_DATA_TYPE_INT_64",
    "CHANNEL_DATA_TYPE_UINT_32",
    "CHANNEL_DATA_TYPE_UINT_6",
    "CHANNEL_DATA_TYPE_ENUM",
    "CHANNEL_DATA_TYPE_BIT_FIELD",
]


class _BaseModel(BaseModel):
    model_config = ConfigDict(extra="forbid")


class EnumType(_BaseModel):
    key: int
    name: str


class BitFieldElement(_BaseModel):
    index: int
    name: str
    bit_count: int


class TimeColumn(_BaseModel):
    format: str
    column_number: int
    relative_start_time: Optional[str] = None

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


class DataColumn(_BaseModel):
    name: str
    data_type: str
    component: Optional[str] = ""
    units: Optional[str] = ""
    description: Optional[str] = ""
    enum_types: Optional[List[EnumType]] = []
    bit_field_elements: Optional[List[BitFieldElement]] = []

    @model_validator(mode="after")
    def validate_data_type(self):
        if self.data_type not in VALID_DATA_TYPES:
            raise PydanticCustomError(
                "invalid_config_error",
                f"Invalid data_type: {self.data_type}.\nValid options: {', '.join(VALID_DATA_TYPES)}",
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


class _CsvConfigImpl(_BaseModel):
    """ """

    asset_name: str
    run_name: Optional[str] = None
    run_id: Optional[str] = None
    first_data_row: int
    time_column: TimeColumn
    data_columns: Dict[int, DataColumn]

    @model_validator(mode="after")
    def validate_config(self):
        if not self.data_columns:
            raise PydanticCustomError("invalid_config_error", "Empty 'data_columns'")


class CsvConfig:
    def __init__(self, config_info) -> None:
        self._config_info = config_info
        self._csv_config = _CsvConfigImpl(**self._config_info)

    def to_json(self):
        return self._csv_config.model_dump_json()

    def to_dict(self):
        return self._csv_config.model_dump()
