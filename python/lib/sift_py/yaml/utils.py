import json
from pathlib import Path
from typing import Any, Callable, Dict, Type, cast

import yaml

try:
    import ryml
except ImportError:
    ryml = None  # type: ignore[assignment]

_HAS_RYML = ryml is not None


def _handle_subdir(path: Path, file_handler: Callable):
    """The file_handler callable must accept a Path object as its only argument."""
    for file_in_dir in path.iterdir():
        if file_in_dir.is_dir():
            _handle_subdir(file_in_dir, file_handler)
        elif file_in_dir.is_file():
            file_handler(file_in_dir)


def _type_fqn(typ: Type) -> str:
    return f"{typ.__module__}.{typ.__name__}"


def _rapidyaml_load(path: Path) -> Dict[Any, Any]:
    """Parse YAML via rapidyaml, round-tripping through JSON for a plain dict.

    rapidyaml (``ryml``) is the C++ ryml binding; it is materially faster than
    pyyaml on Sift telemetry configs (~3-4x on the example files, higher on
    large files). ``tree.resolve()`` is called before emit, which inlines both
    anchors/aliases (``&x`` / ``*x``) and merge keys (``<<: *x``); combined
    with ``emit_json``'s scalar type inference, the returned dict matches
    pyyaml's safe-load semantics on every config we ship today.

    The one semantic diff to watch for is YAML 1.1-isms that rapidyaml (YAML
    1.2) does not treat as typed: e.g. ``yes``/``no`` stay strings, and
    sexagesimal numbers stay strings. Existing Sift configs do not use these.

    Only call this when ``_HAS_RYML`` is true; the ``assert`` below narrows the
    type for static analysis and is an invariant the dispatcher upholds.
    """
    assert ryml is not None, "rapidyaml is not installed; call _pyyaml_load instead"
    with open(path, "rb") as f:
        tree = ryml.parse_in_arena(f.read())
    resolve = getattr(tree, "resolve", None)
    if callable(resolve):
        resolve()
    return cast(Dict[Any, Any], json.loads(ryml.emit_json(tree)))


def _pyyaml_load(path: Path) -> Dict[Any, Any]:
    """Fallback loader using pyyaml's C-backed ``CSafeLoader`` when available."""
    with open(path, "r") as f:
        if hasattr(yaml, "CSafeLoader"):
            return cast(Dict[Any, Any], yaml.load(f.read(), Loader=yaml.CSafeLoader))
        return cast(Dict[Any, Any], yaml.safe_load(f.read()))


def try_fast_yaml_load(path: Path) -> Dict[Any, Any]:
    """
    Try to load the YAML file using the fastest available parser.

    Order of preference:

    1. ``rapidyaml`` (``ryml``) - C++ binding, ~100x faster than pyyaml on
       large files. Requires ``pip install rapidyaml``. See
       :func:`_rapidyaml_load` for caveats (notably: no YAML 1.1 merge-key
       semantics).
    2. ``pyyaml.CSafeLoader`` - libyaml-backed, bundled with most pyyaml
       wheels but not every Python/platform combination.
    3. ``pyyaml.safe_load`` - pure-Python fallback.

    rapidyaml failures are not swallowed silently: if ``ryml`` is installed
    but raises while parsing ``path``, the exception propagates so the
    regression is visible rather than masked by the pyyaml fallback.
    """
    if _HAS_RYML:
        return _rapidyaml_load(path)
    return _pyyaml_load(path)
