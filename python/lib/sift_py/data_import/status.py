import time
from datetime import datetime
from enum import Enum
from typing import Optional, Union
from urllib.parse import urljoin

from pydantic import BaseModel, ConfigDict, field_validator
from pydantic.alias_generators import to_camel
from pydantic_core import PydanticCustomError
from typing_extensions import Self

from sift_py.rest import SiftRestConfig, _RestService


class DataImportStatusType(Enum):
    """Status of the data import."""

    SUCCEEDED = "DATA_IMPORT_STATUS_SUCCEEDED"
    PENDING = "DATA_IMPORT_STATUS_PENDING"
    IN_PROGRESS = "DATA_IMPORT_STATUS_IN_PROGRESS"
    FAILED = "DATA_IMPORT_STATUS_FAILED"

    @classmethod
    def from_str(cls, val: str) -> Optional[Self]:
        try:
            return cls(val)
        except ValueError:
            return None

    def as_human_str(self) -> str:
        return self.value


class DataImport(BaseModel):
    """Metadata regarding the data import."""

    model_config = ConfigDict(alias_generator=to_camel, populate_by_name=True)

    data_import_id: str
    created_date: datetime
    modified_date: datetime
    source_url: str = ""
    status: Union[str, DataImportStatusType]
    error_message: str = ""
    csv_config: dict

    @field_validator("status", mode="before")
    @classmethod
    def convert_status(cls, raw: Union[str, DataImportStatusType]) -> DataImportStatusType:
        if isinstance(raw, DataImportStatusType):
            return raw
        elif isinstance(raw, str):
            value = DataImportStatusType.from_str(raw)
            if value is not None:
                return value

        raise PydanticCustomError(
            "invalid_data_import_error", f"Invalid data import status: {raw}."
        )


class DataImportService(_RestService):
    """
    Service used to retrieve information about a particular data import.
    """

    STATUS_PATH = "/api/v1/data-imports"
    _data_import_id: str

    # TODO: rename restconf to rest_conf for consistency between services
    def __init__(self, restconf: SiftRestConfig, data_import_id: str):
        super().__init__(rest_conf=restconf)
        self._data_import_id = data_import_id
        self._status_uri = urljoin(self._base_uri, self.STATUS_PATH)

    def get_data_import(self) -> DataImport:
        """
        Returns information about the data import.
        """
        response = self._session.get(
            url=f"{self._status_uri}/{self._data_import_id}",
        )
        response.raise_for_status()
        data = response.json().get("dataImport")
        data_import = DataImport(**data)
        return data_import

    def wait_until_complete(self) -> DataImport:
        """
        Blocks until the data import is completed. Check the status to determine
        if the import was successful or not.
        """
        polling_interval = 1
        while True:
            data_import = self.get_data_import()
            status: DataImportStatusType = data_import.status  # type: ignore
            if status in [
                DataImportStatusType.SUCCEEDED,
                DataImportStatusType.FAILED,
            ]:
                return data_import
            elif status in [
                DataImportStatusType.PENDING,
                DataImportStatusType.IN_PROGRESS,
            ]:
                pass
            else:
                raise Exception(f"Unknown status: {status}")
            time.sleep(polling_interval)
            polling_interval = min(polling_interval * 2, 60)
