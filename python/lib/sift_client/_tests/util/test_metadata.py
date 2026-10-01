from sift.metadata.v1.metadata_pb2 import (
    MetadataKey,
    MetadataKeyType,
    MetadataValue,
)

from sift_client.util.metadata import metadata_dict_to_proto, metadata_proto_to_dict


def _string_value(name: str, value: str) -> MetadataValue:
    return MetadataValue(
        key=MetadataKey(name=name, type=MetadataKeyType.METADATA_KEY_TYPE_STRING),
        string_value=value,
    )


class TestMetadataProtoToDict:
    def test_unwraps_each_value_type(self):
        metadata = [
            _string_value("env", "prod"),
            MetadataValue(
                key=MetadataKey(name="build", type=MetadataKeyType.METADATA_KEY_TYPE_NUMBER),
                number_value=1.5,
            ),
            MetadataValue(
                key=MetadataKey(name="armed", type=MetadataKeyType.METADATA_KEY_TYPE_BOOLEAN),
                boolean_value=True,
            ),
        ]
        assert metadata_proto_to_dict(metadata) == {
            "env": "prod",
            "build": 1.5,
            "armed": True,
        }

    def test_multi_value_key_keeps_first_value(self):
        # A key can hold multiple values (multi-value metadata). The API
        # returns values in canonical order; the dict view keeps the first.
        metadata = [
            _string_value("associated_parts", "ABC"),
            _string_value("associated_parts", "XYZ"),
            _string_value("env", "prod"),
        ]
        assert metadata_proto_to_dict(metadata) == {
            "associated_parts": "ABC",
            "env": "prod",
        }

    def test_empty_metadata(self):
        assert metadata_proto_to_dict([]) == {}

    def test_round_trip_from_dict(self):
        original = {"env": "prod", "build": 1.5, "armed": True}
        assert metadata_proto_to_dict(metadata_dict_to_proto(original)) == original
