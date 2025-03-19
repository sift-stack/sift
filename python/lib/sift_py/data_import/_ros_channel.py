from typing import Optional, Tuple, Union, cast

from rosbags.interfaces import Nodetype  # type: ignore
from rosbags.typesys.store import Typestore  # type: ignore

from sift_py.ingestion.channel import ChannelDataType


class RosChannel:
    """Helper class to process message fields and extract channel values from messages."""

    ROS_TO_SIFT_TYPES = {
        "bool": ChannelDataType.BOOL,
        "int8": ChannelDataType.INT_32,
        "int16": ChannelDataType.INT_32,
        "int32": ChannelDataType.INT_32,
        "int64": ChannelDataType.INT_64,
        "uint8": ChannelDataType.UINT_32,
        "uint16": ChannelDataType.UINT_32,
        "uint32": ChannelDataType.UINT_32,
        "uint64": ChannelDataType.UINT_64,
        "float32": ChannelDataType.FLOAT,
        "float64": ChannelDataType.DOUBLE,
        "string": ChannelDataType.STRING,
    }

    node_type: Nodetype
    data_type: ChannelDataType
    field_name: str
    channel_name: str
    index: Optional[int]
    parent_fields: Optional[Tuple[str]]

    def __init__(
        self,
        field_name: str,
        channel_name: str,
        node_type: Nodetype,
        data_type: ChannelDataType,
        index: Optional[int] = None,
        parent_fields: Optional[Tuple[str]] = None,
    ):
        """Constructor.

        Args:
            field_name: The name of this field (e.g, 'x', 'speed').
            channel_name: The fully qualifies name (e.g, 'abc.geometry.position.x').
            node_type: The ROS node type.
            data_type: The data type for this field.
            index: The index of this field if this is an array.
            parent_fields: Tuple of parent fields for nested fields.
        """
        self.node_type = node_type
        self.data_type = data_type
        self.channel_name = channel_name
        self.field_name = field_name
        self.index = index
        self.parent_fields = parent_fields

    def extract_value(self, msg) -> Union[bool, int, float, str]:
        """Extract the value of the channel from the message.

        Args:
            msg: The message to extract the value from.

        Returns:
            The extracted value.
        """
        if self.node_type == Nodetype.BASE:
            if self.parent_fields is not None:
                for parent in self.parent_fields:
                    msg = getattr(msg, parent)

            value = getattr(msg, self.field_name)

            if self.index is None:
                return value
            else:
                return value[self.index]
        else:
            raise Exception(f"Unsupported node type: {self.channel_name} {self.node_type.name}")

    @staticmethod
    def get_underlying_fields(
        prefix,
        field: tuple,
        typestore: Typestore,
        index: Optional[int] = None,
        parent_fields: Optional[Tuple[str]] = None,
    ):
        """Recursively extract fields from a message definition.

        Args:
            prefix: The prefix to prepend to the channel name.
            field: The field information.
            typestore: The typestore to use for message definitions.
            index: The index of the field if it is an array.
            parent_fields: The parent field name if the field is nested.

        Returns:
            A list of RosChannel objects.
        """
        field_name: str = field[0]
        channel_name = field_name
        if index is not None:
            channel_name = f"{channel_name}[{index}]"
        if parent_fields is not None:
            parent_channel_name = ".".join(parent_fields)
            channel_name = f"{parent_channel_name}.{channel_name}"
        channel_name = f"{prefix}.{channel_name}"

        node_type: Nodetype = field[1][0]
        # Example of `field` for single value:
        # ('timestamp', (<Nodetype.BASE: 1>, ('uint64', 0)))
        # This is the base case and can return a single RosChannel object.
        if node_type == Nodetype.BASE:
            ros_type = field[1][1][0]
            if ros_type not in RosChannel.ROS_TO_SIFT_TYPES:
                raise Exception(f"Data type {ros_type} not supported for field: {channel_name}")
            data_type = RosChannel.ROS_TO_SIFT_TYPES[ros_type]
            return [
                RosChannel(field_name, channel_name, node_type, data_type, index, parent_fields)
            ]
        #  Example of `field` for an array value:
        # ('position', (<Nodetype.ARRAY: 3>, ((<Nodetype.BASE: 1>, ('float32', 0)), 3)))
        # Each element in the array is a separate RosChannel.
        elif node_type == Nodetype.ARRAY:
            sub_node_type = field[1][1][0]
            size = field[1][1][1]
            channels = []
            for i in range(size):
                sub_field = (field_name, sub_node_type)
                channels.extend(
                    RosChannel.get_underlying_fields(prefix, sub_field, typestore, index=i)
                )
            return channels
        # Example of `field` for a nested value:
        # ('pose', (<Nodetype.NAME: 2>, 'geometry_msgs/Pose'))
        # Each field in the nested message is a separate RosChannel.
        elif node_type == Nodetype.NAME:
            msg_def = typestore.get_msgdef(field[1][1])
            channels = []
            for sub_field in msg_def.fields:
                temp_parent_fields = list(parent_fields) if parent_fields is not None else []
                temp_parent_fields.append(field_name)
                new_parent_fields = cast(Tuple[str], temp_parent_fields)
                channels.extend(
                    RosChannel.get_underlying_fields(
                        prefix, sub_field, typestore, parent_fields=new_parent_fields
                    )
                )
            return channels
        # Sequences contain an arbitrary array of bytes, users should use handlers
        # to process them.
        elif node_type == Nodetype.SEQUENCE:
            return []
        else:
            raise Exception(f"Unsupported node type: {node_type.name}")
