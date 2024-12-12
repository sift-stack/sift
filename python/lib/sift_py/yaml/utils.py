from pathlib import Path
from typing import Callable, Type


def _handle_subdir(path: Path, file_handler: Callable):
    """The file_handler callable must accept a Path object as its only argument."""
    for file_in_dir in path.iterdir():
        if file_in_dir.is_dir():
            _handle_subdir(file_in_dir, file_handler)
        elif file_in_dir.is_file():
            file_handler(file_in_dir)


def _type_fqn(typ: Type) -> str:
    return f"{typ.__module__}.{typ.__name__}"
