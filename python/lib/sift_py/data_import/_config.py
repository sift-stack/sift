from __future__ import annotations

from typing import Any, Dict, List, Optional, Type, Union

from pydantic import BaseModel, ConfigDict, field_validator, model_validator
from pydantic_core import PydanticCustomError
from typing_extensions import Self

from sift_py._internal.channel import channel_fqn
from sift_py.data_import.parquet_complex_types import ParquetComplexTypesImportModeType
from sift_py.data_import.time_format import TimeFormatType
from sift_py.error import _component_deprecation_warning
from sift_py.ingestion.channel import ChannelBitFieldElement, ChannelDataType, ChannelEnumType


class ConfigBaseModel(BaseModel):
    """
    Specialized BaseMode that forbids extra fields.
    """

    model_config = ConfigDict(extra="forbid")


class ConfigDataModel(ConfigBaseModel):
    """
    Base DataModel with common functionality
    """

    name: str
    data_type: Union[str, ChannelDataType, Type]
    units: str = ""
    description: str = ""
    # Only valid if data_type is "CHANNEL_DATA_TYPE_ENUM".
    enum_types: List[EnumType] = []
    # Only valid if data_type is "CHANNEL_DATA_TYPE_BIT_FIELD"
    bit_field_elements: List[BitFieldElement] = []

    @field_validator("data_type", mode="before")
    @classmethod
    def convert_data_type(cls, raw: Union[str, ChannelDataType, Type]) -> str:
        """
        Converts the provided data_type value to a string.
        """
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

    @model_validator(mode="before")
    @classmethod
    def concatenate_component_and_name(cls, data: Any) -> Any:
        """
        Concatenates Component and Name. If Component is not an empty string, raises a deprecation warning.
        """
        if isinstance(data, dict):
            if "component" in data.keys() and "name" in data.keys():
                _component_deprecation_warning()
                data["name"] = channel_fqn(name=data["name"], component=data["component"])
                data.pop("component")
        return data

    @model_validator(mode="after")
    def validate_enums(self) -> Self:
        """
        Validates the enum configuration.
        """
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
        """
        Validates the bit field configuration.
        """
        data_type = ChannelDataType.from_str(self.data_type)  # type: ignore
        if self.bit_field_elements:
            if data_type != ChannelDataType.BIT_FIELD:
                raise PydanticCustomError(
                    "invalid_config_error",
                    f"Bit fields can only be specified with the CHANNEL_DATA_TYPE_BIT_FIELD data type. {self.name} is {self.data_type}",
                )

        return self


class ConfigTimeModel(ConfigBaseModel):
    """
    Base TimeModel with common functionality
    """

    format: Union[str, TimeFormatType]
    relative_start_time: Optional[str] = None

    @field_validator("format", mode="before")
    @classmethod
    def convert_format(cls, raw: Union[str, TimeFormatType]) -> str:
        """
        Converts the provided format value to a string.
        """
        if isinstance(raw, TimeFormatType):
            return raw.as_human_str()
        elif isinstance(raw, str):
            value = TimeFormatType.from_str(raw)
            if value is not None:
                return value.as_human_str()

        raise PydanticCustomError("invalid_config_error", f"Invalid time format: {raw}.")

    @model_validator(mode="after")
    def validate_time(self) -> Self:
        """
        Validates the provided time format.
        """
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


class CsvConfigImpl(ConfigBaseModel):
    """
    Defines the CSV config spec.
    """

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

        if self.run_name and self.run_id:
            raise PydanticCustomError(
                "invalid_config_error", "Only specify run_name or run_id, not both."
            )

        return self


class Hdf5ConfigImpl(ConfigBaseModel):
    """
    Defines the HDF5 config spec
    """

    asset_name: str
    run_name: str = ""
    run_id: str = ""
    time: TimeCfg
    data: List[Hdf5DataCfg]

    @model_validator(mode="after")
    def validate_config(self) -> Self:
        if not self.data:
            raise PydanticCustomError("invalid_config_error", "Empty 'data'")

        if self.run_name and self.run_id:
            raise PydanticCustomError(
                "invalid_config_error", "Only specify run_name or run_id, not both."
            )

        return self


class EnumType(ConfigBaseModel, ChannelEnumType):
    """
    Defines an enum entry in the CSV config.
    """


class BitFieldElement(ConfigBaseModel, ChannelBitFieldElement):
    """
    Defines a bit field element entry in the CSV config.
    """


class TimeColumn(ConfigTimeModel):
    """
    Defines a time column entry in the CSV config.
    """

    column_number: int


class DataColumn(ConfigDataModel):
    """
    Defines a data column entry in the CSV config.
    """


class TimeCfg(ConfigTimeModel):
    """
    Defines a time entry in the generic file config.
    """


class Hdf5DataCfg(ConfigDataModel):
    """
    Defines a data entry in the HDF5 config.
    """

    time_dataset: str
    time_column: int = 1
    value_dataset: str
    value_column: int = 1


class ParquetTimeColumn(ConfigTimeModel):
    """
    Defines a time column entry in the Parquet config.
    """

    path: str


class ParquetDataColumn(ConfigBaseModel):
    """
    Defines a data column entry in the Parquet config.
    """

    path: str
    channel_config: ConfigDataModel


class ParquetFlatDatasetConfig(ConfigBaseModel):
    """
    Defines the flat dataset config for Parquet files.
    """

    time_column: ParquetTimeColumn
    data_columns: List[ParquetDataColumn]


class ParquetConfigImpl(ConfigBaseModel):
    """
    Defines the Parquet config spec.
    """

    asset_name: str
    run_name: str = ""
    run_id: str = ""
    flat_dataset: Optional[ParquetFlatDatasetConfig] = None
    footer_offset: int
    footer_length: int
    complex_types_import_mode: Union[str, ParquetComplexTypesImportModeType]

    @model_validator(mode="after")
    def validate_config(self) -> Self:
        if self.run_name and self.run_id:
            raise PydanticCustomError(
                "invalid_config_error", "Only specify run_name or run_id, not both."
            )
        return self

    @field_validator("complex_types_import_mode", mode="before")
    @classmethod
    def convert_complex_types_import_mode(cls, raw: Optional[str]) -> Optional[str]:
        """
        Converts the provided complex_types_import_mode value to a string.
        """
        if raw is None:
            return None
        if isinstance(raw, ParquetComplexTypesImportModeType):
            return raw.as_human_str()
        elif isinstance(raw, str):
            value = ParquetComplexTypesImportModeType.from_str(raw)
            if value is not None:
                return value.as_human_str()
        raise PydanticCustomError(
            "invalid_config_error", f"Invalid complex_types_import_mode: {raw}."
        )
