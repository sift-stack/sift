import pytest
from sift_stream_bindings import ValuePy


class TestValuePy:
    """Test ValuePy functionality."""

    def test_create_bool_value(self):
        """Test creating boolean values."""
        val_true = ValuePy.Bool(True)
        val_false = ValuePy.Bool(False)

        assert val_true.is_bool()
        assert val_false.is_bool()
        assert val_true.as_bool() is True
        assert val_false.as_bool() is False

    def test_create_string_value(self):
        """Test creating string values."""
        val = ValuePy.String("hello world")

        assert val.is_string()
        assert val.as_string() == "hello world"

    def test_create_float_value(self):
        """Test creating float values."""
        val = ValuePy.Float(3.14)

        assert val.is_float()
        assert abs(val.as_float() - 3.14) < 0.001

    def test_create_double_value(self):
        """Test creating double values."""
        val = ValuePy.Double(3.141592653589793)

        assert val.is_double()
        assert abs(val.as_double() - 3.141592653589793) < 1e-10

    def test_create_int32_value(self):
        """Test creating int32 values."""
        val = ValuePy.Int32(42)

        assert val.is_int32()
        assert val.as_int32() == 42

    def test_create_uint32_value(self):
        """Test creating uint32 values."""
        val = ValuePy.Uint32(42)

        assert val.is_uint32()
        assert val.as_uint32() == 42

    def test_create_int64_value(self):
        """Test creating int64 values."""
        val = ValuePy.Int64(9223372036854775807)

        assert val.is_int64()
        assert val.as_int64() == 9223372036854775807

    def test_create_uint64_value(self):
        """Test creating uint64 values."""
        val = ValuePy.Uint64(18446744073709551615)

        assert val.is_uint64()
        assert val.as_uint64() == 18446744073709551615

    def test_create_enum_value(self):
        """Test creating enum values."""
        val = ValuePy.Enum(2)

        assert val.is_enum()
        assert val.as_enum() == 2

    def test_create_bit_field_value(self):
        """Test creating bit field values."""
        # BitField expects Vec<u8>, so pass list of bytes
        val = ValuePy.BitField([0x0A])

        assert val.is_bitfield()
        assert val.as_bitfield() == b'\x0A'

    def test_type_error_on_wrong_accessor(self):
        """Test that accessing with wrong type raises error."""
        val = ValuePy.Bool(True)

        with pytest.raises(TypeError):
            val.as_string()

        with pytest.raises(TypeError):
            val.as_int32()

    def test_multiple_values(self):
        """Test creating multiple values of different types."""
        values = [
            ValuePy.Bool(True),
            ValuePy.String("test"),
            ValuePy.Float(1.5),
            ValuePy.Double(2.5),
            ValuePy.Int32(-100),
            ValuePy.Uint32(100),
            ValuePy.Int64(-1000000),
            ValuePy.Uint64(1000000),
            ValuePy.Enum(5),
            ValuePy.BitField([0xFF]),
        ]

        assert len(values) == 10
        assert values[0].is_bool()
        assert values[1].is_string()
        assert values[2].is_float()
        assert values[3].is_double()
        assert values[4].is_int32()
        assert values[5].is_uint32()
        assert values[6].is_int64()
        assert values[7].is_uint64()
        assert values[8].is_enum()
        assert values[9].is_bitfield()
