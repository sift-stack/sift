from sift_stream_bindings import (
    ChannelBitFieldElementPy,
    ChannelConfigPy,
    ChannelDataTypePy,
    ChannelEnumTypePy,
)


class TestChannelDataType:
    """Test ChannelDataTypePy enum."""

    def test_channel_data_types_exist(self):
        """Test that all expected data types are available."""
        assert hasattr(ChannelDataTypePy, "Bool")
        assert hasattr(ChannelDataTypePy, "String")
        assert hasattr(ChannelDataTypePy, "Float")
        assert hasattr(ChannelDataTypePy, "Double")
        assert hasattr(ChannelDataTypePy, "Int32")
        assert hasattr(ChannelDataTypePy, "Uint32")
        assert hasattr(ChannelDataTypePy, "Int64")
        assert hasattr(ChannelDataTypePy, "Uint64")
        assert hasattr(ChannelDataTypePy, "Enum")
        assert hasattr(ChannelDataTypePy, "BitField")
        assert hasattr(ChannelDataTypePy, "Bytes")


class TestChannelEnumType:
    """Test ChannelEnumTypePy functionality."""

    def test_create_enum_type(self):
        """Test creating multiple enum types."""
        enum_types = [
            ChannelEnumTypePy("STATE_1", 0),
            ChannelEnumTypePy("STATE_2", 1),
            ChannelEnumTypePy("STATE_3", 2),
        ]
        assert enum_types[0] is not None
        assert enum_types[0].name == "STATE_1"
        assert enum_types[0].key == 0
        assert enum_types[1] is not None
        assert enum_types[1].name == "STATE_2"
        assert enum_types[1].key == 1
        assert enum_types[2].name == "STATE_3"
        assert enum_types[2].key == 2


class TestChannelBitFieldElement:
    """Test ChannelBitFieldElementPy functionality."""

    def test_create_bitfield(self):
        """Test creating multiple bitfield elements."""
        elements = [
            ChannelBitFieldElementPy("FLAG_1", 0, 1),
            ChannelBitFieldElementPy("FLAG_2", 1, 1),
            ChannelBitFieldElementPy("FLAG_3", 2, 1),
        ]
        assert elements[0].name == "FLAG_1"
        assert elements[0].index == 0
        assert elements[0].bit_count == 1
        assert elements[1].name == "FLAG_2"
        assert elements[1].index == 1
        assert elements[1].bit_count == 1
        assert elements[2].name == "FLAG_3"
        assert elements[2].index == 2
        assert elements[2].bit_count == 1


class TestChannelConfig:
    """Test ChannelConfigPy functionality."""

    def test_create_bool_channel(self):
        """Test creating a boolean channel config."""
        channel = ChannelConfigPy(
            name="test_bool",
            data_type=ChannelDataTypePy.Bool,
            unit="",
            description="Test boolean channel",
            enum_types=[],
            bit_field_elements=[],
        )
        assert channel.name == "test_bool"
        assert channel.unit == ""
        assert channel.description == "Test boolean channel"
        assert channel.enum_types == []
        assert channel.bit_field_elements == []

    def test_create_string_channel(self):
        """Test creating a string channel config."""
        channel = ChannelConfigPy(
            name="test_string",
            data_type=ChannelDataTypePy.String,
            unit="",
            description="Test string channel",
            enum_types=[],
            bit_field_elements=[],
        )
        assert channel.name == "test_string"
        assert channel.unit == ""
        assert channel.description == "Test string channel"
        assert channel.enum_types == []
        assert channel.bit_field_elements == []

    def test_create_numeric_channels(self):
        """Test creating numeric channel configs."""
        numeric_types = [
            ChannelDataTypePy.Float,
            ChannelDataTypePy.Double,
            ChannelDataTypePy.Int32,
            ChannelDataTypePy.Uint32,
            ChannelDataTypePy.Int64,
            ChannelDataTypePy.Uint64,
        ]

        for i, data_type in enumerate(numeric_types):
            channel = ChannelConfigPy(
                name=f"test_numeric_{i}",
                data_type=data_type,
                unit="units",
                description=f"Test numeric channel {i}",
                enum_types=[],
                bit_field_elements=[],
            )
            assert channel.name == f"test_numeric_{i}"
            assert channel.unit == "units"
            assert channel.description == f"Test numeric channel {i}"
            assert channel.enum_types == []
            assert channel.bit_field_elements == []

    def test_create_enum_channel(self):
        """Test creating an enum channel config."""
        enum_types = [
            ChannelEnumTypePy("STATE_1", 0),
            ChannelEnumTypePy("STATE_2", 1),
            ChannelEnumTypePy("STATE_3", 2),
        ]

        channel = ChannelConfigPy(
            name="test_enum",
            data_type=ChannelDataTypePy.Enum,
            unit="",
            description="Test enum channel",
            enum_types=enum_types,
            bit_field_elements=[],
        )
        assert channel.name == "test_enum"
        assert channel.unit == ""
        assert channel.description == "Test enum channel"

    def test_create_bitfield_channel(self):
        """Test creating a bitfield channel config."""
        bit_field_elements = [
            ChannelBitFieldElementPy("FLAG_1", 0, 1),
            ChannelBitFieldElementPy("FLAG_2", 1, 1),
            ChannelBitFieldElementPy("FLAG_3", 2, 1),
        ]

        channel = ChannelConfigPy(
            name="test_bitfield",
            data_type=ChannelDataTypePy.BitField,
            unit="",
            description="Test bitfield channel",
            enum_types=[],
            bit_field_elements=bit_field_elements,
        )
        assert channel.name == "test_bitfield"
        assert channel.unit == ""
        assert channel.description == "Test bitfield channel"
