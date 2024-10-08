import json
import mimetypes
import re
from pathlib import Path
from typing import Optional, Tuple, Union
from urllib.parse import urljoin, urlparse

import requests
from sift_py.rest import SiftRestConfig
from sift_py.upload.config import CsvConfig
from sift_py.upload.status import DataImportStatus


class CsvUploadService:
    UPLOAD_PATH = "/api/v1/data-imports:upload"
    URL_PATH = "/api/v1/data-imports:url"

    _rest_conf: SiftRestConfig
    _upload_uri: str
    _url_uri: str
    _apikey: str

    def __init__(self, rest_conf: SiftRestConfig):
        self._rest_conf = rest_conf
        self._apikey = rest_conf["apikey"]
        base_uri = self.__class__._compute_uri(rest_conf)
        self._upload_uri = urljoin(base_uri, self.UPLOAD_PATH)
        self._url_uri = urljoin(base_uri, self.URL_PATH)

    def upload(
        self,
        path: Union[str, Path],
        csv_config: CsvConfig,
    ) -> DataImportStatus:
        posix_path = Path(path) if isinstance(path, str) else path

        if not posix_path.is_file():
            raise Exception(f"Provided path, '{path}', does not point to a regular file.")

        _, mimetype, content_encoding = self.__class__._mime_and_content_type_from_path(posix_path)

        if not mimetype:
            raise Exception(f"The MIME-type of '{posix_path}' could not be computed.")

        response = requests.post(
            url=self._upload_uri,
            headers={
                "Authorization": f"Bearer {self._apikey}",
                "Content-Encoding": "application/octet-stream",
            },
            data=json.dumps({"csv_config": csv_config.to_dict()}),
        )

        if response.status_code != 200:
            raise Exception(
                f"Config file upload request failed with status code {response.status_code}. {response.text}"
            )

        try:
            upload_info = response.json()
        except (json.decoder.JSONDecodeError, KeyError) as e:
            raise Exception(f"Invalid response: {e}")

        try:
            upload_url: str = upload_info["uploadUrl"]
            data_import_id: str = upload_info["dataImportId"]
        except KeyError as e:
            raise Exception(f"Response missing required keys: {e}")

        with open(path, "rb") as f:
            headers = {
                "Authorization": f"Bearer {self._apikey}",
                "Content-Encoding": content_encoding,
            }

            response = requests.post(
                url=upload_url,
                headers=headers,
                data=f,
            )

            if response.status_code != 200:
                raise Exception(
                    f"Data file upload request failed with status code {response.status_code}. {response.text}"
                )

            return DataImportStatus(self._rest_conf, data_import_id)

    def upload_from_url(
        self,
        url: str,
        csv_config: CsvConfig,
    ) -> DataImportStatus:
        parsed_url = urlparse(url)
        if parsed_url.scheme not in ["s3", "http", "https"]:
            raise Exception(
                f"Invalid URL scheme: '{parsed_url.scheme}'. Only S3 and HTTP(S) URLs are supported."
            )

        headers = {"Authorization": f"Bearer {self._apikey}"}

        response = requests.post(
            url=self._url_uri,
            headers=headers,
            data=json.dumps(
                (
                    {
                        "url": url,
                        "csv_config": csv_config.to_dict(),
                    }
                )
            ),
        )

        if response.status_code != 200:
            raise Exception(
                f"URL upload request failed with status code {response.status_code}. {response.text}"
            )

        try:
            upload_info = response.json()
        except (json.decoder.JSONDecodeError, KeyError) as e:
            raise Exception(f"Invalid response: {e}")

        try:
            data_import_id: str = upload_info["dataImportId"]
        except KeyError as e:
            raise Exception(f"Response missing required keys: {e}")

        return DataImportStatus(self._rest_conf, data_import_id)

    @staticmethod
    def _mime_and_content_type_from_path(path: Path) -> Tuple[str, Optional[str], Optional[str]]:
        file_name = path.name
        mime, encoding = mimetypes.guess_type(path)
        return file_name, mime, encoding

    @staticmethod
    def _compute_uri(restconf: SiftRestConfig) -> str:
        uri = restconf["uri"]

        scheme_match = re.match(r"(.+://).+", uri)
        if scheme_match:
            raise Exception(f"The URL scheme '{scheme_match.groups()[0]}' should not be included")

        if restconf.get("use_ssl", True):
            return f"https://{uri}"

        return f"http://{uri}"
