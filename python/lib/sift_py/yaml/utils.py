from pathlib import Path
from typing import Any, Callable, Dict, Type, cast

import yaml


def _handle_subdir(path: Path, file_handler: Callable):
    """The file_handler callable must accept a Path object as its only argument."""
    for file_in_dir in path.iterdir():
        if file_in_dir.is_dir():
            _handle_subdir(file_in_dir, file_handler)
        elif file_in_dir.is_file():
            file_handler(file_in_dir)


def _type_fqn(typ: Type) -> str:
    return f"{typ.__module__}.{typ.__name__}"


def try_fast_yaml_load(path: Path) -> Dict[Any, Any]:
    """
    Try to load the YAML file using the CSafeLoader, which is faster than the pyyaml safe loader but not built into the wheel for earlier versions of python..
    If the CSafeLoader is not available, use the pyyaml safe loader.
    """
    with open(path, "r") as f:
        if hasattr(yaml, "CSafeLoader"):
            return cast(Dict[Any, Any], yaml.load(f.read(), Loader=yaml.CSafeLoader))
        else:
            return cast(Dict[Any, Any], yaml.safe_load(f.read()))
