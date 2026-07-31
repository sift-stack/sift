import json

import pytest
from pydantic import BaseModel
from sift.metadata.v1.metadata_pb2 import (
    MetadataKey,
    MetadataKeyType,
    MetadataValue,
)

from sift_client.util.metadata import (
    Metadata,
    expand_metadata_for_write,
    metadata_dict_to_proto,
    metadata_proto_to_dict,
)


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

    def test_multi_value_key_exposes_all_values_via_getall(self):
        metadata = [
            _string_value("associated_parts", "ABC"),
            _string_value("associated_parts", "XYZ"),
            _string_value("env", "prod"),
        ]
        result = metadata_proto_to_dict(metadata)
        assert isinstance(result, Metadata)
        assert result.getall("associated_parts") == ["ABC", "XYZ"]
        assert result.getall("env") == ["prod"]
        assert result.getall("missing") == []

    def test_empty_metadata(self):
        assert metadata_proto_to_dict([]) == {}

    def test_round_trip_from_dict(self):
        original = {"env": "prod", "build": 1.5, "armed": True}
        assert metadata_proto_to_dict(metadata_dict_to_proto(original)) == original


class TestMetadataDictToProto:
    def test_list_value_writes_one_proto_per_element_in_order(self):
        protos = metadata_dict_to_proto({"parts": ["ABC", "XYZ"], "env": "prod"})
        parts = [p.string_value for p in protos if p.key.name == "parts"]
        assert parts == ["ABC", "XYZ"]
        assert all(
            p.key.type == MetadataKeyType.METADATA_KEY_TYPE_STRING
            for p in protos
            if p.key.name == "parts"
        )
        assert [p.string_value for p in protos if p.key.name == "env"] == ["prod"]

    def test_list_round_trips_through_proto(self):
        protos = metadata_dict_to_proto({"parts": ["ABC", "XYZ"], "build": 1.5})
        result = metadata_proto_to_dict(protos)
        assert result == {"parts": "ABC", "build": 1.5}
        assert result.getall("parts") == ["ABC", "XYZ"]

    def test_metadata_instance_keeps_all_values(self):
        md = Metadata({"parts": "ABC", "env": "prod"}, {"parts": ["ABC", "XYZ"]})
        protos = metadata_dict_to_proto(md)
        assert [p.string_value for p in protos if p.key.name == "parts"] == ["ABC", "XYZ"]
        assert [p.string_value for p in protos if p.key.name == "env"] == ["prod"]

    def test_empty_list_rejected(self):
        with pytest.raises(ValueError, match="empty value list"):
            metadata_dict_to_proto({"parts": []})

    def test_non_string_list_element_rejected(self):
        with pytest.raises(ValueError, match="only string values may be multi-value"):
            metadata_dict_to_proto({"parts": ["ABC", 2]})  # type: ignore[list-item]

    def test_unsupported_scalar_rejected(self):
        with pytest.raises(ValueError, match="Unsupported metadata value type"):
            metadata_dict_to_proto({"parts": None})  # type: ignore[dict-item]


class TestExpandMetadataForWrite:
    def test_multi_value_keys_become_lists_single_stay_scalar(self):
        md = Metadata(
            {"parts": "ABC", "env": "prod", "build": 1.5},
            {"parts": ["ABC", "XYZ"], "env": ["prod"]},
        )
        assert expand_metadata_for_write(md) == {
            "parts": ["ABC", "XYZ"],
            "env": "prod",
            "build": 1.5,
        }

    def test_plain_dict_returned_unchanged(self):
        plain = {"parts": ["ABC", "XYZ"], "env": "prod"}
        assert expand_metadata_for_write(plain) is plain


class TestMetadataMapping:
    def test_dict_view_and_getall_invariant(self):
        md = Metadata(
            {"associated_parts": "ABC", "env": "prod"},
            {"associated_parts": ["ABC", "XYZ"], "env": ["prod"]},
        )
        assert md["associated_parts"] == "ABC"
        assert md == {"associated_parts": "ABC", "env": "prod"}
        for key in md:
            assert md[key] == md.getall(key)[0]

    def test_getall_falls_back_to_scalar_without_all_values(self):
        md = Metadata({"env": "prod"})
        assert md.getall("env") == ["prod"]
        assert md.getall("missing") == []

    def test_getall_returns_a_copy(self):
        md = Metadata({"k": "a"}, {"k": ["a", "b"]})
        md.getall("k").append("mutated")
        assert md.getall("k") == ["a", "b"]

    def test_json_serializable(self):
        md = Metadata({"env": "prod"}, {"env": ["prod"]})
        assert json.loads(json.dumps(md)) == {"env": "prod"}


class TestMetadataPydanticField:
    class _Model(BaseModel):
        metadata: Metadata

    def test_instance_passes_through_validation_with_all_values(self):
        # Pydantic must not rebuild the mapping as a plain dict -- that would
        # silently drop the extra values behind getall().
        md = Metadata({"parts": "ABC"}, {"parts": ["ABC", "XYZ"]})
        model = self._Model(metadata=md)
        assert isinstance(model.metadata, Metadata)
        assert model.metadata.getall("parts") == ["ABC", "XYZ"]

    def test_plain_dict_is_wrapped(self):
        model = self._Model(metadata={"env": "prod"})
        assert isinstance(model.metadata, Metadata)
        assert model.metadata.getall("env") == ["prod"]

    def test_non_mapping_rejected(self):
        with pytest.raises(ValueError, match="expected a metadata mapping"):
            self._Model(metadata="not-a-mapping")  # type: ignore[arg-type]

    def test_model_dump_serializes_as_plain_dict(self):
        md = Metadata({"parts": "ABC"}, {"parts": ["ABC", "XYZ"]})
        model = self._Model(metadata=md)
        assert model.model_dump() == {"metadata": {"parts": "ABC"}}
        assert json.loads(model.model_dump_json()) == {"metadata": {"parts": "ABC"}}
