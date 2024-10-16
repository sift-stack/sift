from typing import Any, Dict

from sift_py.data_import._config import CsvConfigImpl


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
