from typing import Any, Callable, Dict, List, Optional, Union

from sift.metadata.v1.metadata_pb2 import MetadataKey, MetadataKeyType, MetadataValue


def metadata_dict_to_pb(
    _metadata: Dict[str, Union[str, float, bool, int]],
    parse: Optional[Callable[[Any], Optional[Union[str, float, bool, int]]]] = None,
) -> List[MetadataValue]:
    """
    Wraps metadata dictionary into a list of MetadataValue objects.

    Args:
        _metadata: Dictionary of metadata key-value pairs.
        parse: Optional function to parse complex types into a compatible
           metadata type (i.e, str, float, int, or bool). Function should raise an
           Exception if it can't parse the value.
    Returns:
        List of MetadataValue objects.
    """
    metadata = []

    for key, value in _metadata.items():
        type = MetadataKeyType.METADATA_KEY_TYPE_UNSPECIFIED
        string_value = None
        boolean_value = None
        number_value = None

        if not isinstance(value, (str, float, bool, int)):
            if parse:
                value = parse(value)
            else:
                raise ValueError(f"Unsupported metadata value type for key '{key}': {value}")

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


def metadata_pb_to_dict(metadata: List[MetadataValue]) -> Dict[str, Union[str, float, bool, int]]:
    """
    Unwraps a list of MetadataValue objects into a dictionary.

    Args:
        metadata: List of MetadataValue objects.

    Returns:
        Dictionary of metadata key-value pairs.
    """
    unwrapped_metadata: Dict[str, Union[str, float, bool, int]] = {}
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


def metadata_pb_to_dict_api(metadata: List[MetadataValue]) -> List[Dict[str, Any]]:
    """
    Serializes a list of MetadataValue objects to a n API compatible dict,
    preserving the proto structure.

    Args:
        metadata: List of MetadataValue objects.

    Returns:
        Dict representing the metadata with proto structure.
    """

    def metadata_value_to_dict(md: MetadataValue) -> Dict[str, Any]:
        value_dict: Dict[str, Any] = {"key": {"name": md.key.name, "type": md.key.type}}
        if md.key.type == MetadataKeyType.METADATA_KEY_TYPE_STRING:
            value_dict["string_value"] = md.string_value
        elif md.key.type == MetadataKeyType.METADATA_KEY_TYPE_BOOLEAN:
            value_dict["boolean_value"] = md.boolean_value
        elif md.key.type == MetadataKeyType.METADATA_KEY_TYPE_NUMBER:
            value_dict["number_value"] = md.number_value
        else:
            raise ValueError(f"{md.key.name} set to invalid type: {md.key.type}")
        return value_dict

    metadata_list = [metadata_value_to_dict(md) for md in metadata]
    return metadata_list
