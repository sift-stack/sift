from __future__ import annotations

from typing import Dict, List, Optional, Type, Union

from pydantic import BaseModel, ConfigDict, field_validator, model_validator
from pydantic_core import PydanticCustomError
from sift_py.data_import.time_format import TimeFormatType
from sift_py.ingestion.channel import ChannelBitFieldElement, ChannelDataType, ChannelEnumType
from typing_extensions import Self


class ConfigBaseModel(BaseModel):
    model_config = ConfigDict(extra="forbid")


class CsvConfigImpl(ConfigBaseModel):
    """"""

    asset_name: str
    run_name: str = ""
    run_id: str = ""
    first_data_row: int
    time_column: TimeColumn
    data_columns: Dict[int, DataColumn]

    @model_validator(mode="after")
    def validate_config(self) -> Self:
        if not self.data_columns:
            raise PydanticCustomError("invalid_config_error", "Empty 'data_columns'")
        return self


class EnumType(ConfigBaseModel, ChannelEnumType):
    pass


class BitFieldElement(ConfigBaseModel, ChannelBitFieldElement):
    pass


class TimeColumn(ConfigBaseModel):
    format: Union[str, TimeFormatType]
    column_number: int
    relative_start_time: Optional[str] = None

    @field_validator("format", mode="before")
    @classmethod
    def convert_format(cls, raw: Union[str, TimeFormatType]) -> str:
        if isinstance(raw, TimeFormatType):
            return raw.as_human_str()
        elif isinstance(raw, str):
            value = TimeFormatType.from_str(raw)
            if value is not None:
                return value.as_human_str()

        raise PydanticCustomError("invalid_config_error", f"Invalid time format: {raw}.")

    @model_validator(mode="after")
    def validate_time(self) -> Self:
        format = TimeFormatType.from_str(self.format)  # type: ignore
        if format is None:
            raise PydanticCustomError(
                "invalid_config_error", f"Invalid time format: {self.format}."
            )

        if format.is_relative():
            if self.relative_start_time is None:
                raise PydanticCustomError("invalid_config_error", "Missing 'relative_start_time'")
        else:
            if self.relative_start_time is not None:
                raise PydanticCustomError(
                    "invalid_config_error",
                    "'relative_start_time' specified for non relative time format.",
                )

        return self


class DataColumn(ConfigBaseModel):
    name: str
    data_type: Union[str, ChannelDataType, Type]
    component: str = ""
    units: str = ""
    description: str = ""
    enum_types: List[EnumType] = []
    bit_field_elements: List[BitFieldElement] = []

    @field_validator("data_type", mode="before")
    @classmethod
    def convert_data_type(cls, raw: Union[str, ChannelDataType, Type]) -> str:
        if isinstance(raw, type):
            if raw == int:
                return ChannelDataType.INT_64.as_human_str(api_format=True)
            elif raw == float:
                return ChannelDataType.DOUBLE.as_human_str(api_format=True)
            elif raw == str:
                return ChannelDataType.STRING.as_human_str(api_format=True)
            elif raw == bool:
                return ChannelDataType.BOOL.as_human_str(api_format=True)
        elif isinstance(raw, ChannelDataType):
            return raw.as_human_str(api_format=True)
        elif isinstance(raw, str):
            value = ChannelDataType.from_str(raw)
            if value is not None:
                return value.as_human_str(api_format=True)

        raise PydanticCustomError("invalid_config_error", f"Invalid data_type: {raw}.")

    @model_validator(mode="after")
    def validate_enums(self) -> Self:
        data_type = ChannelDataType.from_str(self.data_type)  # type: ignore
        if self.enum_types:
            if data_type != ChannelDataType.ENUM:
                raise PydanticCustomError(
                    "invalid_config_error",
                    f"Enums can only be specified with the CHANNEL_DATA_TYPE_ENUM data type. {self.name} is {self.data_type}",
                )

        return self

    @model_validator(mode="after")
    def validate_bit_fields(self) -> Self:
        data_type = ChannelDataType.from_str(self.data_type)  # type: ignore
        if self.bit_field_elements:
            if data_type != ChannelDataType.BIT_FIELD:
                raise PydanticCustomError(
                    "invalid_config_error",
                    f"Bit fields can only be specified with the CHANNEL_DATA_TYPE_BIT_FIELD data type. {self.name} is {self.data_type}",
                )

        return self
