import json

import pytest
from pytest_mock import MockFixture
from sift_py.data_import.status import DataImportStatus, DataImportStatusType
from sift_py.rest import SiftRestConfig

rest_config: SiftRestConfig = {
    "uri": "some_uri.com",
    "apikey": "123123123",
}


class MockResponse:
    status_code: int
    text: str

    def __init__(self, status_code: int, text: str):
        self.status_code = status_code
        self.text = text

    def json(self):
        return json.loads(self.text)

    def raise_for_status(self):
        if self.status_code != 200:
            raise Exception("Invalid status")


def test_get_status(mocker: MockFixture):
    mock_requests_post = mocker.patch("sift_py.data_import.status.requests.get")
    mock_requests_post.return_value = MockResponse(
        status_code=200, text=json.dumps({"dataImport": {"status": "DATA_IMPORT_STATUS_SUCCEEDED"}})
    )
    status = DataImportStatus(rest_config, "123-123-123")
    assert status.get_status() == DataImportStatusType.SUCCEEDED

    mock_requests_post.return_value = MockResponse(
        status_code=200, text=json.dumps({"dataImport": {"status": "DATA_IMPORT_STATUS_PENDING"}})
    )
    status = DataImportStatus(rest_config, "123-123-123")
    assert status.get_status() == DataImportStatusType.PENDING

    mock_requests_post.return_value = MockResponse(
        status_code=200,
        text=json.dumps({"dataImport": {"status": "DATA_IMPORT_STATUS_IN_PROGRESS"}}),
    )
    status = DataImportStatus(rest_config, "123-123-123")
    assert status.get_status() == DataImportStatusType.IN_PROGRESS

    mock_requests_post.return_value = MockResponse(
        status_code=200, text=json.dumps({"dataImport": {"status": "DATA_IMPORT_STATUS_FAILED"}})
    )
    status = DataImportStatus(rest_config, "123-123-123")
    assert status.get_status() == DataImportStatusType.FAILED

    with pytest.raises(Exception, match="Unknown data import status"):
        mock_requests_post.return_value = MockResponse(
            status_code=200, text=json.dumps({"dataImport": {"status": "INVALID_STATUS"}})
        )
        status = DataImportStatus(rest_config, "123-123-123")
        status.get_status()


def test_wait_success(mocker: MockFixture):
    mock_time_sleep = mocker.patch("sift_py.data_import.status.time.sleep")
    mock_requests_get = mocker.patch("sift_py.data_import.status.requests.get")
    mock_requests_get.side_effect = [
        MockResponse(
            status_code=200,
            text=json.dumps({"dataImport": {"status": "DATA_IMPORT_STATUS_PENDING"}}),
        ),
        MockResponse(
            status_code=200,
            text=json.dumps({"dataImport": {"status": "DATA_IMPORT_STATUS_IN_PROGRESS"}}),
        ),
        MockResponse(
            status_code=200,
            text=json.dumps({"dataImport": {"status": "DATA_IMPORT_STATUS_SUCCEEDED"}}),
        ),
    ]

    status = DataImportStatus(rest_config, "123-123-123")
    assert status.wait_until_complete() == True
    mock_time_sleep.assert_any_call(1)
    mock_time_sleep.assert_any_call(2)


def test_wait_failure(mocker: MockFixture):
    mock_requests_get = mocker.patch("sift_py.data_import.status.requests.get")
    mock_requests_get.side_effect = [
        MockResponse(
            status_code=200,
            text=json.dumps({"dataImport": {"status": "DATA_IMPORT_STATUS_PENDING"}}),
        ),
        MockResponse(
            status_code=200,
            text=json.dumps({"dataImport": {"status": "DATA_IMPORT_STATUS_IN_PROGRESS"}}),
        ),
        MockResponse(
            status_code=200,
            text=json.dumps({"dataImport": {"status": "DATA_IMPORT_STATUS_FAILED"}}),
        ),
    ]

    status = DataImportStatus(rest_config, "123-123-123")
    assert status.wait_until_complete() == False


def test_wait_max_polling_interval(mocker: MockFixture):
    mock_time_sleep = mocker.patch("sift_py.data_import.status.time.sleep")
    mock_requests_get = mocker.patch("sift_py.data_import.status.requests.get")
    mock_requests_get.side_effect = [
        MockResponse(
            status_code=200,
            text=json.dumps({"dataImport": {"status": "DATA_IMPORT_STATUS_IN_PROGRESS"}}),
        )
        for _ in range(60)
    ] + [
        MockResponse(
            status_code=200,
            text=json.dumps({"dataImport": {"status": "DATA_IMPORT_STATUS_SUCCEEDED"}}),
        )
    ]

    status = DataImportStatus(rest_config, "123-123-123")
    assert status.wait_until_complete() == True
    mock_time_sleep.assert_called_with(60)
