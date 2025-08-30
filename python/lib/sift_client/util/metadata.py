from __future__ import annotations

from sift.metadata.v1.metadata_pb2 import (
    MetadataKey,
    MetadataKeyType,
)
from sift.metadata.v1.metadata_pb2 import (
    MetadataValue as MetadataProto,
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


def metadata_proto_to_dict(metadata: list[MetadataProto]) -> dict[str, str | float | bool]:
    """Converts a list of MetadataValue objects into a dictionary.

    Args:
        metadata: List of MetadataValue objects.

    Returns:
        Dictionary of metadata key-value pairs.
    """
    unwrapped_metadata: dict[str, str | float | bool] = {}
    for md in metadata:
        if md.key.name in unwrapped_metadata:
            raise ValueError(f"Key already exists: {md.key.name}")
        if md.key.type == MetadataKeyType.METADATA_KEY_TYPE_STRING:
            unwrapped_metadata[md.key.name] = md.string_value
        elif md.key.type == MetadataKeyType.METADATA_KEY_TYPE_BOOLEAN:
            unwrapped_metadata[md.key.name] = md.boolean_value
        elif md.key.type == MetadataKeyType.METADATA_KEY_TYPE_NUMBER:
            unwrapped_metadata[md.key.name] = md.number_value

    return unwrapped_metadata
