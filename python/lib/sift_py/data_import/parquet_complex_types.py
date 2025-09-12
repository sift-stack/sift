from enum import Enum
from typing import Optional


class ParquetComplexTypesImportModeType(Enum):
    UNSPECIFIED = "PARQUET_COMPLEX_TYPES_IMPORT_MODE_UNSPECIFIED"
    IGNORE = "PARQUET_COMPLEX_TYPES_IMPORT_MODE_IGNORE"
    BOTH = "PARQUET_COMPLEX_TYPES_IMPORT_MODE_BOTH"
    STRING = "PARQUET_COMPLEX_TYPES_IMPORT_MODE_STRING"
    BYTES = "PARQUET_COMPLEX_TYPES_IMPORT_MODE_BYTES"

    @classmethod
    def from_str(cls, val: str) -> Optional["ParquetComplexTypesImportModeType"]:
        try:
            return cls(val)
        except ValueError:
            return None

    def as_human_str(self) -> str:
        return self.value
