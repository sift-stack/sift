import csv
import os

from dotenv import load_dotenv
from sift_py.rest import SiftRestConfig
from sift_py.upload.config import CsvConfig
from sift_py.upload.csv import CsvUploadService

if __name__ == "__main__":
    """
    Example of uploading a CSV file from a CSV file into Sift.
    """

    load_dotenv()

    sift_uri = os.getenv("SIFT_API_URI")
    assert sift_uri, "expected 'SIFT_API_URI' environment variable to be set"

    apikey = os.getenv("SIFT_API_KEY")
    assert apikey, "expected 'SIFT_API_KEY' environment variable to be set"

    asset_name = os.getenv("ASSET_NAME")
    assert asset_name, "expected 'ASSET_NAME' environment variable to be set"

    rest_config: SiftRestConfig = {
        # Be sure to exclude the "https://" or "http://" scheme out of the uri
        "uri": sift_uri,
        "apikey": apikey,
        # TODO: Remove
        "use_ssl": False,
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
                "data_type": "CHANNEL_DATA_TYPE_DOUBLE",  # Assume all channels are doubles
                "description": f"Example channel {channel}",
            }

    csv_config = CsvConfig(
        {
            "asset_name": asset_name,
            "first_data_row": 2,
            "time_column": {
                "format": "TIME_FORMAT_ABSOLUTE_DATETIME",
                "column_number": 1,
            },
            "data_columns": data_config,
        }
    )

    status = csv_upload_service.upload(input_csv, csv_config)
    status.wait(verbose=True)
    print("Upload example complete!")
