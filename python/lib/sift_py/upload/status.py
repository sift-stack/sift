import re
import time
from urllib.parse import urljoin

import requests
from sift_py.rest import SiftRestConfig


class DataImportStatus:
    STATUS_PATH = "/api/v1/data-imports"
    _data_import_id: str

    def __init__(self, restconf: SiftRestConfig, data_import_id: str):
        base_uri = self.__class__._compute_uri(restconf)
        self._data_import_id = data_import_id
        self._status_uri = urljoin(base_uri, self.STATUS_PATH)
        self._apikey = restconf["apikey"]

    def get_status(self):
        response = requests.get(
            url=f"{self._status_uri}/{self._data_import_id}",
            headers={"Authorization": f"Bearer {self._apikey}"},
        )
        response.raise_for_status()
        return response.json().get("dataImport").get("status")

    def wait(self, verbose: bool = False) -> bool:
        polling_interval = 1
        while True:
            status = self.get_status()
            if status == "DATA_IMPORT_STATUS_SUCCEEDED":
                if verbose:
                    print("Upload completed!")
                return True
            elif status == "DATA_IMPORT_STATUS_PENDING":
                if verbose:
                    print("Upload pending...")
            elif status == "DATA_IMPORT_STATUS_IN_PROGRESS":
                if verbose:
                    print("Upload in progress...")
            elif status == "DATA_IMPORT_STATUS_FAILED":
                if verbose:
                    print("Upload failed")
                return False
            else:
                raise Exception(f"Unknown status: {status}")
            time.sleep(polling_interval)
            polling_interval = min(polling_interval * 2, 60)

    @staticmethod
    def _compute_uri(restconf: SiftRestConfig) -> str:
        uri = restconf["uri"]

        scheme_match = re.match(r"(.+://).+", uri)
        if scheme_match:
            raise Exception(f"The URL scheme '{scheme_match.groups()[0]}' should not be included")

        if restconf.get("use_ssl", True):
            return f"https://{uri}"

        return f"http://{uri}"
