import json
from typing import Any, Dict, List, Optional

import pandas as pd
import pytest
from nptdms import TdmsFile, types  # type: ignore
from pytest_mock import MockFixture

from sift_py.data_import.tdms import TdmsTimeFormat, TdmsUploadService, sanitize_string
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
def mock_waveform_tdms_file():
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


@pytest.fixture
def mock_time_channel_tdms_file():
    mock_tdms_groups = [
        MockTdmsGroup(
            name=f"Group {g}",
            channels=[
                MockTdmsChannel(
                    name=f"Test/channel_{c}",
                    group_name=f"Group {g}",
                    data=[1, 2, 3],
                    properties={},
                )
                for c in range(5)
            ],
        )
        for g in range(5)
    ]

    # Add a Time channel to each group.
    for g in range(5):
        mock_tdms_groups[g].channels().append(
            MockTdmsChannel(
                name=f"Time {g}",
                group_name=f"Group {g}",
                data=[1, 2, 3],
                properties={},
                data_type=types.TimeStamp,
            )
        )

    return MockTdmsFile(mock_tdms_groups)


rest_config: SiftRestConfig = {
    "uri": "some_uri.com",
    "apikey": "123123123",
}


def test_sanitize_string(mocker: MockFixture):
    invalid_strings = [
        'Test"Channel',
        "Test\\Channel",
        "Test`Channel",
        "Test~Channel",
        "Test|Channel",
    ]
    for invalid_string in invalid_strings:
        assert sanitize_string(invalid_string) == "Test_Channel"


def test_tdms_upload_service_upload_validate_path(mocker: MockFixture):
    mock_path_is_file = mocker.patch("sift_py.data_import.tdms.Path.is_file")
    mock_path_is_file.return_value = False

    with pytest.raises(Exception, match="does not point to a regular file"):
        svc = TdmsUploadService(rest_config)
        svc.upload("some_data.tdms", "asset_name")


def test_waveform_tdms_upload_success(mocker: MockFixture, mock_waveform_tdms_file: MockTdmsFile):
    mock_path_is_file = mocker.patch("sift_py.data_import.tdms.Path.is_file")
    mock_path_is_file.return_value = True

    mock_path_getsize = mocker.patch("sift_py.data_import.csv.os.path.getsize")
    mock_path_getsize.return_value = 10

    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse()

    def mock_tdms_file_constructor(path):
        """The first call should always return the mocked object since
        it is mocking a call to open the orignal tdms file.

        The second call should return a real TdmsFile since the unit
        test will actually create one with filtered channels.
        """
        print(path)
        if path == "some_tdms.tdms":
            return mock_waveform_tdms_file
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
                "name": f"Test/channel_{j}",
                "data_type": "CHANNEL_DATA_TYPE_INT_32",
                "units": "",
                "description": "",
                "enum_types": [],
                "bit_field_elements": [],
            }
    assert config == expected_config

    # Test with grouping
    svc.upload("some_tdms.tdms", "asset_name", prefix_channel_with_group=True)
    config = get_csv_config(mock_requests_post, 2)
    for i in range(5):
        for j in range(5):
            name_with_group = (
                f"Group {i}.{expected_config['data_columns'][str(2 + (i * 5) + j)]['name']}"
            )
            expected_config["data_columns"][str(2 + (i * 5) + j)]["name"] = name_with_group
    assert config == expected_config

    # Test with run information and group_into_components deprecation
    with pytest.warns(FutureWarning, match="component"):
        svc.upload(
            "some_tdms.tdms",
            "asset_name",
            group_into_components=True,
            run_name="Run Name",
            run_id="",
        )
    expected_config["run_name"] = "Run Name"
    expected_config["run_id"] = ""
    config = get_csv_config(mock_requests_post, 4)
    assert config == expected_config


def test_time_channel_tdms_upload_success(
    mocker: MockFixture, mock_time_channel_tdms_file: MockTdmsFile
):
    mock_path_is_file = mocker.patch("sift_py.data_import.tdms.Path.is_file")
    mock_path_is_file.return_value = True

    mock_path_getsize = mocker.patch("sift_py.data_import.csv.os.path.getsize")
    mock_path_getsize.return_value = 10

    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse()

    def mock_tdms_file_constructor(path):
        """The first call should always return the mocked object since
        it is mocking a call to open the orignal tdms file.

        The second call should return a real TdmsFile since the unit
        test will actually create one with filtered channels.
        """
        return mock_time_channel_tdms_file

    mocker.patch("sift_py.data_import.tdms.TdmsFile", mock_tdms_file_constructor)

    csv_results = None

    def mock_writerows(_, row):
        nonlocal csv_results
        csv_results = row

    mocker.patch("sift_py.data_import.tdms.DictWriter.writerows", mock_writerows)

    svc = TdmsUploadService(rest_config)

    def get_csv_config(mock, n):
        """Return the CSV config that was created and uploaded under the hood."""
        return json.loads(mock_requests_post.call_args_list[n].kwargs["data"])["csv_config"]

    # Test without grouping
    svc.upload("some_tdms.tdms", "asset_name", tdms_time_format=TdmsTimeFormat.TIME_CHANNEL)
    config = get_csv_config(mock_requests_post, 0)
    expected_config: Dict[str, Any] = {
        "asset_name": "asset_name",
        "run_name": "",
        "run_id": "",
        "first_data_row": 2,
        "time_column": {
            "format": "TIME_FORMAT_ABSOLUTE_RFC3339",
            "column_number": 1,
            "relative_start_time": None,
        },
        "data_columns": {},
    }
    for i in range(5):
        for j in range(5):
            expected_config["data_columns"][str(2 + (i * 5) + j)] = {
                "name": f"Test/channel_{j}",
                "data_type": "CHANNEL_DATA_TYPE_INT_32",
                "units": "",
                "description": "",
                "enum_types": [],
                "bit_field_elements": [],
            }

    expected_csv_results = [
        {
            "Test/channel_0": 1,
            "Test/channel_1": 1,
            "Test/channel_2": 1,
            "Test/channel_3": 1,
            "Test/channel_4": 1,
            "Time": "1970-01-01T00:00:00.000000001Z",
        },
        {
            "Test/channel_0": 2,
            "Test/channel_1": 2,
            "Test/channel_2": 2,
            "Test/channel_3": 2,
            "Test/channel_4": 2,
            "Time": "1970-01-01T00:00:00.000000002Z",
        },
        {
            "Test/channel_0": 3,
            "Test/channel_1": 3,
            "Test/channel_2": 3,
            "Test/channel_3": 3,
            "Test/channel_4": 3,
            "Time": "1970-01-01T00:00:00.000000003Z",
        },
        {
            "Test/channel_0": 1,
            "Test/channel_1": 1,
            "Test/channel_2": 1,
            "Test/channel_3": 1,
            "Test/channel_4": 1,
            "Time": "1970-01-01T00:00:00.000000001Z",
        },
        {
            "Test/channel_0": 2,
            "Test/channel_1": 2,
            "Test/channel_2": 2,
            "Test/channel_3": 2,
            "Test/channel_4": 2,
            "Time": "1970-01-01T00:00:00.000000002Z",
        },
        {
            "Test/channel_0": 3,
            "Test/channel_1": 3,
            "Test/channel_2": 3,
            "Test/channel_3": 3,
            "Test/channel_4": 3,
            "Time": "1970-01-01T00:00:00.000000003Z",
        },
        {
            "Test/channel_0": 1,
            "Test/channel_1": 1,
            "Test/channel_2": 1,
            "Test/channel_3": 1,
            "Test/channel_4": 1,
            "Time": "1970-01-01T00:00:00.000000001Z",
        },
        {
            "Test/channel_0": 2,
            "Test/channel_1": 2,
            "Test/channel_2": 2,
            "Test/channel_3": 2,
            "Test/channel_4": 2,
            "Time": "1970-01-01T00:00:00.000000002Z",
        },
        {
            "Test/channel_0": 3,
            "Test/channel_1": 3,
            "Test/channel_2": 3,
            "Test/channel_3": 3,
            "Test/channel_4": 3,
            "Time": "1970-01-01T00:00:00.000000003Z",
        },
        {
            "Test/channel_0": 1,
            "Test/channel_1": 1,
            "Test/channel_2": 1,
            "Test/channel_3": 1,
            "Test/channel_4": 1,
            "Time": "1970-01-01T00:00:00.000000001Z",
        },
        {
            "Test/channel_0": 2,
            "Test/channel_1": 2,
            "Test/channel_2": 2,
            "Test/channel_3": 2,
            "Test/channel_4": 2,
            "Time": "1970-01-01T00:00:00.000000002Z",
        },
        {
            "Test/channel_0": 3,
            "Test/channel_1": 3,
            "Test/channel_2": 3,
            "Test/channel_3": 3,
            "Test/channel_4": 3,
            "Time": "1970-01-01T00:00:00.000000003Z",
        },
        {
            "Test/channel_0": 1,
            "Test/channel_1": 1,
            "Test/channel_2": 1,
            "Test/channel_3": 1,
            "Test/channel_4": 1,
            "Time": "1970-01-01T00:00:00.000000001Z",
        },
        {
            "Test/channel_0": 2,
            "Test/channel_1": 2,
            "Test/channel_2": 2,
            "Test/channel_3": 2,
            "Test/channel_4": 2,
            "Time": "1970-01-01T00:00:00.000000002Z",
        },
        {
            "Test/channel_0": 3,
            "Test/channel_1": 3,
            "Test/channel_2": 3,
            "Test/channel_3": 3,
            "Test/channel_4": 3,
            "Time": "1970-01-01T00:00:00.000000003Z",
        },
    ]
    assert config == expected_config
    assert csv_results == expected_csv_results

    # Test with grouping
    svc.upload(
        "some_tdms.tdms",
        "asset_name",
        prefix_channel_with_group=True,
        tdms_time_format=TdmsTimeFormat.TIME_CHANNEL,
    )
    config = get_csv_config(mock_requests_post, 2)
    for i in range(5):
        for j in range(5):
            name_with_group = (
                f"Group {i}.{expected_config['data_columns'][str(2 + (i * 5) + j)]['name']}"
            )
            expected_config["data_columns"][str(2 + (i * 5) + j)]["name"] = name_with_group
    assert config == expected_config
    assert csv_results == expected_csv_results

    # Test with run information and group_into_components deprecation
    with pytest.warns(FutureWarning, match="component"):
        svc.upload(
            "some_tdms.tdms",
            "asset_name",
            group_into_components=True,
            run_name="Run Name",
            run_id="",
            tdms_time_format=TdmsTimeFormat.TIME_CHANNEL,
        )
    expected_config["run_name"] = "Run Name"
    expected_config["run_id"] = ""
    config = get_csv_config(mock_requests_post, 4)
    assert config == expected_config
    assert csv_results == expected_csv_results


def test_waveform_tdms_upload_ignore_errors(mocker: MockFixture):
    mock_path_is_file = mocker.patch("sift_py.data_import.tdms.Path.is_file")
    mock_path_is_file.return_value = True

    mocker.patch("sift_py.data_import.tdms.TdmsWriter")

    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
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
        if path == "some_tdms.tdms":
            return mock_tdms_file
        else:
            return TdmsFile(path)

    mocker.patch("sift_py.data_import.tdms.TdmsFile", mock_tdms_file_constructor)

    svc = TdmsUploadService(rest_config)

    with pytest.raises(Exception, match="does not contain timing information"):
        svc.upload("some_tdms.tdms", "asset_name")

    with pytest.raises(Exception, match="No valid channels found in"):
        svc.upload("some_tdms.tdms", "asset_name", ignore_errors=True)


def test_time_channel_tdms_upload_ignore_errors(
    mocker: MockFixture, mock_time_channel_tdms_file: MockTdmsFile
):
    mock_path_is_file = mocker.patch("sift_py.data_import.tdms.Path.is_file")
    mock_path_is_file.return_value = True

    mocker.patch("sift_py.data_import.tdms.TdmsWriter")

    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse()

    # Start with all invalid groups with multiple time channels
    for g, group in enumerate(mock_time_channel_tdms_file.groups()):
        group.channels().append(
            MockTdmsChannel(
                name=f"Another Time Channel {g}",
                group_name=f"Group {g}",
                data=[1, 2, 3],
                properties={},
                data_type=types.TimeStamp,
            )
        )

    def mock_tdms_file_constructor2(path):
        return mock_time_channel_tdms_file

    mocker.patch("sift_py.data_import.tdms.TdmsFile", mock_tdms_file_constructor2)

    svc = TdmsUploadService(rest_config)

    with pytest.raises(Exception, match="contains more than one time channel"):
        svc.upload("some_tdms.tdms", "asset_name", tdms_time_format=TdmsTimeFormat.TIME_CHANNEL)

    with pytest.raises(Exception, match="No valid groups remaining"):
        svc.upload(
            "some_tdms.tdms",
            "asset_name",
            tdms_time_format=TdmsTimeFormat.TIME_CHANNEL,
            ignore_errors=True,
        )

    # Create a TDMS file with no time channels.
    no_time_channels_mock_tdms_file = MockTdmsFile(
        groups=[
            MockTdmsGroup(
                "Group 0",
                channels=[
                    MockTdmsChannel(
                        name="Test/channel_0",
                        group_name="Group 0",
                        properties={},
                        data=[1, 2, 3],
                        data_type=types.Int32,
                    )
                ],
            )
        ]
    )

    def mock_tdms_file_constructor(path):
        return no_time_channels_mock_tdms_file

    mocker.patch("sift_py.data_import.tdms.TdmsFile", mock_tdms_file_constructor)

    with pytest.raises(Exception, match="no time channels"):
        svc.upload("some_tdms.tdms", "asset_name", tdms_time_format=TdmsTimeFormat.TIME_CHANNEL)

    with pytest.raises(Exception, match="No valid groups remaining"):
        svc.upload(
            "some_tdms.tdms",
            "asset_name",
            tdms_time_format=TdmsTimeFormat.TIME_CHANNEL,
            ignore_errors=True,
        )


def test_tdms_upload_unknown_data_type(mocker: MockFixture, mock_waveform_tdms_file: MockTdmsFile):
    mock_path_is_file = mocker.patch("sift_py.data_import.tdms.Path.is_file")
    mock_path_is_file.return_value = True

    mocker.patch("sift_py.data_import.tdms.TdmsWriter")

    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse()

    mock_waveform_tdms_file.groups()[0].channels()[0].data_type = types.ComplexDoubleFloat
    mocker.patch("sift_py.data_import.tdms.TdmsFile").return_value = mock_waveform_tdms_file

    svc = TdmsUploadService(rest_config)

    with pytest.raises(Exception, match="data type not supported"):
        svc.upload("some_tdms.tdms", "asset_name")


def test_time_channel_tdms_different_lengths(
    mocker: MockFixture, mock_time_channel_tdms_file: MockTdmsFile
):
    mock_path_is_file = mocker.patch("sift_py.data_import.tdms.Path.is_file")
    mock_path_is_file.return_value = True

    mock_path_getsize = mocker.patch("sift_py.data_import.csv.os.path.getsize")
    mock_path_getsize.return_value = 10

    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse()

    # Create a TDMS file with channels that have different number of data points.
    mock_tdms_file = MockTdmsFile(
        groups=[
            MockTdmsGroup(
                "Group 0",
                channels=[
                    MockTdmsChannel(
                        name="Test/Time",
                        group_name="Group 0",
                        properties={},
                        data=[1, 2, 3, 4, 5, 6],
                        data_type=types.TimeStamp,
                    ),
                    MockTdmsChannel(
                        name="Test/channel_0",
                        group_name="Group 0",
                        properties={},
                        data=[1, 2, 3],
                        data_type=types.Int32,
                    ),
                    MockTdmsChannel(
                        name="Test/channel_1",
                        group_name="Group 0",
                        properties={},
                        data=[1, 2, 3, 4, 5, 6],
                        data_type=types.Int32,
                    ),
                ],
            ),
            MockTdmsGroup(
                "Group 1",
                channels=[
                    MockTdmsChannel(
                        name="Test/Time2",
                        group_name="Group 0",
                        properties={},
                        data=[1, 2, 3],
                        data_type=types.TimeStamp,
                    ),
                    MockTdmsChannel(
                        name="Test/channel2_0",
                        group_name="Group 0",
                        properties={},
                        data=[1, 2, 3],
                        data_type=types.Int32,
                    ),
                    MockTdmsChannel(
                        name="Test/channel2_1",
                        group_name="Group 0",
                        properties={},
                        data=[1, 2, 3],
                        data_type=types.Int32,
                    ),
                ],
            ),
        ]
    )

    def mock_tdms_file_constructor(path):
        return mock_tdms_file

    mocker.patch("sift_py.data_import.tdms.TdmsFile", mock_tdms_file_constructor)

    csv_results = None

    def mock_writerows(_, row):
        nonlocal csv_results
        csv_results = row

    mocker.patch("sift_py.data_import.tdms.DictWriter.writerows", mock_writerows)

    svc = TdmsUploadService(rest_config)

    svc.upload(
        "some_tdms.tdms",
        "asset_name",
        tdms_time_format=TdmsTimeFormat.TIME_CHANNEL,
        ignore_errors=True,
    )
    expected_csv_results = [
        {"Time": "1970-01-01T00:00:00.000000001Z", "Test/channel_1": 1},
        {"Time": "1970-01-01T00:00:00.000000002Z", "Test/channel_1": 2},
        {"Time": "1970-01-01T00:00:00.000000003Z", "Test/channel_1": 3},
        {"Time": "1970-01-01T00:00:00.000000004Z", "Test/channel_1": 4},
        {"Time": "1970-01-01T00:00:00.000000005Z", "Test/channel_1": 5},
        {"Time": "1970-01-01T00:00:00.000000006Z", "Test/channel_1": 6},
        {"Time": "1970-01-01T00:00:00.000000001Z", "Test/channel2_0": 1, "Test/channel2_1": 1},
        {"Time": "1970-01-01T00:00:00.000000002Z", "Test/channel2_0": 2, "Test/channel2_1": 2},
        {"Time": "1970-01-01T00:00:00.000000003Z", "Test/channel2_0": 3, "Test/channel2_1": 3},
    ]
    assert csv_results == expected_csv_results

    with pytest.raises(Exception, match="Length mismatch"):
        svc.upload("some_tdms.tdms", "asset_name", tdms_time_format=TdmsTimeFormat.TIME_CHANNEL)

    # Create a TDMS file with no valid channels
    mock_tdms_file = MockTdmsFile(
        groups=[
            MockTdmsGroup(
                "Group 0",
                channels=[
                    MockTdmsChannel(
                        name="Test/Time",
                        group_name="Group 0",
                        properties={},
                        data=[1],
                        data_type=types.TimeStamp,
                    ),
                    MockTdmsChannel(
                        name="Test/channel_0",
                        group_name="Group 0",
                        properties={},
                        data=[1, 2, 3],
                        data_type=types.Int32,
                    ),
                ],
            )
        ]
    )

    def mock_tdms_file_constructor2(path):
        return mock_tdms_file

    mocker.patch("sift_py.data_import.tdms.TdmsFile", mock_tdms_file_constructor2)

    with pytest.raises(Exception, match="No valid groups"):
        svc.upload(
            "some_tdms.tdms",
            "asset_name",
            tdms_time_format=TdmsTimeFormat.TIME_CHANNEL,
            ignore_errors=True,
        )
