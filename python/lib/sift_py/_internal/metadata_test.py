from unittest import TestCase

from sift.metadata.v1.metadata_pb2 import MetadataKey, MetadataKeyType, MetadataValue

from sift_py._internal.metadata import metadata_dict_to_pb, metadata_pb_to_dict


class TestMetadata(TestCase):
    """Tests for metadata wrapping and unwrapping functions."""

    def test_metadata_dict_to_pb_mixed_types(self):
        # Arrange
        metadata = {
            "string_key": "test",
            "number_key": 42,
            "float_key": 3.14,
            "bool_key": True,
            "zero_value": 0,
            "empty_string": "",
        }

        # Act
        result = metadata_dict_to_pb(metadata)

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

    def test_metadata_dict_to_pb_invalid_type(self):
        # Arrange
        invalid_metadata = {
            "list_key": [1, 2, 3],
            "dict_key": {"nested": "value"},
            "none_key": None,
        }

        # Act & Assert
        for key, value in invalid_metadata.items():
            with self.assertRaises(ValueError) as context:
                metadata_dict_to_pb({key: value})
            self.assertIn("Unsupported metadata value type", str(context.exception))

    def test_metadata_pb_to_dict_mixed_types(self):
        # Arrange
        wrapped_metadata = metadata_dict_to_pb(
            {
                "string_key": "test",
                "number_key": 42,
                "float_key": 3.14,
                "bool_key": True,
                "zero_value": 0,
                "empty_string": "",
            }
        )

        # Act
        result = metadata_pb_to_dict(wrapped_metadata)

        # Assert
        self.assertEqual(len(result), 6)
        self.assertEqual(result["string_key"], "test")
        self.assertEqual(result["number_key"], 42.0)
        self.assertEqual(result["float_key"], 3.14)
        self.assertTrue(result["bool_key"])
        self.assertEqual(result["zero_value"], 0.0)
        self.assertEqual(result["empty_string"], "")

    def test_metadata_pb_to_dict_empty(self):
        # Arrange
        wrapped_metadata = []

        # Act
        result = metadata_pb_to_dict(wrapped_metadata)

        # Assert
        self.assertEqual(result, {})

    def test_metadata_pb_to_dict_duplicate_key(self):
        # Arrange
        # Create metadata with duplicate keys
        wrapped_metadata = [
            MetadataValue(
                key=MetadataKey(
                    name="duplicate_key", type=MetadataKeyType.METADATA_KEY_TYPE_STRING
                ),
                string_value="first_value",
            ),
            MetadataValue(
                key=MetadataKey(
                    name="duplicate_key", type=MetadataKeyType.METADATA_KEY_TYPE_STRING
                ),
                string_value="second_value",
            ),
        ]

        # Act & Assert
        with self.assertRaises(ValueError) as context:
            metadata_pb_to_dict(wrapped_metadata)
        self.assertIn("Key already exists: duplicate_key", str(context.exception))

    def test_metadata_roundtrip(self):
        # Arrange
        original_metadata = {
            "string_key": "test",
            "number_key": 42,
            "float_key": 3.14,
            "bool_key": True,
            "zero_value": 0,
            "empty_string": "",
        }

        # Act
        wrapped = metadata_dict_to_pb(original_metadata)
        unwrapped = metadata_pb_to_dict(wrapped)

        # Assert
        self.assertEqual(unwrapped, original_metadata)
