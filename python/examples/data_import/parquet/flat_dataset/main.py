import os

from dotenv import load_dotenv
from sift_py.data_import.parquet import ParquetUploadService
from sift_py.data_import.status import DataImportService
from sift_py.data_import.time_format import TimeFormatType
from sift_py.rest import SiftRestConfig

if __name__ == "__main__":
    """
    Example usage for uploading a Parquet (flat dataset).
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

    parquet_upload_service = ParquetUploadService(rest_config)

    import_service: DataImportService = parquet_upload_service.flat_dataset_upload(
        asset_name=asset_name,
        run_name="Example Parquet Upload",
        path="sample_data.parquet",
        time_path="timestamp",
        time_format=TimeFormatType.ABSOLUTE_UNIX_NANOSECONDS,
    )

    data_import = import_service.get_data_import()
    print(data_import.model_dump_json(indent=1))

    print("Waiting for upload to complete...")
    import_service.wait_until_complete()
    print("Upload example complete!")
