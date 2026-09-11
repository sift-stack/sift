from __future__ import annotations

from typing import TYPE_CHECKING, Any, Dict, Union

from pydantic_core import core_schema
from sift.metadata.v1.metadata_pb2 import (
    MetadataKey,
    MetadataKeyType,
)
from sift.metadata.v1.metadata_pb2 import (
    MetadataValue as MetadataProto,
)

if TYPE_CHECKING:
    from pydantic import GetCoreSchemaHandler


class Metadata(Dict[str, Union[str, float, bool]]):
    """Entity metadata: a dict of key -> first value, plus access to every
    value of a key via getall().

    A metadata key may hold multiple values (multi-value metadata), kept in
    canonical order. Plain dict access (md["k"], md.get("k"), iteration, ==)
    sees the first value only -- the same value every single-value context
    uses -- so existing code written against scalar metadata keeps working
    unchanged. getall(key) returns the full ordered value list; its first
    element is always the dict value.
    """

    def __init__(
        self,
        first_values: dict[str, str | float | bool] | None = None,
        all_values: dict[str, list[str | float | bool]] | None = None,
    ):
        """Build the mapping from a first-value dict and optional full value lists."""
        super().__init__(first_values or {})
        self._all_values: dict[str, list[str | float | bool]] = {
            key: list(values) for key, values in (all_values or {}).items()
        }

    def getall(self, key: str) -> list[str | float | bool]:
        """Return every value of ``key`` as a new list, in canonical order.

        Keys absent from the metadata yield []; keys holding a single value
        yield a one-element list.
        """
        if key in self._all_values:
            return list(self._all_values[key])
        if key in self:
            return [self[key]]
        return []

    @classmethod
    def __get_pydantic_core_schema__(
        cls, source_type: Any, handler: GetCoreSchemaHandler
    ) -> core_schema.CoreSchema:
        # Without this hook, pydantic validates ``Metadata`` fields as plain
        # dicts and rebuilds them, dropping the extra values behind getall().
        # Pass instances through untouched; wrap plain mappings.
        def _validate(value: Any) -> Metadata:
            if isinstance(value, cls):
                return value
            if isinstance(value, dict):
                return cls(value)
            raise ValueError(f"expected a metadata mapping, got {type(value).__name__}")

        return core_schema.no_info_plain_validator_function(
            _validate,
            serialization=core_schema.plain_serializer_function_ser_schema(dict),
        )


def metadata_dict_to_proto(_metadata: dict[str, str | float | bool]) -> list[MetadataProto]:
    """Converts metadata dictionary into a list of MetadataValue objects.

    Args:
        _metadata: Dictionary of metadata key-value pairs.

    Returns:
        List of MetadataValue objects.
    """
    metadata = []

    for key, value in _metadata.items():
        metadata_key_type = MetadataKeyType.METADATA_KEY_TYPE_UNSPECIFIED
        string_value = None
        boolean_value = None
        number_value = None

        if isinstance(value, str):
            string_value = value
            metadata_key_type = MetadataKeyType.METADATA_KEY_TYPE_STRING
        elif isinstance(value, bool):
            # Need to check bool before int since python thinks "True" is an int
            boolean_value = value
            metadata_key_type = MetadataKeyType.METADATA_KEY_TYPE_BOOLEAN
        elif isinstance(value, (int, float)):
            number_value = value
            metadata_key_type = MetadataKeyType.METADATA_KEY_TYPE_NUMBER
        else:
            raise ValueError(f"Unsupported metadata value type for key '{key}': {value}")

        wrapped_key = MetadataKey(name=key, type=metadata_key_type)
        wrapped_value = MetadataProto(
            key=wrapped_key,
            string_value=string_value,  # type: ignore
            boolean_value=boolean_value,  # type: ignore
            number_value=number_value,  # type: ignore
        )
        metadata.append(wrapped_value)

    return metadata


def metadata_proto_to_dict(metadata: list[MetadataProto]) -> Metadata:
    """Converts a list of MetadataValue objects into a Metadata mapping.

    A key may appear multiple times when it holds multiple values
    (multi-value metadata). The mapping's dict view keeps the first value of
    each key -- the API returns values in canonical order, so this matches
    the value every other single-value context (e.g. backend flattening)
    uses -- and ``Metadata.getall(key)`` returns the full ordered list.

    Args:
        metadata: List of MetadataValue objects.

    Returns:
        Metadata mapping of key-value pairs (first value per key; all values
        via getall).
    """
    first_values: dict[str, str | float | bool] = {}
    all_values: dict[str, list[str | float | bool]] = {}
    for md in metadata:
        value: str | float | bool
        if md.key.type == MetadataKeyType.METADATA_KEY_TYPE_STRING:
            value = md.string_value
        elif md.key.type == MetadataKeyType.METADATA_KEY_TYPE_BOOLEAN:
            value = md.boolean_value
        elif md.key.type == MetadataKeyType.METADATA_KEY_TYPE_NUMBER:
            value = md.number_value
        else:
            continue
        first_values.setdefault(md.key.name, value)
        all_values.setdefault(md.key.name, []).append(value)

    return Metadata(first_values, all_values)
