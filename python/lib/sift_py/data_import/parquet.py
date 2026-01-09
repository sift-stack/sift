import base64
import gzip
import json
import os
import struct
from pathlib import Path
from typing import Optional, Tuple, Union
from urllib.parse import urljoin, urlparse

from sift_py.data_import._utils import ProgressFile, convert_keys_to_snake_case
from sift_py.data_import.config import ParquetConfig
from sift_py.data_import.parquet_complex_types import ParquetComplexTypesImportModeType
from sift_py.data_import.status import DataImportService
from sift_py.data_import.time_format import TimeFormatType
from sift_py.rest import SiftRestConfig, _RestService


class ParquetUploadService(_RestService):
    UPLOAD_PATH = "/api/v1/data-imports:upload"
    URL_PATH = "/api/v1/data-imports:url"
    DETECT_CONFIG_PATH = "/api/v0/data-imports:detect-config"

    _rest_conf: SiftRestConfig
    _upload_uri: str
    _url_uri: str
    _apikey: str

    def __init__(self, rest_conf: SiftRestConfig):
        super().__init__(rest_conf=rest_conf)
        self._upload_uri = urljoin(self._base_uri, self.UPLOAD_PATH)
        self._url_uri = urljoin(self._base_uri, self.URL_PATH)
        self._detect_config_uri = urljoin(self._base_uri, self.DETECT_CONFIG_PATH)

    def upload(
        self,
        path: Union[str, Path],
        parquet_config: ParquetConfig,
        show_progress: bool = True,
    ) -> DataImportService:
        """
        Uploads the Parquet file pointed to by `path` using a custom Parquet config.

        Args:
            path: The path to the Parquet file.
            parquet_config: The Parquet config.
            show_progress: Whether to show the status bar or not.
        """
        # Verify this is a valid Parquet file.
        _extract_parquet_footer(path)

        response = self._session.post(
            url=self._upload_uri,
            headers={
                "Content-Encoding": "application/octet-stream",
            },
            data=json.dumps({"parquet_config": parquet_config.to_dict()}),
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

        with ProgressFile(path, disable=not show_progress) as f:
            headers = {
                "Content-Encoding": "application/octet-stream",
                "Content-Disposition": f'attachment; filename="{os.path.basename(path)}"',
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
        parquet_config: ParquetConfig,
    ) -> DataImportService:
        """
        Uploads the Parquet file pointed to by `url` using a custom Parquet config.
        """
        parsed_url = urlparse(url)
        if parsed_url.scheme not in ["s3", "http", "https"]:
            raise Exception(
                f"Invalid URL scheme: '{parsed_url.scheme}'. Only S3 and HTTP(S) URLs are supported."
            )

        response = self._session.post(
            url=self._url_uri,
            data=json.dumps(
                {
                    "url": url,
                    "parquet_config": parquet_config.to_dict(),
                }
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

    def flat_dataset_upload(
        self,
        asset_name: str,
        path: Union[str, Path],
        time_path: str,
        time_format: TimeFormatType = TimeFormatType.ABSOLUTE_UNIX_NANOSECONDS,
        complex_types_import_mode: ParquetComplexTypesImportModeType = ParquetComplexTypesImportModeType.BOTH,
        run_name: Optional[str] = None,
        run_id: Optional[str] = None,
        relative_start_time: Optional[str] = None,
    ) -> DataImportService:
        """
        Uploads the Parquet file pointed to by `path` to the specified asset. This function will
        automatically generate the Parquet Config using the footer. See the options
        below for what parameters can be overridden. Use `upload` if you need to specify a custom Parquet config.

        Set `time_path` to specify which column contains timestamp information and `time_format`
        to specify the time data format. Default is `TimeFormatType.ABSOLUTE_UNIX_NANOSECONDS`.

        Override `complex_types_import_mode` to specify how to import complex types (maps and list). Default is both strings and bytes.
        Override `run_name` to specify the name of the run to create for this data. Default is None.
        Override `run_id` to specify the id of the run to add this data to. Default is None.
        Override `relative_start_time` if a relative time format is used. Default is None.
        """
        config_info = self._detect_config_flat_dataset(path)

        config_info["asset_name"] = asset_name

        config_info["flat_dataset"]["time_column"]["path"] = time_path
        config_info["flat_dataset"]["time_column"]["format"] = time_format
        if relative_start_time is not None:
            config_info["flat_dataset"]["time_column"]["relative_start_time"] = relative_start_time

        config_info["complex_types_import_mode"] = complex_types_import_mode

        if run_name is not None:
            config_info["run_name"] = run_name

        if run_id is not None:
            config_info["run_id"] = run_id

        parquet_config = ParquetConfig(config_info)

        return self.upload(path, parquet_config)

    def _detect_config_flat_dataset(self, path: Union[str, Path]) -> dict:
        """Returns a dictionary representing the flat dataset Parquet config detected
        from the file.
        """
        footer_bytes, footer_offset = _extract_parquet_footer(path)
        encoded_data = base64.b64encode(footer_bytes).decode("utf-8")
        request_data = json.dumps(
            {
                "data": encoded_data,
                "type": "DATA_TYPE_KEY_PARQUET_FLATDATASET",
            }
        )
        compressed_data = gzip.compress(request_data.encode())
        response = self._session.post(
            url=self._detect_config_uri, data=compressed_data, headers={"Content-Encoding": "gzip"}
        )

        if response.status_code != 200:
            raise Exception(
                f"Detect config request failed with status code {response.status_code}. {response.text}"
            )

        try:
            config_info = convert_keys_to_snake_case(response.json())
        except (json.decoder.JSONDecodeError, KeyError) as e:
            raise Exception(f"Invalid response: {e}")

        if "parquet_config" not in config_info:
            raise Exception(f"Parquet config missing from detect config response: {config_info}")

        # Add the footer_offset which includes the 8 byte footer tail.
        config_info["parquet_config"]["footer_offset"] = footer_offset

        return config_info["parquet_config"]


def _extract_parquet_footer(filename: Union[str, Path]) -> Tuple[bytes, int]:
    """Return the Parquet footer bytes and footer offset"""
    # Footer length is 8 bytes long at the end of the file.
    with open(filename, "rb") as f:
        f.seek(-8, 2)
        footer_tail_bytes = f.read(8)
        footer_len = struct.unpack("<I", footer_tail_bytes[:4])[0]
        magic = footer_tail_bytes[4:]
        if magic != b"PAR1":
            raise ValueError("Invalid Parquet file: missing magic bytes")
        f.seek(-(footer_len + 8), 2)
        footer_bytes = f.read(footer_len)
        return footer_bytes, os.path.getsize(filename) - len(footer_bytes) - 8
