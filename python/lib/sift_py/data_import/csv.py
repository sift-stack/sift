import json
import mimetypes
import os
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional, Tuple, Union, cast
from urllib.parse import urljoin, urlparse

import pandas as pd
from alive_progress import alive_bar  # type: ignore

from sift_py.data_import.config import CsvConfig
from sift_py.data_import.status import DataImportService
from sift_py.data_import.time_format import TimeFormatType
from sift_py.ingestion.channel import ChannelDataType
from sift_py.rest import SiftRestConfig, _RestService


class CsvUploadService(_RestService):
    UPLOAD_PATH = "/api/v1/data-imports:upload"
    URL_PATH = "/api/v1/data-imports:url"

    _rest_conf: SiftRestConfig
    _upload_uri: str
    _url_uri: str
    _apikey: str

    def __init__(self, rest_conf: SiftRestConfig):
        super().__init__(rest_conf=rest_conf)
        self._upload_uri = urljoin(self._base_uri, self.UPLOAD_PATH)
        self._url_uri = urljoin(self._base_uri, self.URL_PATH)

    def upload(
        self,
        path: Union[str, Path],
        csv_config: CsvConfig,
        show_progress: bool = True,
    ) -> DataImportService:
        """
        Uploads the CSV file pointed to by `path` using a custom CSV config.

        Args:
            path: The path to the CSV file.
            csv_config: The CSV config.
            show_progress: Whether to show the status bar or not.
        """
        content_encoding = self._validate_file_type(path)

        response = self._session.post(
            url=self._upload_uri,
            headers={
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
        except (json.decoder.JSONDecodeError, KeyError):
            raise Exception(f"Invalid response: {response.text}")

        try:
            upload_url: str = upload_info["uploadUrl"]
            data_import_id: str = upload_info["dataImportId"]
        except KeyError as e:
            raise Exception(f"Response missing required keys: {e}")

        with _ProgressFile(path, disable=not show_progress) as f:
            headers = {
                "Content-Encoding": content_encoding,
            }

            response = self._session.post(
                url=upload_url,
                headers=headers,
                data=f,
            )

            if response.status_code != 200:
                raise Exception(
                    f"Data file upload request failed with status code {response.status_code}. {response.text}"
                )

            return DataImportService(self._rest_conf, data_import_id)

    def upload_from_url(
        self,
        url: str,
        csv_config: CsvConfig,
    ) -> DataImportService:
        """
        Uploads the CSV file pointed to by `url` using a custom CSV config.
        """
        parsed_url = urlparse(url)
        if parsed_url.scheme not in ["s3", "http", "https"]:
            raise Exception(
                f"Invalid URL scheme: '{parsed_url.scheme}'. Only S3 and HTTP(S) URLs are supported."
            )

        response = self._session.post(
            url=self._url_uri,
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

        return DataImportService(self._rest_conf, data_import_id)

    def simple_upload(
        self,
        asset_name: str,
        path: Union[str, Path],
        first_data_row: int = 2,
        time_column: int = 1,
        time_format: TimeFormatType = TimeFormatType.ABSOLUTE_DATETIME,
        run_name: Optional[str] = None,
        run_id: Optional[str] = None,
        units_row: Optional[int] = None,
        descriptions_row: Optional[int] = None,
        relative_start_time: Optional[str] = None,
    ) -> DataImportService:
        """
        Uploads the CSV file pointed to by `path` to the specified asset. This function will
        infer the data types and assume certain things about how the data is formatted. See the options
        below for what parameters can be overridden. Use `upload` if you need to specify a custom CSV config.

        Override `first_data_row` to specify which is the first row with data. Default is 2.
        Override `time_column` to specify which column contains timestamp information. Default is 1.
        Override `time_format` to specify the time data format. Default is `TimeFormatType.ABSOLUTE_DATETIME`.
        Override `run_name` to specify the name of the run to create for this data. Default is None.
        Override `run_id` to specify the id of the run to add this data to. Default is None.
        Override `units_row` to specify which row contains unit information. Default is None.
        Override `descriptions_row` to specify which row contains channel description information. Default is None.
        Override `relative_start_time` if a relative time format is used. Default is None.
        """
        self._validate_file_type(path)

        # Convert to 0 index
        skip_rows: List[int] = []
        if units_row is not None:
            units_row -= 1
            skip_rows.append(units_row)
        if descriptions_row is not None:
            descriptions_row -= 1
            skip_rows.append(descriptions_row)

        data_config = {}
        df = pd.read_csv(path, skiprows=skip_rows)

        units: List[str] = []
        if units_row is not None:
            df_units = pd.read_csv(path, nrows=units_row)
            units = list(cast(List[str], df_units.iloc[units_row - 1].astype(str)))

        descriptions: List[str] = []
        if descriptions_row is not None:
            df_descriptions = pd.read_csv(path, nrows=descriptions_row)
            descriptions = list(
                cast(List[str], df_descriptions.iloc[descriptions_row - 1].astype(str))
            )

        for i, header in enumerate(df.columns):
            if i + 1 == time_column:
                continue

            raw_dtype = str(df[df.columns[i]].dtype)
            if raw_dtype == "float64":
                raw_dtype = "double"
            # String columns are set to 'object'. Use infer_dtypes
            # to verify this is a string column
            elif raw_dtype == "object":
                raw_dtype = pd.api.types.infer_dtype(df[df.columns[i]], skipna=False)

            data_type = ChannelDataType.from_str(raw_dtype)
            if data_type is None:
                raise Exception(
                    f"Unable to upload data type in column {i + 1} {header}: Type: {raw_dtype}."
                )
            data_config[i + 1] = {"name": header, "data_type": data_type}

            if units:
                data_config[i + 1]["units"] = units[i] if units[i] != "nan" else ""

            if descriptions:
                data_config[i + 1]["description"] = (
                    descriptions[i] if descriptions[i] != "nan" else ""
                )

        config_info: Dict[str, Any] = {
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
            config_info["run_id"] = run_id

        if relative_start_time is not None:
            config_info["time_column"]["relative_start_time"] = relative_start_time

        csv_config = CsvConfig(config_info)

        return self.upload(path, csv_config)

    def _validate_file_type(self, path: Union[str, Path]) -> Optional[str]:
        posix_path = Path(path) if isinstance(path, str) else path

        if not posix_path.is_file():
            raise Exception(f"Provided path, '{path}', does not point to a regular file.")

        _, mimetype, content_encoding = self.__class__._mime_and_content_type_from_path(posix_path)

        if not mimetype:
            raise Exception(f"The MIME-type of '{posix_path}' could not be computed.")

        valid_types = ["test/plain", "text/csv", "application/vnd.ms-excel"]
        if mimetype not in valid_types:
            raise Exception(
                f"{path} is not a valid file type ({mimetype}). Must be {', '.join(valid_types)}."
            )

        return content_encoding

    @staticmethod
    def _mime_and_content_type_from_path(path: Path) -> Tuple[str, Optional[str], Optional[str]]:
        file_name = path.name
        mime, encoding = mimetypes.guess_type(path)
        return file_name, mime, encoding


class _ProgressFile:
    """Displays the status with alive_bar while reading the file."""

    # alive_bar only supports context managers, so we have to make the
    # context manager calls manually.
    _bar_context: Callable

    def __init__(self, path: Union[str, Path], disable=False):
        self.path = path

        self.file_size = os.path.getsize(self.path)
        if self.file_size == 0:
            raise Exception(f"{path} is 0 bytes")

        self._file = open(self.path, mode="rb")
        self._bar = alive_bar(self.file_size, unit=" bytes", disable=disable, scale="SI")

    def read(self, *args, **kwargs):
        chunk = self._file.read(*args, **kwargs)
        self._bar_context(len(chunk))
        return chunk

    def __enter__(self):
        self._bar_context = self._bar.__enter__()
        return self

    def __exit__(self, *args, **kwargs):
        self._bar.__exit__(None, None, None)
        return
