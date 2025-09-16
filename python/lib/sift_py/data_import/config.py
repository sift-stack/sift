from typing import Any, Dict

from sift_py.data_import._config import CsvConfigImpl, Hdf5ConfigImpl, ParquetConfigImpl


class CsvConfig:
    """
    Defines the CSV config for data imports.
    """

    def __init__(self, config_info: Dict[str, Any]):
        self._config_info = config_info
        self._csv_config = CsvConfigImpl(**self._config_info)

    def to_json(self) -> str:
        return self._csv_config.model_dump_json()

    def to_dict(self) -> Dict[str, Any]:
        return self._csv_config.model_dump()


class Hdf5Config:
    """
    Defines the HDF5 config for data imports.
    """

    def __init__(self, config_info: Dict[str, Any]):
        self._config_info = config_info
        self._hdf5_config = Hdf5ConfigImpl(**self._config_info)

    def to_json(self) -> str:
        return self._hdf5_config.model_dump_json()

    def to_dict(self) -> Dict[str, Any]:
        return self._hdf5_config.model_dump()


class ParquetConfig:
    """
    Defines the Parquet config for data imports.
    """

    def __init__(self, config_info: Dict[str, Any]):
        self._config_info = config_info
        self._parquet_config = ParquetConfigImpl(**self._config_info)

    def to_json(self) -> str:
        return self._parquet_config.model_dump_json()

    def to_dict(self) -> Dict[str, Any]:
        return self._parquet_config.model_dump()
