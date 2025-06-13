from typing import Dict, List, Union

from sift.metadata.v1.metadata_pb2 import MetadataKey, MetadataKeyType, MetadataValue


def wrap_metadata(_metadata: Dict[str, Union[str, float, bool]]) -> List[MetadataValue]:
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
            string_value=string_value,
            boolean_value=boolean_value,
            number_value=number_value,
        )
        metadata.append(wrapped_value)

    return metadata
