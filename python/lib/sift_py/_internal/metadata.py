from typing import Dict, List, Union

from sift.metadata.v1.metadata_pb2 import MetadataKey, MetadataKeyType, MetadataValue


def metadata_dict_to_pb(_metadata: Dict[str, Union[str, float, bool]]) -> List[MetadataValue]:
    """
    Wraps metadata dictionary into a list of MetadataValue objects.

    Args:
        _metadata: Dictionary of metadata key-value pairs.

    Returns:
        List of MetadataValue objects.
    """
    metadata = []

    for key, value in _metadata.items():
        type = MetadataKeyType.METADATA_KEY_TYPE_UNSPECIFIED
        string_value = None
        boolean_value = None
        number_value = None

        if isinstance(value, str):
            string_value = value
            type = MetadataKeyType.METADATA_KEY_TYPE_STRING
        elif isinstance(value, bool):
            # Need to check bool before int since python thinks "True" is an int
            boolean_value = value
            type = MetadataKeyType.METADATA_KEY_TYPE_BOOLEAN
        elif isinstance(value, (int, float)):
            number_value = value
            type = MetadataKeyType.METADATA_KEY_TYPE_NUMBER
        else:
            raise ValueError(f"Unsupported metadata value type for key '{key}': {value}")

        wrapped_key = MetadataKey(name=key, type=type)
        wrapped_value = MetadataValue(
            key=wrapped_key,
            string_value=string_value,  # type: ignore
            boolean_value=boolean_value,  # type: ignore
            number_value=number_value,  # type: ignore
        )
        metadata.append(wrapped_value)

    return metadata


def metadata_pb_to_dict(metadata: List[MetadataValue]) -> Dict[str, Union[str, float, bool]]:
    """
    Unwraps a list of MetadataValue objects into a dictionary.

    Args:
        metadata: List of MetadataValue objects.

    Returns:
        Dictionary of metadata key-value pairs.
    """
    unwrapped_metadata: Dict[str, Union[str, float, bool]] = {}
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
