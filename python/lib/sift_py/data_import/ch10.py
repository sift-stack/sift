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

    Set `gzip` to `True` if sending a compressed stream.

    Example:
    ```python

    class Ch10(BaseCh10File):

        def __init__(self, path):
            self.file = open(path, "rb")
            self.initialize_csv_data_columns = None

        def initialize_csv_data_columns(self):
            self.csv_config_data_columns = self.process_ch10_computer_f1_packet()

        def process_ch10_computer_f1_packet(self) -> Dict[int, dict]:
            # Processes the first Computer F1 packet
            # and returns the measurements as a dict.
            ...

        def process_ch10_pcm_packet(self) -> str:
            # Processed the data packets and returns
            # a CSV row.
            ...

        def __next__(self) -> str:
            # On all iterations, return data for the CSV file.
            if end_of_file:
                raise StopIteration()
            else:
                return self.process_ch10_data_packet()
    ```
    """

    csv_config_data_columns: Dict[int, dict]
    gzip: bool = False

    def initialize_csv_data_columns(self) -> None:
        """
        Must populate the `csv_config_data_columns` attribute
        that is the data_columns entry in the CsvConfig.

        See the Sift data_import module or API docs for the schema.
        """
        raise NotImplementedError

    def __iter__(self):
        return self

    def __next__(self) -> str:
        raise NotImplementedError


class Ch10UploadService(CsvUploadService):
    """Service to upload ch10 files."""

    def upload(  # type: ignore
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
        ch10_file.initialize_csv_data_columns()

        assert getattr(ch10_file, "csv_config_data_columns"), (
            "`csv_config_data_columns` was not set correctly on the first iteration"
        )

        config_info: Dict[str, Any] = {
            "asset_name": asset_name,
            "first_data_row": 2,
            "time_column": {
                "format": time_format,
                "column_number": 1,
            },
            "data_columns": ch10_file.csv_config_data_columns,
        }
        if run_name:
            config_info["run_name"] = run_name

        if run_id:
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
        except (json.decoder.JSONDecodeError, KeyError) as e:
            raise Exception(f"Invalid response: {response.text}.\n{e}")

        try:
            upload_url: str = upload_info["uploadUrl"]
            data_import_id: str = upload_info["dataImportId"]
        except KeyError as e:
            raise Exception(
                f"Response missing required keys: {e}. This is unexpected. Please reach out to the Sift team about this error."
            )

        headers = {
            "Authorization": f"Bearer {self._apikey}",
        }

        if ch10_file.gzip:
            headers["Content-Encoding"] = "gzip"

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
