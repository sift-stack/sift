import json
import mimetypes
from pathlib import Path
from typing import Optional, Tuple, Union
from urllib.parse import urljoin, urlparse

import pandas as pd
import requests
from sift_py.data_import.config import CsvConfig
from sift_py.data_import.status import DataImportStatus
from sift_py.data_import.time_format import TimeFormatType
from sift_py.rest import SiftRestConfig, compute_uri


class CsvUploadService:
    UPLOAD_PATH = "/api/v1/data-imports:upload"
    URL_PATH = "/api/v1/data-imports:url"

    _rest_conf: SiftRestConfig
    _upload_uri: str
    _url_uri: str
    _apikey: str

    def __init__(self, rest_conf: SiftRestConfig):
        self._rest_conf = rest_conf
        base_uri = compute_uri(rest_conf)
        self._apikey = rest_conf["apikey"]
        self._upload_uri = urljoin(base_uri, self.UPLOAD_PATH)
        self._url_uri = urljoin(base_uri, self.URL_PATH)

    def upload(
        self,
        path: Union[str, Path],
        csv_config: CsvConfig,
    ) -> DataImportStatus:
        content_encoding = self._validate_file_type(path)

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

    def simple_upload(
        self,
        asset_name: str,
        path: Union[str, Path],
        run_name: Optional[str] = None,
        run_id: Optional[str] = None,
        first_data_row: int = 2,
        time_column: int = 1,
        time_format: TimeFormatType = TimeFormatType.ABSOLUTE_DATETIME,
    ) -> DataImportStatus:
        self._validate_file_type(path)

        types = {
            "integer": int,
            "string": str,
            "floating": float,
            "boolean": bool,
        }
        data_config = {}
        df = pd.read_csv(path)
        for i, header in enumerate(df.columns):
            if i + 1 == time_column:
                continue

            inferred_dtype = pd.api.types.infer_dtype(df[df.columns[i]], skipna=False)
            dtype = types.get(inferred_dtype)
            if dtype is None:
                raise Exception(
                    f"Unable to upload data type in column {i+1} {header}. Inferred type: {inferred_dtype}"
                )

            data_config[i + 1] = {"name": header, "data_type": dtype}

        config_info = {
            "asset_name": asset_name,
            "first_data_row": first_data_row,
            "time_column": {
                "format": time_format,
                "column_number": time_column,
            },
            "data_columns": data_config,
        }

        if run_name is not None:
            config_info["run_name"] = run_name

        if run_id is not None:
            config_info["run_id"] = run_name

        csv_config = CsvConfig(config_info)

        return self.upload(path, csv_config)

    def _validate_file_type(self, path: Union[str, Path]) -> Optional[str]:
        posix_path = Path(path) if isinstance(path, str) else path

        if not posix_path.is_file():
            raise Exception(f"Provided path, '{path}', does not point to a regular file.")

        _, mimetype, content_encoding = self.__class__._mime_and_content_type_from_path(posix_path)

        if not mimetype:
            raise Exception(f"The MIME-type of '{posix_path}' could not be computed.")

        if mimetype not in ["test/plain", "text/csv"]:
            raise Exception(f"{path} is not a valid file type. Must be text or csv.")

        return content_encoding

    @staticmethod
    def _mime_and_content_type_from_path(path: Path) -> Tuple[str, Optional[str], Optional[str]]:
        file_name = path.name
        mime, encoding = mimetypes.guess_type(path)
        return file_name, mime, encoding
