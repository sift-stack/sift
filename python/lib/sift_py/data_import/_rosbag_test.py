import json
import unittest
from unittest.mock import Mock

import pytest
from pytest_mock import MockFixture
from rosbags.interfaces import Nodetype
from rosbags.typesys import Stores
from rosbags.typesys.store import Typestore

from sift_py.data_import._ros_channel import RosChannel
from sift_py.data_import.rosbags import RosbagsUploadService
from sift_py.ingestion.channel import ChannelDataType
from sift_py.rest import SiftRestConfig


class TestRosChannel(unittest.TestCase):
    def setUp(self):
        self.typestore = Mock(spec=Typestore)

    def test_extract_value_base_type(self):
        msg = Mock()
        msg.my_field = 42
        ros_channel = RosChannel(
            field_name="my_field",
            channel_name="channel_name",
            node_type=Nodetype.BASE,
            data_type=ChannelDataType.INT_32,
        )
        self.assertEqual(ros_channel.extract_value(msg), 42)

    def test_extract_value_with_parent_field(self):
        msg = Mock()
        my_parent_field = Mock()
        my_parent_field.child_field = 42
        msg.my_parent_field = my_parent_field
        ros_channel = RosChannel(
            field_name="child_field",
            channel_name="channel_name",
            node_type=Nodetype.BASE,
            data_type=ChannelDataType.INT_32,
            parent_fields=("my_parent_field",),
        )
        self.assertEqual(ros_channel.extract_value(msg), 42)

    def test_extract_value_with_nested_parent_field(self):
        msg = Mock()
        my_parent_field1 = Mock()
        my_parent_field1.child_field = 42
        msg.my_parent_field1 = my_parent_field1

        my_parent_field2 = Mock()
        my_parent_field2.child_field = 7
        msg.my_parent_field1.my_parent_field2 = my_parent_field2

        ros_channel = RosChannel(
            field_name="child_field",
            channel_name="channel_name",
            node_type=Nodetype.BASE,
            data_type=ChannelDataType.INT_32,
            parent_fields=("my_parent_field1", "my_parent_field2"),
        )
        self.assertEqual(ros_channel.extract_value(msg), 7)

    def test_extract_value_with_index(self):
        msg = Mock()
        msg.my_field = [1, 2, 3]
        ros_channels = [
            RosChannel(
                field_name="my_field",
                channel_name="channel_name",
                node_type=Nodetype.BASE,
                data_type=ChannelDataType.INT_32,
                index=i,
            )
            for i in range(3)
        ]
        self.assertEqual(ros_channels[0].extract_value(msg), 1)
        self.assertEqual(ros_channels[1].extract_value(msg), 2)
        self.assertEqual(ros_channels[2].extract_value(msg), 3)

    def test_extract_value_unsupported_node_type(self):
        msg = Mock()
        ros_channel = RosChannel(
            field_name="my_field",
            channel_name="channel_name",
            node_type=Nodetype.NAME,
            data_type=ChannelDataType.INT_32,
        )
        with self.assertRaisesRegex(Exception, "Unsupported node type"):
            ros_channel.extract_value(msg)

    def test_get_underlying_fields_base_type(self):
        field = ("field_name", (Nodetype.BASE, ("int32", 0)))
        channels = RosChannel.get_underlying_fields("my_prefix", field, self.typestore)
        self.assertEqual(len(channels), 1)
        self.assertEqual(channels[0].field_name, "field_name")
        self.assertEqual(channels[0].channel_name, "my_prefix.field_name")
        self.assertEqual(channels[0].node_type, Nodetype.BASE)
        self.assertEqual(channels[0].data_type, ChannelDataType.INT_32)

    def test_get_underlying_fields_array_type(self):
        field = ("field_name", (Nodetype.ARRAY, ((Nodetype.BASE, ("int32", 0)), 3)))
        channels = RosChannel.get_underlying_fields("my_prefix", field, self.typestore)
        self.assertEqual(len(channels), 3)
        for i in range(3):
            self.assertEqual(channels[i].field_name, "field_name")
            self.assertEqual(channels[i].channel_name, f"my_prefix.field_name[{i}]")
            self.assertEqual(channels[i].node_type, Nodetype.BASE)
            self.assertEqual(channels[i].data_type, ChannelDataType.INT_32)

    def test_get_underlying_fields_nested_type(self):
        sub_field1 = ("sub_field1", (Nodetype.BASE, ("int8", 0)))
        sub_field2 = ("sub_field2", (Nodetype.BASE, ("float64", 0)))
        sub_field3 = ("sub_field3", (Nodetype.ARRAY, ((Nodetype.BASE, ("float64", 0)), 3)))
        self.typestore.get_msgdef.return_value.fields = [sub_field1, sub_field2, sub_field3]
        field = ("field_name", (Nodetype.NAME, "some_msg_type"))
        channels = RosChannel.get_underlying_fields("my_prefix", field, self.typestore)
        self.assertEqual(len(channels), 5)
        self.assertEqual(channels[0].field_name, "sub_field1")
        self.assertEqual(channels[0].channel_name, "my_prefix.field_name.sub_field1")
        self.assertEqual(channels[0].node_type, Nodetype.BASE)
        self.assertEqual(channels[0].data_type, ChannelDataType.INT_32)

        self.assertEqual(channels[1].field_name, "sub_field2")
        self.assertEqual(channels[1].channel_name, "my_prefix.field_name.sub_field2")
        self.assertEqual(channels[1].node_type, Nodetype.BASE)
        self.assertEqual(channels[1].data_type, ChannelDataType.DOUBLE)

        self.assertEqual(channels[2].field_name, "sub_field3")
        self.assertEqual(channels[2].channel_name, "my_prefix.field_name.sub_field3[0]")
        self.assertEqual(channels[2].node_type, Nodetype.BASE)
        self.assertEqual(channels[2].data_type, ChannelDataType.DOUBLE)

        self.assertEqual(channels[3].field_name, "sub_field3")
        self.assertEqual(channels[3].channel_name, "my_prefix.field_name.sub_field3[1]")
        self.assertEqual(channels[3].node_type, Nodetype.BASE)
        self.assertEqual(channels[3].data_type, ChannelDataType.DOUBLE)

        self.assertEqual(channels[4].field_name, "sub_field3")
        self.assertEqual(channels[4].channel_name, "my_prefix.field_name.sub_field3[2]")
        self.assertEqual(channels[4].node_type, Nodetype.BASE)
        self.assertEqual(channels[4].data_type, ChannelDataType.DOUBLE)


rest_config: SiftRestConfig = {
    "uri": "some_uri.com",
    "apikey": "123123123",
}


class MockResponse:
    def __init__(self):
        self.status_code = 200
        self.text = json.dumps({"uploadUrl": "some_url.com", "dataImportId": "123-123-123"})

    def json(self) -> dict:
        return json.loads(self.text)


class MockReader:
    def __init__(self, connections, messages):
        self.msgs = messages
        self.message_count = len(messages)
        self.connections = connections

    def messages(self):
        return self.msgs

    def __enter__(self):
        return self

    def __exit__(self, *args):
        pass


class MockConnection:
    def __init__(self, topic, msgtype):
        self.topic = topic
        self.msgtype = msgtype


class MessageA:
    name = "msgs/MessageA"
    fields = [
        ("x", (Nodetype.BASE, ("int32", 0))),
        ("y", (Nodetype.BASE, ("float64", 0))),
    ]

    def __init__(self, x, y):
        self.x = x
        self.y = y


class MessageB:
    name = "msgs/MessageB"
    fields = [
        ("x", (Nodetype.BASE, ("string", 0))),
        ("message_a", (Nodetype.NAME, "msgs/MessageA")),
    ]

    def __init__(self, x, a):
        self.x = x
        self.message_a = a


class MockDeserializeCdr:
    def __init__(self, messages):
        self.count = 0
        self.messages = messages

    def __call__(self, *args, **kwargs):
        msg = self.messages[self.count]
        self.count += 1
        return msg[2]


def test_rosbag_upload_service_upload_validate_path(mocker: MockFixture):
    mock_path_is_file = mocker.patch("sift_py.data_import.rosbags.Path.is_dir")
    mock_path_is_file.return_value = False

    mock_path_exists = mocker.patch("sift_py.data_import.rosbags.Path.exists")
    mock_path_exists.return_value = True

    with pytest.raises(Exception, match="does not point to a directory"):
        svc = RosbagsUploadService(rest_config)
        svc.upload("some_data", ["msgs"], Stores.ROS2_HUMBLE, "asset_name")


def test_rosbag_upload_service_unknown_data_type(mocker: MockFixture):
    mock_path_is_file = mocker.patch("sift_py.data_import.rosbags.Path.is_dir")
    mock_path_is_file.return_value = True

    mock_path_exists = mocker.patch("sift_py.data_import.rosbags.Path.exists")
    mock_path_exists.return_value = True

    def mock_register_types(*args, **kwargs):
        return {}

    mocker.patch(
        "sift_py.data_import.rosbags.RosbagsUploadService._register_types", mock_register_types
    )

    mock_reader = mocker.patch("sift_py.data_import.rosbags.Reader")
    mock_reader.return_value = MockReader(
        [
            MockConnection("a/b/c/topic1", "msgs/MessageA"),
        ],
        [],
    )

    svc = RosbagsUploadService(rest_config)
    with pytest.raises(Exception, match="not found in custom types"):
        svc.upload("some_data", ["msgs"], Stores.ROS2_HUMBLE, "asset_name")

    svc = RosbagsUploadService(rest_config)
    with pytest.raises(Exception, match="No valid channels remaining"):
        svc.upload("some_data", ["msgs"], Stores.ROS2_HUMBLE, "asset_name", ignore_errors=True)


def test_rosbag_upload_success(mocker: MockFixture):
    mock_path_is_file = mocker.patch("sift_py.data_import.rosbags.Path.is_dir")
    mock_path_is_file.return_value = True

    mock_path_exists = mocker.patch("sift_py.data_import.rosbags.Path.exists")
    mock_path_exists.return_value = True

    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse()

    def mock_register_types(*args, **kwargs):
        return {"msgs/MessageA", "msgs/MessageB"}

    mocker.patch(
        "sift_py.data_import.rosbags.RosbagsUploadService._register_types", mock_register_types
    )

    def mock_get_msgdef(self, typename):
        return {
            "msgs/MessageA": MessageA,
            "msgs/MessageB": MessageB,
        }[typename]

    mocker.patch("sift_py.data_import.rosbags.Typestore.get_msgdef", mock_get_msgdef)

    connection1 = MockConnection("a/b/c/topic1", "msgs/MessageA")
    connection2 = MockConnection("topic2", "msgs/MessageB")
    timestamp = 1234567890
    messages = [
        (connection1, timestamp, MessageA(1, 2)),
        (connection2, timestamp, MessageB("Hello world", MessageA(4, 5))),
    ]
    mock_reader = mocker.patch("sift_py.data_import.rosbags.Reader")
    mock_reader.return_value = MockReader(
        [
            connection1,
            connection2,
        ],
        messages,
    )

    mocker.patch(
        "sift_py.data_import.rosbags.Typestore.deserialize_cdr", MockDeserializeCdr(messages)
    )

    csv_results = []

    def mock_writerow(self, row):
        csv_results.append(row)

    mocker.patch("sift_py.data_import.rosbags.csv.DictWriter.writerow", mock_writerow)

    svc = RosbagsUploadService(rest_config)

    def get_csv_config(mock, n):
        """Return the CSV config that was created and uploaded under the hood."""
        return json.loads(mock_requests_post.call_args_list[n].kwargs["data"])["csv_config"]

    # Test without grouping
    svc.upload("some_data", ["msgs"], Stores.ROS2_HUMBLE, "asset_name")
    config = get_csv_config(mock_requests_post, 0)

    expected_config = {
        "asset_name": "asset_name",
        "run_name": "",
        "run_id": "",
        "first_data_row": 2,
        "time_column": {
            "format": "TIME_FORMAT_ABSOLUTE_UNIX_NANOSECONDS",
            "column_number": 1,
            "relative_start_time": None,
        },
        "data_columns": {
            "2": {
                "name": "a_b_c_topic1.x",
                "data_type": "CHANNEL_DATA_TYPE_INT_32",
                "units": "",
                "description": "",
                "enum_types": [],
                "bit_field_elements": [],
            },
            "3": {
                "name": "a_b_c_topic1.y",
                "data_type": "CHANNEL_DATA_TYPE_DOUBLE",
                "units": "",
                "description": "",
                "enum_types": [],
                "bit_field_elements": [],
            },
            "4": {
                "name": "topic2.x",
                "data_type": "CHANNEL_DATA_TYPE_STRING",
                "units": "",
                "description": "",
                "enum_types": [],
                "bit_field_elements": [],
            },
            "5": {
                "name": "topic2.message_a.x",
                "data_type": "CHANNEL_DATA_TYPE_INT_32",
                "units": "",
                "description": "",
                "enum_types": [],
                "bit_field_elements": [],
            },
            "6": {
                "name": "topic2.message_a.y",
                "data_type": "CHANNEL_DATA_TYPE_DOUBLE",
                "units": "",
                "description": "",
                "enum_types": [],
                "bit_field_elements": [],
            },
        },
    }
    assert config == expected_config

    expected_rows = [
        {
            "time": "time",
            "a_b_c_topic1.x": "a_b_c_topic1.x",
            "a_b_c_topic1.y": "a_b_c_topic1.y",
            "topic2.x": "topic2.x",
            "topic2.message_a.x": "topic2.message_a.x",
            "topic2.message_a.y": "topic2.message_a.y",
        },
        {"time": 1234567890, "a_b_c_topic1.x": 1, "a_b_c_topic1.y": 2},
        {
            "time": 1234567890,
            "topic2.x": "Hello world",
            "topic2.message_a.x": 4,
            "topic2.message_a.y": 5,
        },
    ]
    assert csv_results == expected_rows


def test_rosbag_handlers(mocker: MockFixture):
    mock_path_is_file = mocker.patch("sift_py.data_import.rosbags.Path.is_dir")
    mock_path_is_file.return_value = True

    mock_path_exists = mocker.patch("sift_py.data_import.rosbags.Path.exists")
    mock_path_exists.return_value = True

    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse()

    def mock_register_types(*args, **kwargs):
        return {"msgs/MessageA", "msgs/MessageB"}

    mocker.patch(
        "sift_py.data_import.rosbags.RosbagsUploadService._register_types", mock_register_types
    )

    def mock_get_msgdef(self, typename):
        return {
            "msgs/MessageA": MessageA,
            "msgs/MessageB": MessageB,
        }[typename]

    mocker.patch("sift_py.data_import.rosbags.Typestore.get_msgdef", mock_get_msgdef)

    connection1 = MockConnection("a/b/c/topic1", "msgs/MessageA")
    connection2 = MockConnection("topic2", "msgs/MessageB")
    timestamp = 1234567890
    msg_a = MessageA(1, 2)
    msg_b = MessageB("Hello world", MessageA(4, 5))
    messages = [
        (connection1, timestamp, msg_a),
        (connection2, timestamp, msg_b),
    ]
    mock_reader = mocker.patch("sift_py.data_import.rosbags.Reader")
    mock_reader.return_value = MockReader(
        [
            connection1,
            connection2,
        ],
        messages,
    )

    mocker.patch(
        "sift_py.data_import.rosbags.Typestore.deserialize_cdr", MockDeserializeCdr(messages)
    )

    svc = RosbagsUploadService(rest_config)

    def handler1(topic, timestamp1, msg):
        assert topic == "a/b/c/topic1"
        assert timestamp1 == 1234567890
        assert msg == msg_a

    def handler2(topic, timestamp2, msg):
        assert topic == "topic2"
        assert timestamp2 == 1234567890
        assert msg == msg_b

    handlers = {
        "a/b/c/topic1": handler1,
        "topic2": handler2,
    }
    svc.upload("some_data", ["msgs"], Stores.ROS2_HUMBLE, "asset_name", handlers=handlers)
