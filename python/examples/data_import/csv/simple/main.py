import os

from dotenv import load_dotenv
from sift_py.data_import.csv import CsvUploadService
from sift_py.data_import.status import DataImportService
from sift_py.rest import SiftRestConfig

if __name__ == "__main__":
    """
    Example of uploading a CSV file into Sift using default CSV config.
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

    # Can optionally specify units_row=N or description_row=N if these rows exist.
    # Must also specify first_data_row=N in this case.
    import_service: DataImportService = csv_upload_service.simple_upload(
        asset_name,
        "sample_data.csv",
    )
    data_import = import_service.get_data_import()
    print(data_import.model_dump_json(indent=1))

    print("Waiting for upload to complete...")
    import_service.wait_until_complete()
    print("Upload example complete!")
