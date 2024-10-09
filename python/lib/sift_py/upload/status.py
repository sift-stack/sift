import time
from enum import Enum
from urllib.parse import urljoin

import requests
from sift_py.rest import SiftRestConfig, compute_uri


class DataImportStatusValue(Enum):
    SUCCEEDED = "DATA_IMPORT_STATUS_SUCCEEDED"
    PENDING = "DATA_IMPORT_STATUS_PENDING"
    IN_PROGRESS = "DATA_IMPORT_STATUS_IN_PROGRESS"
    FAILED = "DATA_IMPORT_STATUS_FAILED"

    @classmethod
    def from_str(cls, val: str):
        if val == cls.SUCCEEDED.value:
            return cls.SUCCEEDED
        elif val == cls.PENDING.value:
            return cls.PENDING
        elif val == cls.IN_PROGRESS.value:
            return cls.IN_PROGRESS
        elif val == cls.FAILED.value:
            return cls.FAILED
        else:
            raise ValueError("Argument 'val' is not a valid status.")


class DataImportStatus:
    STATUS_PATH = "/api/v1/data-imports"
    _data_import_id: str

    def __init__(self, restconf: SiftRestConfig, data_import_id: str):
        base_uri = compute_uri(restconf)
        self._data_import_id = data_import_id
        self._status_uri = urljoin(base_uri, self.STATUS_PATH)
        self._apikey = restconf["apikey"]

    def get_status(self) -> DataImportStatusValue:
        response = requests.get(
            url=f"{self._status_uri}/{self._data_import_id}",
            headers={"Authorization": f"Bearer {self._apikey}"},
        )
        response.raise_for_status()

        status = response.json().get("dataImport").get("status")
        return DataImportStatusValue.from_str(status)

    def wait_until_complete(self) -> bool:
        polling_interval = 1
        while True:
            status: DataImportStatusValue = self.get_status()
            if status == DataImportStatusValue.SUCCEEDED:
                return True
            elif status == DataImportStatusValue.PENDING:
                pass
            elif status == DataImportStatusValue.IN_PROGRESS:
                pass
            elif status == DataImportStatusValue.FAILED:
                return False
            else:
                raise Exception(f"Unknown status: {status}")
            time.sleep(polling_interval)
            polling_interval = min(polling_interval * 2, 60)
