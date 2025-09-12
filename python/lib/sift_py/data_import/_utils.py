import mimetypes
import os
import re
from pathlib import Path
from typing import Callable, List, Optional, Tuple, Union

from alive_progress import alive_bar  # type: ignore


def mime_and_content_type_from_path(path: Path) -> Tuple[str, Optional[str], Optional[str]]:
    file_name = path.name
    mime, encoding = mimetypes.guess_type(path)
    return file_name, mime, encoding


def validate_file_type(path: Union[str, Path], valid_file_types: List[str]) -> Optional[str]:
    posix_path = Path(path) if isinstance(path, str) else path

    if not posix_path.is_file():
        raise Exception(f"Provided path, '{path}', does not point to a regular file.")

    _, mimetype, content_encoding = mime_and_content_type_from_path(posix_path)

    if not mimetype:
        raise Exception(f"The MIME-type of '{posix_path}' could not be computed.")

    if mimetype not in valid_file_types:
        raise Exception(
            f"{path} is not a valid file type ({mimetype}). Must be {', '.join(valid_file_types)}."
        )

    return content_encoding


def convert_keys_to_snake_case(obj: dict) -> dict:
    """Recursively convert all dict keys from camelCase to snake_case."""

    def camel_to_snake(name: str) -> str:
        """Convert camelCase or PascalCase to snake_case."""
        s1 = re.sub("(.)([A-Z][a-z]+)", r"\1_\2", name)
        return re.sub("([a-z0-9])([A-Z])", r"\1_\2", s1).lower()

    if isinstance(obj, dict):
        return {camel_to_snake(k): convert_keys_to_snake_case(v) for k, v in obj.items()}
    elif isinstance(obj, list):
        return [convert_keys_to_snake_case(item) for item in obj]
    else:
        return obj


class ProgressFile:
    """Displays the status with alive_bar while reading the file."""

    # alive_bar only supports context managers, so we have to make the
    # context manager calls manually.
    _bar_context: Callable

    def __init__(self, path: Union[str, Path], disable=False):
        self.path = path

        self.file_size = os.path.getsize(self.path)
        if self.file_size == 0:
            raise Exception(f"{path} is 0 bytes")

        self._file = open(self.path, mode="rb")
        self._bar = alive_bar(self.file_size, unit=" bytes", disable=disable, scale="SI")

    def read(self, *args, **kwargs):
        chunk = self._file.read(*args, **kwargs)
        self._bar_context(len(chunk))
        return chunk

    def __enter__(self):
        self._bar_context = self._bar.__enter__()
        return self

    def __exit__(self, *args, **kwargs):
        self._bar.__exit__(None, None, None)
        return
