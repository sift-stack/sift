import json

import pandas as pd
import pytest
from pytest_mock import MockFixture

from sift_py.data_import.config import CsvConfig
from sift_py.data_import.csv import CsvUploadService
from sift_py.rest import SiftRestConfig


class MockResponse:
    status_code: int
    text: str

    def __init__(self, status_code: int, text: str):
        self.status_code = status_code
        self.text = text

    def json(self) -> dict:
        return json.loads(self.text)


csv_config = CsvConfig(
    {
        "asset_name": "test_asset",
        "first_data_row": 2,
        "time_column": {
            "format": "TIME_FORMAT_ABSOLUTE_DATETIME",
            "column_number": 1,
        },
        "data_columns": {
            2: {
                "name": "channel_1",
                "data_type": "CHANNEL_DATA_TYPE_DOUBLE",
            }
        },
    }
)

rest_config: SiftRestConfig = {
    "uri": "some_uri.com",
    "apikey": "123123123",
}


def test_csv_upload_service_upload_validate_path(mocker: MockFixture):
    mock_path_is_file = mocker.patch("sift_py.data_import.csv.Path.is_file")
    mock_path_is_file.return_value = False

    with pytest.raises(Exception, match="does not point to a regular file"):
        svc = CsvUploadService(rest_config)
        svc.upload(
            path="some_csv.csv",
            csv_config=csv_config,
        )


def test_csv_upload_service_upload_validate_mime_type(mocker: MockFixture):
    mock_path_is_file = mocker.patch("sift_py.data_import.csv.Path.is_file")
    mock_path_is_file.return_value = True

    with pytest.raises(Exception, match="MIME"):
        svc = CsvUploadService(rest_config)
        svc.upload(
            path="some_csv.asdfghjkl",
            csv_config=csv_config,
        )

    with pytest.raises(Exception, match="Must be"):
        svc = CsvUploadService(rest_config)
        svc.upload(
            path="some_file.pdf",
            csv_config=csv_config,
        )


def test_csv_upload_service_invalid_config_response(mocker: MockFixture):
    mock_path_is_file = mocker.patch("sift_py.data_import.csv.Path.is_file")
    mock_path_is_file.return_value = True

    mock_requests_post = mocker.patch("sift_py.rest.requests.Session").post
    mock_requests_post.return_value = MockResponse(status_code=400, text="Invalid request")
    with pytest.raises(Exception, match="Config file upload request failed"):
        svc = CsvUploadService(rest_config)

        svc.upload(
            path="some_csv.csv",
            csv_config=csv_config,
        )


def test_csv_upload_service_invalid_data_response(mocker: MockFixture):
    mock_path_is_file = mocker.patch("sift_py.data_import.csv.Path.is_file")
    mock_path_is_file.return_value = True
    mock_path_getsize = mocker.patch("sift_py.data_import.csv.os.path.getsize")
    mock_path_getsize.return_value = 1024

    mocker.patch(
        "sift_py.data_import.csv.open",
        mocker.mock_open(),
    )

    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse(status_code=200, text="asdgasdg")

    with pytest.raises(Exception, match="Invalid response"):
        svc = CsvUploadService(rest_config)

        svc.upload(
            path="some_csv.csv",
            csv_config=csv_config,
        )

    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse(status_code=200, text="{}")

    with pytest.raises(Exception, match="Response missing required keys"):
        svc = CsvUploadService(rest_config)

        svc.upload(
            path="some_csv.csv",
            csv_config=csv_config,
        )

    mock_requests_post.side_effect = [
        MockResponse(
            status_code=200,
            text=json.dumps({"uploadUrl": "some_url.com", "dataImportId": "123-123-123"}),
        ),
        MockResponse(status_code=400, text="Invalid request"),
    ]

    with pytest.raises(Exception, match="Data file upload request failed"):
        svc = CsvUploadService(rest_config)

        svc.upload(
            path="some_csv.csv",
            csv_config=csv_config,
        )


def test_csv_upload_service_success(mocker: MockFixture):
    mock_path_is_file = mocker.patch("sift_py.data_import.csv.Path.is_file")
    mock_path_is_file.return_value = True
    mock_path_getsize = mocker.patch("sift_py.data_import.csv.os.path.getsize")
    mock_path_getsize.return_value = 1024

    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.side_effect = [
        MockResponse(
            status_code=200,
            text=json.dumps({"uploadUrl": "some_url.com", "dataImportId": "123-123-123"}),
        ),
        MockResponse(status_code=200, text=""),
    ]

    mocker.patch(
        "sift_py.data_import.csv.open",
        mocker.mock_open(),
    )
    svc = CsvUploadService(
        {
            "uri": "some_uri.com",
            "apikey": "123123123",
        },
    )

    svc.upload(
        path="some_csv.csv",
        csv_config=csv_config,
    )


def test_csv_upload_service_upload_validate_url(mocker: MockFixture):
    with pytest.raises(Exception, match="Invalid URL scheme:"):
        svc = CsvUploadService(rest_config)

        svc.upload_from_url(
            url="asdf://some_url.com/file.csv",
            csv_config=csv_config,
        )


def test_csv_upload_service_upload_from_url_invalid_config(mocker: MockFixture):
    mock_requests_post = mocker.patch("sift_py.rest.requests.Session").post
    mock_requests_post.return_value = MockResponse(status_code=400, text="Invalid request")
    with pytest.raises(Exception, match="URL upload request failed"):
        svc = CsvUploadService(rest_config)

        svc.upload_from_url(
            url="http://some_url.com/file.csv",
            csv_config=csv_config,
        )


def test_csv_upload_service_upload_from_url_success(mocker: MockFixture):
    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse(
        status_code=200,
        text=json.dumps({"uploadUrl": "some_url.com", "dataImportId": "123-123-123"}),
    )
    svc = CsvUploadService(
        {
            "uri": "some_uri.com",
            "apikey": "123123123",
        },
    )

    svc.upload_from_url(
        url="http://some_url.com/file.csv",
        csv_config=csv_config,
    )


def test_simple_upload_invalid_csv(mocker: MockFixture):
    mock_path_is_file = mocker.patch("sift_py.data_import.csv.Path.is_file")
    mock_path_is_file.return_value = True
    mock_path_getsize = mocker.patch("sift_py.data_import.csv.os.path.getsize")
    mock_path_getsize.return_value = 1024

    mock_read_csv = mocker.patch("sift_py.data_import.csv.pd.read_csv")
    mock_read_csv.return_value = pd.DataFrame(
        {
            "time": [1, 2, 3],
            "channel_1": [1, 1.0, True],
        }
    )
    with pytest.raises(Exception, match="Unable to upload.*"):
        svc = CsvUploadService(rest_config)
        svc.simple_upload("test_asset", "sample.csv")

    mock_read_csv = mocker.patch("sift_py.data_import.csv.pd.read_csv")
    mock_read_csv.return_value = pd.DataFrame(
        {
            "time": [1, 2, 3],
            "channel_1": [complex(1), complex(1), complex(1)],
        }
    )
    with pytest.raises(Exception, match="Unable to upload.*"):
        svc = CsvUploadService(rest_config)
        svc.simple_upload("test_asset", "sample.csv")

    mock_read_csv = mocker.patch("sift_py.data_import.csv.pd.read_csv")
    mock_read_csv.return_value = pd.DataFrame(
        {
            "time": [1, 2, 3],
            "channel_bool": [True, True, False],
            "channel_int": [-1, 2, 0],
            "channel_double": [1.0, 2.0, -3.3],
            "channel_string": ["a", "b", "c"],
        }
    )
    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.side_effect = [
        MockResponse(
            status_code=200,
            text=json.dumps({"uploadUrl": "some_url.com", "dataImportId": "123-123-123"}),
        ),
        MockResponse(status_code=200, text=""),
    ]
    mocker.patch(
        "sift_py.data_import.csv.open",
        mocker.mock_open(),
    )
    svc = CsvUploadService(rest_config)
    svc.simple_upload("test_asset", "sample.csv")


def test_simple_upload_metadata_csv(mocker: MockFixture):
    mock_path_is_file = mocker.patch("sift_py.data_import.csv.Path.is_file")
    mock_path_is_file.return_value = True
    mock_path_getsize = mocker.patch("sift_py.data_import.csv.os.path.getsize")
    mock_path_getsize.return_value = 1024

    def mock_read_csv(*_, **kwargs):
        if "skiprows" in kwargs:
            return pd.DataFrame(
                {
                    "time": [1, 2, 3],
                    "channel_int": [-1, 2, 1],
                }
            )
        else:
            return pd.DataFrame(
                {
                    "time": ["s", "a description", 1, 2, 3],
                    "channel_int": ["degC", "another description", -1, 2, 1],
                }
            )

    mocker.patch("sift_py.data_import.csv.pd.read_csv", mock_read_csv)

    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.side_effect = [
        MockResponse(
            status_code=200,
            text=json.dumps({"uploadUrl": "some_url.com", "dataImportId": "123-123-123"}),
        ),
        MockResponse(status_code=200, text=""),
    ]
    mocker.patch(
        "sift_py.data_import.csv.open",
        mocker.mock_open(),
    )
    svc = CsvUploadService(rest_config)

    svc.simple_upload("test_asset", "sample.csv", units_row=2, descriptions_row=3)

    expected_csv_config = CsvConfig(
        {
            "asset_name": "test_asset",
            "run_name": "",
            "run_id": "",
            "first_data_row": 2,
            "time_column": {
                "format": "TIME_FORMAT_ABSOLUTE_DATETIME",
                "column_number": 1,
            },
            "data_columns": {
                "2": {
                    "name": "channel_int",
                    "data_type": "CHANNEL_DATA_TYPE_INT_64",
                    "units": "degC",
                    "description": "another description",
                    "enum_types": [],
                    "bit_field_elements": [],
                }
            },
        }
    )
    mock_requests_post.assert_any_call(
        url="https://some_uri.com/api/v1/data-imports:upload",
        headers={
            "Content-Encoding": "application/octet-stream",
        },
        data=json.dumps({"csv_config": expected_csv_config.to_dict()}),
    )


def test_simple_upload_uint64_csv(mocker: MockFixture):
    mock_path_is_file = mocker.patch("sift_py.data_import.csv.Path.is_file")
    mock_path_is_file.return_value = True
    mock_path_getsize = mocker.patch("sift_py.data_import.csv.os.path.getsize")
    mock_path_getsize.return_value = 1024

    mock_read_csv = mocker.patch("sift_py.data_import.csv.pd.read_csv")
    mock_read_csv.return_value = pd.DataFrame(
        {
            "time": [1, 2, 3],
            "channel_uint64": [1, 2, 2**63],
        }
    )

    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.side_effect = [
        MockResponse(
            status_code=200,
            text=json.dumps({"uploadUrl": "some_url.com", "dataImportId": "123-123-123"}),
        ),
        MockResponse(status_code=200, text=""),
    ]
    mocker.patch(
        "sift_py.data_import.csv.open",
        mocker.mock_open(),
    )
    svc = CsvUploadService(rest_config)

    svc.simple_upload("test_asset", "sample.csv")

    expected_csv_config = CsvConfig(
        {
            "asset_name": "test_asset",
            "run_name": "",
            "run_id": "",
            "first_data_row": 2,
            "time_column": {
                "format": "TIME_FORMAT_ABSOLUTE_DATETIME",
                "column_number": 1,
            },
            "data_columns": {
                "2": {
                    "name": "channel_uint64",
                    "data_type": "CHANNEL_DATA_TYPE_UINT_64",
                    "units": "",
                    "description": "",
                    "enum_types": [],
                    "bit_field_elements": [],
                }
            },
        }
    )
    mock_requests_post.assert_any_call(
        url="https://some_uri.com/api/v1/data-imports:upload",
        headers={
            "Content-Encoding": "application/octet-stream",
        },
        data=json.dumps({"csv_config": expected_csv_config.to_dict()}),
    )
