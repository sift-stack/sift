import json

import pytest
from pytest_mock import MockFixture

from sift_py.data_import.config import ParquetConfig
from sift_py.data_import.parquet import ParquetUploadService, _extract_parquet_footer
from sift_py.data_import.parquet_complex_types import ParquetComplexTypesImportModeType
from sift_py.data_import.status import DataImportService
from sift_py.data_import.time_format import TimeFormatType
from sift_py.rest import SiftRestConfig


class MockResponse:
    def __init__(self, status_code=200, text="", json_data=None):
        self.status_code = status_code
        self.text = text
        self._json_data = json_data

    def json(self):
        if self._json_data is not None:
            return self._json_data
        raise Exception("Invalid response")


rest_config: SiftRestConfig = {
    "uri": "http://some_uri.com",
    "apikey": "123123123",
}


@pytest.fixture
def parquet_config():
    return ParquetConfig(
        {
            "asset_name": "test_asset",
            "flat_dataset": {
                "time_column": {"path": "time", "format": "TIME_FORMAT_ABSOLUTE_UNIX_NANOSECONDS"},
                "data_columns": [
                    {
                        "path": "col1",
                        "channel_config": {"name": "col1", "data_type": "CHANNEL_DATA_TYPE_INT_64"},
                    },
                    {
                        "path": "col2",
                        "channel_config": {
                            "name": "col2",
                            "data_type": "CHANNEL_DATA_TYPE_FLOAT",
                            "units": "m",
                            "description": "second column",
                        },
                    },
                    {
                        "path": "col3",
                        "channel_config": {
                            "name": "col3",
                            "data_type": "CHANNEL_DATA_TYPE_STRING",
                            "description": "third column",
                        },
                    },
                ],
            },
            "complex_types_import_mode": "PARQUET_COMPLEX_TYPES_IMPORT_MODE_BOTH",
            "footer_offset": 0,
            "footer_length": 0,
        }
    )


def test_upload_invalid_extension(mocker: MockFixture, parquet_config):
    svc = ParquetUploadService(rest_config)
    mocker.patch("sift_py.data_import.parquet.open", mocker.mock_open(read_data=b"a" * 200))
    with pytest.raises(Exception, match="Invalid Parquet file: missing magic bytes"):
        svc.upload("file.txt", parquet_config)


def test_upload_config_request_failed(mocker: MockFixture, parquet_config):
    mocker.patch(
        "sift_py.data_import.parquet._extract_parquet_footer", return_value=(b"footerbytes", 8)
    )
    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse(status_code=400, text="Invalid request")
    svc = ParquetUploadService(rest_config)
    with pytest.raises(Exception, match="Config file upload request failed"):
        svc.upload("file.parquet", parquet_config)


def test_upload_invalid_config_response(mocker: MockFixture, parquet_config):
    mocker.patch(
        "sift_py.data_import.parquet._extract_parquet_footer", return_value=(b"footerbytes", 8)
    )
    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse(status_code=200, json_data=None)
    svc = ParquetUploadService(rest_config)
    with pytest.raises(Exception, match="Invalid response"):
        svc.upload("file.parquet", parquet_config)


def test_upload_missing_keys(mocker: MockFixture, parquet_config):
    mocker.patch(
        "sift_py.data_import.parquet._extract_parquet_footer", return_value=(b"footerbytes", 8)
    )
    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse(status_code=200, json_data={})
    svc = ParquetUploadService(rest_config)
    with pytest.raises(Exception, match="Response missing required keys"):
        svc.upload("file.parquet", parquet_config)


def test_upload_data_file_failed(mocker: MockFixture, parquet_config):
    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.side_effect = [
        MockResponse(
            status_code=200, json_data={"uploadUrl": "http://upload.com", "dataImportId": "id123"}
        ),
        MockResponse(status_code=400, text="Upload failed"),
    ]
    mocker.patch("sift_py.data_import.parquet.ProgressFile", mocker.mock_open())
    svc = ParquetUploadService(rest_config)
    with pytest.raises(Exception):
        svc.upload("file.parquet", parquet_config)


def test_upload_success(mocker: MockFixture, parquet_config):
    mocker.patch(
        "sift_py.data_import.parquet._extract_parquet_footer", return_value=(b"footerbytes", 8)
    )
    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.side_effect = [
        MockResponse(
            status_code=200, json_data={"uploadUrl": "http://upload.com", "dataImportId": "id123"}
        ),
        MockResponse(status_code=200, text=""),
    ]
    mocker.patch("sift_py.data_import.parquet.ProgressFile", mocker.mock_open())
    svc = ParquetUploadService(rest_config)
    result = svc.upload("file.parquet", parquet_config)
    assert isinstance(result, DataImportService)


def test_upload_from_url_invalid_scheme(mocker: MockFixture, parquet_config):
    svc = ParquetUploadService(rest_config)
    with pytest.raises(Exception, match="Invalid URL scheme"):
        svc.upload_from_url("ftp://file.parquet", parquet_config)


def test_upload_from_url_failed(mocker: MockFixture, parquet_config):
    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse(status_code=400, text="Invalid request")
    svc = ParquetUploadService(rest_config)
    with pytest.raises(Exception, match="URL upload request failed"):
        svc.upload_from_url("http://file.parquet", parquet_config)


def test_upload_from_url_invalid_response(mocker: MockFixture, parquet_config):
    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse(status_code=200, json_data=None)
    svc = ParquetUploadService(rest_config)
    with pytest.raises(Exception, match="Invalid response"):
        svc.upload_from_url("http://file.parquet", parquet_config)


def test_upload_from_url_missing_keys(mocker: MockFixture, parquet_config):
    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse(status_code=200, json_data={})
    svc = ParquetUploadService(rest_config)
    with pytest.raises(Exception, match="Response missing required keys"):
        svc.upload_from_url("http://file.parquet", parquet_config)


def test_upload_from_url_success(mocker: MockFixture, parquet_config):
    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse(
        status_code=200, json_data={"dataImportId": "id123"}
    )
    svc = ParquetUploadService(rest_config)
    result = svc.upload_from_url("http://file.parquet", parquet_config)
    assert isinstance(result, DataImportService)


def test_flat_dataset_upload_invalid_extension(mocker: MockFixture, parquet_config):
    mocker.patch("sift_py.data_import.parquet.open", mocker.mock_open(read_data=b"a" * 200))
    svc = ParquetUploadService(rest_config)
    with pytest.raises(Exception, match="Invalid Parquet file: missing magic bytes"):
        svc.flat_dataset_upload("asset", "file.txt", "time")


def test_flat_dataset_upload_success(mocker: MockFixture, parquet_config):
    mocker.patch(
        "sift_py.data_import.parquet._extract_parquet_footer", return_value=(b"footerbytes", 8)
    )
    svc = ParquetUploadService(rest_config)
    mock_detect = mocker.patch.object(svc, "_detect_config_flat_dataset")
    mock_detect.return_value = parquet_config.to_dict()
    mock_post = mocker.patch("sift_py.rest.requests.Session.post")

    # First call: config upload, second call: data upload
    mock_post.side_effect = [
        MockResponse(
            status_code=200, json_data={"uploadUrl": "http://upload.com", "dataImportId": "id123"}
        ),
        MockResponse(status_code=200, text=""),
    ]
    mocker.patch("sift_py.data_import.parquet.ProgressFile", mocker.mock_open())
    result = svc.flat_dataset_upload("asset", "file.parquet", "time")
    assert isinstance(result, DataImportService)

    # Check config upload call
    config_call = mock_post.call_args_list[0]
    assert config_call[1]["url"] == svc._upload_uri

    expected_config = {
        "parquet_config": {
            "asset_name": "asset",
            "run_name": "",
            "run_id": "",
            "flat_dataset": {
                "time_column": {
                    "format": "TIME_FORMAT_ABSOLUTE_UNIX_NANOSECONDS",
                    "relative_start_time": None,
                    "path": "time",
                },
                "data_columns": [
                    {
                        "path": "col1",
                        "channel_config": {
                            "name": "col1",
                            "data_type": "CHANNEL_DATA_TYPE_INT_64",
                            "units": "",
                            "description": "",
                            "enum_types": [],
                            "bit_field_elements": [],
                        },
                    },
                    {
                        "path": "col2",
                        "channel_config": {
                            "name": "col2",
                            "data_type": "CHANNEL_DATA_TYPE_FLOAT",
                            "units": "m",
                            "description": "second column",
                            "enum_types": [],
                            "bit_field_elements": [],
                        },
                    },
                    {
                        "path": "col3",
                        "channel_config": {
                            "name": "col3",
                            "data_type": "CHANNEL_DATA_TYPE_STRING",
                            "units": "",
                            "description": "third column",
                            "enum_types": [],
                            "bit_field_elements": [],
                        },
                    },
                ],
            },
            "footer_offset": 0,
            "footer_length": 0,
            "complex_types_import_mode": "PARQUET_COMPLEX_TYPES_IMPORT_MODE_BOTH",
        }
    }
    assert expected_config == json.loads(config_call[1]["data"])


def test_flat_dataset_upload_overrides_success(mocker: MockFixture, parquet_config):
    mocker.patch(
        "sift_py.data_import.parquet._extract_parquet_footer",
        return_value=(b"parquetfooterbytes", 8),
    )
    svc = ParquetUploadService(rest_config)
    mock_detect = mocker.patch.object(svc, "_detect_config_flat_dataset")
    mock_detect.return_value = parquet_config.to_dict()
    mock_post = mocker.patch("sift_py.rest.requests.Session.post")

    # First call: config upload, second call: data upload
    mock_post.side_effect = [
        MockResponse(
            status_code=200, json_data={"uploadUrl": "http://upload.com", "dataImportId": "id123"}
        ),
        MockResponse(status_code=200, text=""),
    ]
    mocker.patch("sift_py.data_import.parquet.ProgressFile", mocker.mock_open())
    result = svc.flat_dataset_upload(
        "asset",
        "file.parquet",
        "time",
        time_format=TimeFormatType.RELATIVE_SECONDS,
        complex_types_import_mode=ParquetComplexTypesImportModeType.BYTES,
        run_id="run_42",
        relative_start_time="2024-01-01T00:00:00Z",
    )
    assert isinstance(result, DataImportService)

    # Check config upload call
    config_call = mock_post.call_args_list[0]
    assert config_call[1]["url"] == svc._upload_uri

    config_call = mock_post.call_args_list[0]
    expected_config = {
        "parquet_config": {
            "asset_name": "asset",
            "run_name": "",
            "run_id": "run_42",
            "flat_dataset": {
                "time_column": {
                    "format": "TIME_FORMAT_RELATIVE_SECONDS",
                    "relative_start_time": "2024-01-01T00:00:00Z",
                    "path": "time",
                },
                "data_columns": [
                    {
                        "path": "col1",
                        "channel_config": {
                            "name": "col1",
                            "data_type": "CHANNEL_DATA_TYPE_INT_64",
                            "units": "",
                            "description": "",
                            "enum_types": [],
                            "bit_field_elements": [],
                        },
                    },
                    {
                        "path": "col2",
                        "channel_config": {
                            "name": "col2",
                            "data_type": "CHANNEL_DATA_TYPE_FLOAT",
                            "units": "m",
                            "description": "second column",
                            "enum_types": [],
                            "bit_field_elements": [],
                        },
                    },
                    {
                        "path": "col3",
                        "channel_config": {
                            "name": "col3",
                            "data_type": "CHANNEL_DATA_TYPE_STRING",
                            "units": "",
                            "description": "third column",
                            "enum_types": [],
                            "bit_field_elements": [],
                        },
                    },
                ],
            },
            "footer_offset": 0,
            "footer_length": 0,
            "complex_types_import_mode": "PARQUET_COMPLEX_TYPES_IMPORT_MODE_BYTES",
        }
    }
    assert expected_config == json.loads(config_call[1]["data"])


def test_detect_config_flat_dataset_failed(mocker: MockFixture):
    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse(status_code=400, text="Failed")
    svc = ParquetUploadService(rest_config)
    svc._detect_config_uri = "http://detect.com"
    mocker.patch("sift_py.data_import.parquet._extract_parquet_footer", return_value=(b"bytes", 8))
    with pytest.raises(Exception):
        svc._detect_config_flat_dataset("file.parquet")


def test_extract_parquet_footer_invalid_magic(mocker):
    mocker.patch(
        "sift_py.data_import.parquet.open", mocker.mock_open(read_data=b"\x08\x00\x00\x00BADMAGIC")
    )
    with pytest.raises(Exception, match="Invalid Parquet file: missing magic bytes"):
        _extract_parquet_footer("test.parquet")


def test_detect_config_flat_dataset_success(mocker: MockFixture):
    # Mock the _extract_parquet_footer to return known bytes and offset
    mocker.patch(
        "sift_py.data_import.parquet._extract_parquet_footer", return_value=(b"footerbytes", 123)
    )
    # Prepare a mock response for the requests post
    mock_response_data = {
        "parquet_config": {
            "asset_name": "asset",
            "flat_dataset": {
                "time_column": {"path": "time", "format": "TIME_FORMAT_ABSOLUTE_UNIX_NANOSECONDS"},
                "data_columns": [],
            },
            "footer_offset": 0,
            "footer_length": 0,
        }
    }
    mock_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_post.return_value = MockResponse(status_code=200, json_data=mock_response_data)
    svc = ParquetUploadService(rest_config)
    svc._detect_config_uri = "http://detect.com"
    result = svc._detect_config_flat_dataset("file.parquet")

    # Should return the dict inside "parquet_config" and add the correct footer_offset
    assert isinstance(result, dict)
    assert result["asset_name"] == "asset"
    assert result["footer_offset"] == 123


def test_detect_config_flat_dataset_invalid_json(mocker: MockFixture):
    mocker.patch(
        "sift_py.data_import.parquet._extract_parquet_footer", return_value=(b"footerbytes", 123)
    )
    mock_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_post.return_value = MockResponse(status_code=200, json_data=None)
    svc = ParquetUploadService(rest_config)
    svc._detect_config_uri = "http://detect.com"
    with pytest.raises(Exception, match="Invalid response"):
        svc._detect_config_flat_dataset("file.parquet")


def test_detect_config_flat_dataset_missing_parquet_config(mocker: MockFixture):
    mocker.patch(
        "sift_py.data_import.parquet._extract_parquet_footer", return_value=(b"footerbytes", 123)
    )
    mock_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_post.return_value = MockResponse(status_code=200, json_data={})
    svc = ParquetUploadService(rest_config)
    svc._detect_config_uri = "http://detect.com"
    with pytest.raises(Exception, match="Parquet config missing from detect config response"):
        svc._detect_config_flat_dataset("file.parquet")


def test_extract_parquet_footer_valid_parquet(mocker):
    # Parquet files end with "PAR1" magic bytes
    # Footer length is stored in the last 8 bytes before the final magic bytes
    # We'll construct a valid parquet tailer file in memory
    file_bytes = b"\x08\x00\x00\x00" + b"PAR1"
    mocker.patch("sift_py.data_import.parquet.open", mocker.mock_open(read_data=file_bytes))
    mocker.patch("sift_py.data_import.parquet.os.path.getsize", return_value=24)
    result_footer, result_offset = _extract_parquet_footer("valid.parquet")
    assert result_offset == 16


def test_extract_parquet_footer_too_short(mocker):
    # File too short to be a valid parquet file
    mocker.patch("sift_py.data_import.parquet.open", mocker.mock_open(read_data=b"PAR1"))
    with pytest.raises(Exception, match="Invalid Parquet file: missing magic bytes"):
        _extract_parquet_footer("short.parquet")
