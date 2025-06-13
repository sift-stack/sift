from unittest import TestCase

from sift_py._internal.metadata import unwrap_metadata, wrap_metadata


class TestMetadata(TestCase):
    """Tests for metadata wrapping and unwrapping functions."""

    def test_wrap_metadata_mixed_types(self):
        # Arrange
        metadata = {
            "string_key": "test",
            "number_key": 42,
            "float_key": 3.14,
            "bool_key": True,
            "zero_value": 0,
            "empty_string": ""
        }

        # Act
        result = wrap_metadata(metadata)

        # Assert
        self.assertEqual(len(result), 6)

        # Verify each type is wrapped correctly
        metadata_dict = {md.key.name: md for md in result}

        # String values
        self.assertEqual(metadata_dict["string_key"].key.type, 1)
        self.assertEqual(metadata_dict["string_key"].string_value, "test")
        self.assertEqual(metadata_dict["empty_string"].string_value, "")

        # Number values
        self.assertEqual(metadata_dict["number_key"].key.type, 2)
        self.assertEqual(metadata_dict["number_key"].number_value, 42.0)
        self.assertEqual(metadata_dict["float_key"].number_value, 3.14)
        self.assertEqual(metadata_dict["zero_value"].number_value, 0.0)

        # Boolean value
        self.assertEqual(metadata_dict["bool_key"].key.type, 3)
        self.assertTrue(metadata_dict["bool_key"].boolean_value)

    def test_wrap_metadata_invalid_type(self):
        # Arrange
        invalid_metadata = {
            "list_key": [1, 2, 3],
            "dict_key": {"nested": "value"},
            "none_key": None
        }

        # Act & Assert
        for key, value in invalid_metadata.items():
            with self.assertRaises(ValueError) as context:
                wrap_metadata({key: value})
            self.assertIn("Unsupported metadata value type", str(context.exception))

    def test_unwrap_metadata_mixed_types(self):
        # Arrange
        wrapped_metadata = wrap_metadata({
            "string_key": "test",
            "number_key": 42,
            "float_key": 3.14,
            "bool_key": True,
            "zero_value": 0,
            "empty_string": ""
        })

        # Act
        result = unwrap_metadata(wrapped_metadata)

        # Assert
        self.assertEqual(len(result), 6)
        self.assertEqual(result["string_key"], "test")
        self.assertEqual(result["number_key"], 42.0)
        self.assertEqual(result["float_key"], 3.14)
        self.assertTrue(result["bool_key"])
        self.assertEqual(result["zero_value"], 0.0)
        self.assertEqual(result["empty_string"], "")

    def test_unwrap_metadata_empty(self):
        # Arrange
        wrapped_metadata = []

        # Act
        result = unwrap_metadata(wrapped_metadata)

        # Assert
        self.assertEqual(result, {})

    def test_metadata_roundtrip(self):
        # Arrange
        original_metadata = {
            "string_key": "test",
            "number_key": 42,
            "float_key": 3.14,
            "bool_key": True,
            "zero_value": 0,
            "empty_string": ""
        }

        # Act
        wrapped = wrap_metadata(original_metadata)
        unwrapped = unwrap_metadata(wrapped)

        # Assert
        self.assertEqual(unwrapped, original_metadata)
