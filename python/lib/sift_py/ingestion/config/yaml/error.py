from typing import Any, List, Optional

from typing_extensions import Self

_PREFIX = "  "
_YAML_SEP = "---"
_LIST_ITEM_PREFIX = "- "


class YamlConfigError(Exception):
    """
    When the YAML config has missing or invalid properties.
    """

    message: str

    def __init__(self, message: str):
        super().__init__(message)

    @classmethod
    def _invalid_property(
        cls,
        actual_value: Any,
        key_name: str,
        expected_type: str,
        ancestors: Optional[List[str]] = None,
    ) -> Self:
        header = f"Expected '{key_name.lstrip(_LIST_ITEM_PREFIX)}' to be <{expected_type}> but it is <{type(actual_value).__name__}>\n{_YAML_SEP}\n"

        if ancestors is None or len(ancestors) == 0:
            return cls(f"{header}{key_name}: <{expected_type}>")

        key_value_path = ""

        current_indentation_level = 0

        for ancestor in ancestors:
            key_value_path += f"{_PREFIX * current_indentation_level}{ancestor}:\n"
            current_indentation_level += 1

        key_value_path += f"{_PREFIX * current_indentation_level}{key_name}: <{expected_type}>"
        output = f"{header}{key_value_path}"

        return cls(output)
