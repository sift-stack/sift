import json
from copy import deepcopy

import pytest
from pytest_mock import MockFixture

from sift_py.data_import.status import DataImportService, DataImportStatusType
from sift_py.rest import SiftRestConfig

rest_config: SiftRestConfig = {
    "uri": "some_uri.com",
    "apikey": "123123123",
}


@pytest.fixture
def data_import_data():
    return {
        "dataImport": {
            "dataImportId": "random-data-import-id",
            "createdDate": "2024-10-07T18:37:00.146649Z",
            "modifiedDate": "2024-10-07T18:37:00.146649Z",
            "sourceUrl": "",
            "status": "",
            "errorMessage": "",
            "csvConfig": {},
        }
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


def test_get_status(mocker: MockFixture, data_import_data: dict):
    mock_session = mocker.patch("sift_py.rest.requests.Session", autospec=True)
    mock_requests_get = mock_session.return_value.get

    data_import_data["dataImport"]["status"] = "DATA_IMPORT_STATUS_SUCCEEDED"
    mock_requests_get.return_value = MockResponse(
        status_code=200, text=json.dumps(data_import_data)
    )
    service = DataImportService(rest_config, "123-123-123")
    assert service.get_data_import().status == DataImportStatusType.SUCCEEDED

    data_import_data["dataImport"]["status"] = "DATA_IMPORT_STATUS_PENDING"
    mock_requests_get.return_value = MockResponse(
        status_code=200, text=json.dumps(data_import_data)
    )
    service = DataImportService(rest_config, "123-123-123")
    assert service.get_data_import().status == DataImportStatusType.PENDING

    data_import_data["dataImport"]["status"] = "DATA_IMPORT_STATUS_IN_PROGRESS"
    mock_requests_get.return_value = MockResponse(
        status_code=200, text=json.dumps(data_import_data)
    )
    service = DataImportService(rest_config, "123-123-123")
    assert service.get_data_import().status == DataImportStatusType.IN_PROGRESS

    data_import_data["dataImport"]["status"] = "DATA_IMPORT_STATUS_FAILED"
    mock_requests_get.return_value = MockResponse(
        status_code=200, text=json.dumps(data_import_data)
    )
    service = DataImportService(rest_config, "123-123-123")
    assert service.get_data_import().status == DataImportStatusType.FAILED

    data_import_data["dataImport"]["status"] = "INVALID_STATUS"
    with pytest.raises(Exception, match="Invalid data import status"):
        mock_requests_get.return_value = MockResponse(
            status_code=200, text=json.dumps(data_import_data)
        )
        service = DataImportService(rest_config, "123-123-123")
        service.get_data_import()


def test_wait_success(mocker: MockFixture, data_import_data: dict):
    mock_time_sleep = mocker.patch("sift_py.data_import.status.time.sleep")
    mock_session = mocker.patch("sift_py.rest.requests.Session", autospec=True)
    mock_requests_get = mock_session.return_value.get

    succeeded = deepcopy(data_import_data)
    succeeded["dataImport"]["status"] = "DATA_IMPORT_STATUS_SUCCEEDED"

    pending = deepcopy(data_import_data)
    pending["dataImport"]["status"] = "DATA_IMPORT_STATUS_PENDING"

    in_progress = deepcopy(data_import_data)
    in_progress["dataImport"]["status"] = "DATA_IMPORT_STATUS_IN_PROGRESS"

    mock_requests_get.side_effect = [
        MockResponse(
            status_code=200,
            text=json.dumps(pending),
        ),
        MockResponse(
            status_code=200,
            text=json.dumps(in_progress),
        ),
        MockResponse(
            status_code=200,
            text=json.dumps(succeeded),
        ),
    ]

    service = DataImportService(rest_config, "123-123-123")
    assert service.wait_until_complete().status == DataImportStatusType.SUCCEEDED
    mock_time_sleep.assert_any_call(1)
    mock_time_sleep.assert_any_call(2)


def test_wait_failure(mocker: MockFixture, data_import_data: dict):
    mock_session = mocker.patch("sift_py.rest.requests.Session", autospec=True)
    mock_requests_get = mock_session.return_value.get

    failed = deepcopy(data_import_data)
    failed["dataImport"]["status"] = "DATA_IMPORT_STATUS_FAILED"

    pending = deepcopy(data_import_data)
    pending["dataImport"]["status"] = "DATA_IMPORT_STATUS_PENDING"

    in_progress = deepcopy(data_import_data)
    in_progress["dataImport"]["status"] = "DATA_IMPORT_STATUS_IN_PROGRESS"

    mock_requests_get.side_effect = [
        MockResponse(
            status_code=200,
            text=json.dumps(pending),
        ),
        MockResponse(
            status_code=200,
            text=json.dumps(in_progress),
        ),
        MockResponse(
            status_code=200,
            text=json.dumps(failed),
        ),
    ]

    service = DataImportService(rest_config, "123-123-123")
    assert service.wait_until_complete().status == DataImportStatusType.FAILED


def test_wait_max_polling_interval(mocker: MockFixture, data_import_data: dict):
    mock_time_sleep = mocker.patch("sift_py.data_import.status.time.sleep")
    mock_session = mocker.patch("sift_py.rest.requests.Session", autospec=True)
    mock_requests_get = mock_session.return_value.get

    succeeded = deepcopy(data_import_data)
    succeeded["dataImport"]["status"] = "DATA_IMPORT_STATUS_SUCCEEDED"

    in_progress = deepcopy(data_import_data)
    in_progress["dataImport"]["status"] = "DATA_IMPORT_STATUS_IN_PROGRESS"

    mock_requests_get.side_effect = [
        MockResponse(
            status_code=200,
            text=json.dumps(in_progress),
        )
        for _ in range(60)
    ] + [
        MockResponse(
            status_code=200,
            text=json.dumps(succeeded),
        )
    ]

    service = DataImportService(rest_config, "123-123-123")
    assert service.wait_until_complete().status == DataImportStatusType.SUCCEEDED
    mock_time_sleep.assert_called_with(60)
