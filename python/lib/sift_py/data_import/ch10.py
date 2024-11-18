import json
from typing import Any, Dict, Optional

import requests

from sift_py.data_import.config import CsvConfig
from sift_py.data_import.csv import CsvUploadService
from sift_py.data_import.status import DataImportService
from sift_py.data_import.time_format import TimeFormatType


class BaseCh10File:
    """
    Base class for uploading IRIG Chapter 10/Chapter 11 files.

    Implement a concrete version of this class that parses a ch10 stream and returns
    a csv row of data on each iteration.

    Note: The first iteration must populate the csv_config_data_columns attribute
    that is the data_columns entry in the CsvConfig.
    """

    csv_config_data_columns: Dict[int, dict]

    def __iter__(self):
        return self

    def __next__(self):
        raise NotImplementedError


class Ch10UploadService(CsvUploadService):
    """Service to upload ch10 files."""

    def upload_ch10(
        self,
        ch10_file: BaseCh10File,
        asset_name: str,
        time_format: TimeFormatType = TimeFormatType.ABSOLUTE_UNIX_NANOSECONDS,
        run_name: Optional[str] = None,
        run_id: Optional[str] = None,
    ) -> DataImportService:
        """
        Uploads the ch10 file to the specified asset.

        Override `time_format` to specify the time data format. Default is `TimeFormatType.ABSOLUTE_UNIX_NANOSECONDS`.
        Override `run_name` to specify the name of the run to create for this data. Default is None.
        Override `run_id` to specify the id of the run to add this data to. Default is None.
        """
        # Trigger the first packet read to get the row headers
        # and csv_config_data_columns.
        next(ch10_file)

        config_info: Dict[str, Any] = {
            "asset_name": asset_name,
            "first_data_row": 1,
            "time_column": {
                "format": time_format,
                "column_number": 1,
            },
            "data_columns": ch10_file.csv_config_data_columns,
        }
        if run_name is not None:
            config_info["run_name"] = run_name

        if run_id is not None:
            config_info["run_id"] = run_name

        csv_config = CsvConfig(config_info)

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
        except (json.decoder.JSONDecodeError, KeyError):
            raise Exception(f"Invalid response: {response.text}")

        try:
            upload_url: str = upload_info["uploadUrl"]
            data_import_id: str = upload_info["dataImportId"]
        except KeyError as e:
            raise Exception(f"Response missing required keys: {e}")

        headers = {
            "Authorization": f"Bearer {self._apikey}",
        }

        response = requests.post(
            url=upload_url,
            headers=headers,
            data=ch10_file,
        )

        if response.status_code != 200:
            raise Exception(
                f"Data file upload request failed with status code {response.status_code}. {response.text}"
            )

        return DataImportService(self._rest_conf, data_import_id)
