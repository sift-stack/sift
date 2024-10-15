import csv
import os

from dotenv import load_dotenv
from sift_py.data_import.config import CsvConfig
from sift_py.data_import.csv import CsvUploadService
from sift_py.data_import.status import DataImportService
from sift_py.rest import SiftRestConfig

if __name__ == "__main__":
    """
    Example of uploading a CSV file into Sift using custom CSV config.
    """

    load_dotenv()

    sift_uri = os.getenv("SIFT_API_URI")
    assert sift_uri, "expected 'SIFT_API_URI' environment variable to be set"

    apikey = os.getenv("SIFT_API_KEY")
    assert apikey, "expected 'SIFT_API_KEY' environment variable to be set"

    asset_name = os.getenv("ASSET_NAME")
    assert asset_name, "expected 'ASSET_NAME' environment variable to be set"

    rest_config: SiftRestConfig = {
        "uri": sift_uri,
        "apikey": apikey,
    }

    csv_upload_service = CsvUploadService(rest_config)

    # Create CSV config.
    input_csv = "sample_data.csv"

    # Parse CSV to get channel names.
    data_config = {}
    with open(input_csv, "r") as f:
        reader = csv.DictReader(f)
        headers = next(reader)
        for i, channel in enumerate(headers):
            if channel == "timestamp":
                continue
            data_config[i + 1] = {
                "name": channel,
                # This example assumes all channels are doubles.
                # Can also use `ChannelDoubleType.DOUBLE` or `double`
                "data_type": "CHANNEL_DATA_TYPE_DOUBLE",
                "description": f"Example channel {channel}",
            }

    csv_config = CsvConfig(
        {
            "asset_name": asset_name,
            "first_data_row": 2,
            "time_column": {
                "format": "TIME_FORMAT_ABSOLUTE_DATETIME",
                # Can also use `TimeFormatType.ABSOLUTE_DATETIME`
                "column_number": 1,
            },
            "data_columns": data_config,
        }
    )

    import_service: DataImportService = csv_upload_service.upload(input_csv, csv_config)
    import_service.wait_until_complete()
    print("Upload example complete!")
