import json
from typing import Any, Dict, List, Optional

import pandas as pd
import pytest
from nptdms import TdmsFile, types  # type: ignore
from pytest_mock import MockFixture

from sift_py.data_import.tdms import TdmsUploadService
from sift_py.rest import SiftRestConfig


class MockTdmsChannel:
    def __init__(
        self,
        name: str,
        group_name: str,
        properties: Optional[Dict[str, Any]] = None,
        data: Optional[List[int]] = None,
        data_type: type = types.Int32,
    ):
        self.name: str = name
        self.group_name: str = group_name
        self.properties: Optional[Dict[str, str]] = properties or {}
        self.data: Optional[List[int]] = data or []
        self.data_type: type = data_type


class MockTdmsGroup:
    def __init__(self, name, channels: List[MockTdmsChannel]):
        self.name: str = name
        self.path: str = f"/'{name}'"
        self._channels: List[MockTdmsChannel] = channels
        self.properties: Optional[Dict[str, str]] = {}

    def channels(self) -> List[MockTdmsChannel]:
        return self._channels


class MockTdmsFile:
    def __init__(self, groups: List[MockTdmsGroup]):
        self._groups: List[MockTdmsGroup] = groups
        self.properties: Dict[str, str] = {}

    def groups(self) -> List[MockTdmsGroup]:
        return self._groups

    def as_dataframe(self, *_, **__):
        return pd.DataFrame()


class MockResponse:
    def __init__(self):
        self.status_code = 200
        self.text = json.dumps({"uploadUrl": "some_url.com", "dataImportId": "123-123-123"})

    def json(self) -> dict:
        return json.loads(self.text)


@pytest.fixture
def mock_tdms_file():
    mock_tdms_groups = [
        MockTdmsGroup(
            name=f"Group {g}",
            channels=[
                MockTdmsChannel(
                    name=f"Test/channel_{c}",
                    group_name=f"Group {g}",
                    data=[1, 2, 3],
                    properties={
                        "wf_start_time": 0,
                        "wf_increment": 0.1,
                        "wf_start_offset": 0,
                        "extra": "info",
                    },
                )
                for c in range(5)
            ],
        )
        for g in range(5)
    ]

    return MockTdmsFile(mock_tdms_groups)


rest_config: SiftRestConfig = {
    "uri": "some_uri.com",
    "apikey": "123123123",
}


def test_tdms_upload_service_upload_validate_path(mocker: MockFixture):
    mock_path_is_file = mocker.patch("sift_py.data_import.tdms.Path.is_file")
    mock_path_is_file.return_value = False

    with pytest.raises(Exception, match="does not point to a regular file"):
        svc = TdmsUploadService(rest_config)
        svc.upload("some_data.tdms", "asset_name")


def test_tdms_upload_success(mocker: MockFixture, mock_tdms_file: MockTdmsFile):
    mock_path_is_file = mocker.patch("sift_py.data_import.tdms.Path.is_file")
    mock_path_is_file.return_value = True

    mock_requests_post = mocker.patch("sift_py.data_import.csv.requests.post")
    mock_requests_post.return_value = MockResponse()

    def mock_tdms_file_constructor(path):
        """The first call should always return the mocked object since
        it is mocking a call to open the orignal tdms file.

        The second call should return a real TdmsFile since the unit
        test will actually create one with filtered channels.
        """
        print(path)
        if path == "some_tdms.tdms":
            return mock_tdms_file
        else:
            return TdmsFile(path)

    mocker.patch("sift_py.data_import.tdms.TdmsFile", mock_tdms_file_constructor)

    svc = TdmsUploadService(rest_config)

    def get_csv_config(mock, n):
        """Return the CSV config that was created and uploaded under the hood."""
        return json.loads(mock_requests_post.call_args_list[n].kwargs["data"])["csv_config"]

    # Test without grouping
    svc.upload("some_tdms.tdms", "asset_name")
    config = get_csv_config(mock_requests_post, 0)
    expected_config: Dict[str, Any] = {
        "asset_name": "asset_name",
        "run_name": "",
        "run_id": "",
        "first_data_row": 2,
        "time_column": {
            "format": "TIME_FORMAT_ABSOLUTE_DATETIME",
            "column_number": 1,
            "relative_start_time": None,
        },
        "data_columns": {},
    }
    for i in range(5):
        for j in range(5):
            expected_config["data_columns"][str(2 + (i * 5) + j)] = {
                "name": f"Test channel_{j}",
                "data_type": "CHANNEL_DATA_TYPE_INT_32",
                "component": "",
                "units": "",
                "description": "None\nwf_start_time: 0\nwf_increment: 0.1\nwf_start_offset: 0\nextra: info\n",
                "enum_types": [],
                "bit_field_elements": [],
            }
    assert config == expected_config

    # Test with grouping
    svc.upload("some_tdms.tdms", "asset_name", group_into_components=True)
    config = get_csv_config(mock_requests_post, 2)
    for i in range(5):
        for j in range(5):
            expected_config["data_columns"][str(2 + (i * 5) + j)]["component"] = f"Group {i}"
    assert config == expected_config

    # Test with run information
    svc.upload(
        "some_tdms.tdms",
        "asset_name",
        group_into_components=True,
        run_name="Run Name",
        run_id="Run ID",
    )
    expected_config["run_name"] = "Run Name"
    expected_config["run_id"] = "Run ID"
    config = get_csv_config(mock_requests_post, 4)
    assert config == expected_config


def test_tdms_upload_ignore_errors(mocker: MockFixture):
    mock_path_is_file = mocker.patch("sift_py.data_import.tdms.Path.is_file")
    mock_path_is_file.return_value = True

    mocker.patch("sift_py.data_import.tdms.TdmsWriter")

    mock_requests_post = mocker.patch("sift_py.data_import.csv.requests.post")
    mock_requests_post.return_value = MockResponse()

    # Start with all invalid channels
    mock_tdms_groups = [
        MockTdmsGroup(
            name=f"Group {g}",
            channels=[MockTdmsChannel(f"channel_{c}", f"Group {g}") for c in range(5)],
        )
        for g in range(5)
    ]
    mock_tdms_file = MockTdmsFile(mock_tdms_groups)

    def mock_tdms_file_constructor(path):
        """The first call should always return the mocked object since
        it is mocking a call to open the orignal tdms file.

        The second call should return a real TdmsFile since the unit
        test will actually create one with filtered channels.
        """
        print(path)
        if path == "some_tdms.tdms":
            return mock_tdms_file
        else:
            return TdmsFile(path)

    mocker.patch("sift_py.data_import.tdms.TdmsFile", mock_tdms_file_constructor)

    svc = TdmsUploadService(rest_config)

    with pytest.raises(Exception, match="does not contain timing information"):
        svc.upload("some_tdms.tdms", "asset_name")

    with pytest.raises(Exception, match="No valid channels remaining"):
        svc.upload("some_tdms.tdms", "asset_name", ignore_errors=True)


def test_tdms_upload_unknown_data_type(mocker: MockFixture, mock_tdms_file: MockTdmsFile):
    mock_path_is_file = mocker.patch("sift_py.data_import.tdms.Path.is_file")
    mock_path_is_file.return_value = True

    mocker.patch("sift_py.data_import.tdms.TdmsWriter")

    mock_requests_post = mocker.patch("sift_py.data_import.csv.requests.post")
    mock_requests_post.return_value = MockResponse()

    mock_tdms_file.groups()[0].channels()[0].data_type = types.ComplexDoubleFloat
    mocker.patch("sift_py.data_import.tdms.TdmsFile").return_value = mock_tdms_file

    svc = TdmsUploadService(rest_config)

    with pytest.raises(Exception, match="data type not supported"):
        svc.upload("some_tdms.tdms", "asset_name")
