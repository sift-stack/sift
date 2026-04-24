"""Exercises the three YAML loader paths in :mod:`sift_py.yaml.utils`.

The three paths, in priority order inside :func:`try_fast_yaml_load`:

1. ``rapidyaml`` (``ryml``) -- C++ bindings, declared as a required dependency.
2. pyyaml ``CSafeLoader`` -- libyaml-backed, available on platforms whose
   pyyaml wheel ships libyaml (i.e. almost all of them).
3. pyyaml ``safe_load`` -- pure-Python fallback used when neither libyaml nor
   rapidyaml is installed.

Each path should produce an identical dict for the shipped example ingest
config, including correct resolution of anchors (``&foo`` / ``*foo``) and
YAML 1.1 merge keys (``<<: *foo``) used heavily by that config.
"""

from pathlib import Path

import pytest
import yaml

from sift_py.yaml import utils

EXAMPLE_CONFIG = (
    Path(__file__).resolve().parents[3] / "examples" / "telemetry_config.example.yml"
)


def _force_pure_python_pyyaml(monkeypatch: pytest.MonkeyPatch) -> None:
    """Hide ``yaml.CSafeLoader`` so ``_pyyaml_load`` falls through to ``safe_load``."""
    monkeypatch.delattr(yaml, "CSafeLoader", raising=False)


def _assert_sift_ingest_shape(config: dict) -> None:
    """Sanity-check that *config* is a well-formed Sift ingest YAML dict.

    Focuses on fields that a miswired loader would get wrong: anchors resolving
    to inline dicts, merge keys (``<<:``) being expanded rather than left as
    literal ``"<<"`` keys, and numeric scalars surviving the round-trip as
    ints rather than strings.
    """
    assert isinstance(config, dict)
    assert isinstance(config.get("asset_name"), str)
    assert isinstance(config["channels"], dict)
    assert isinstance(config["flows"], list)
    assert config["flows"], "example config should declare at least one flow"

    for flow in config["flows"]:
        assert isinstance(flow["name"], str)
        for ch in flow.get("channels", []):
            if not isinstance(ch, dict):
                continue
            # Merge keys must be inlined, not left as a literal "<<" key.
            assert "<<" not in ch, f"unresolved merge key in flow channel: {ch!r}"
            # Any numeric field (bit_field_elements[].index, enum_types[].key)
            # must come back as int, not str.
            for enum in ch.get("enum_types", []) or []:
                assert isinstance(enum.get("key"), int), f"enum key not int: {enum!r}"
            for bit in ch.get("bit_field_elements", []) or []:
                assert isinstance(bit.get("index"), int), f"bit index not int: {bit!r}"
                assert isinstance(bit.get("bit_count"), int), (
                    f"bit_count not int: {bit!r}"
                )


def test_rapidyaml_load_path():
    """Primary path: rapidyaml produces a schema-valid ingest dict."""
    pytest.importorskip("ryml")
    result = utils._rapidyaml_load(EXAMPLE_CONFIG)
    _assert_sift_ingest_shape(result)


def test_pyyaml_csafeloader_path():
    """Fallback path 1: pyyaml's libyaml-backed ``CSafeLoader``."""
    if not hasattr(yaml, "CSafeLoader"):
        pytest.skip("libyaml/CSafeLoader not installed in this environment")
    result = utils._pyyaml_load(EXAMPLE_CONFIG)
    _assert_sift_ingest_shape(result)


def test_pyyaml_pure_python_path(monkeypatch: pytest.MonkeyPatch):
    """Fallback path 2: pure-Python ``safe_load`` with ``CSafeLoader`` hidden."""
    _force_pure_python_pyyaml(monkeypatch)
    assert not hasattr(yaml, "CSafeLoader"), "monkeypatch failed to hide CSafeLoader"
    result = utils._pyyaml_load(EXAMPLE_CONFIG)
    _assert_sift_ingest_shape(result)


def test_all_three_loaders_agree(monkeypatch: pytest.MonkeyPatch):
    """rapidyaml, CSafeLoader, and pure-Python ``safe_load`` return the same dict.

    Dispatch order matters: call the two pyyaml paths *after* rapidyaml, and
    perform the pure-Python load last so the ``CSafeLoader`` monkeypatch
    cannot leak into the libyaml path.
    """
    pytest.importorskip("ryml")
    if not hasattr(yaml, "CSafeLoader"):
        pytest.skip("libyaml/CSafeLoader not installed; cannot compare all three")

    via_ryml = utils._rapidyaml_load(EXAMPLE_CONFIG)
    via_csafe = utils._pyyaml_load(EXAMPLE_CONFIG)

    _force_pure_python_pyyaml(monkeypatch)
    via_safe = utils._pyyaml_load(EXAMPLE_CONFIG)

    assert via_ryml == via_csafe
    assert via_csafe == via_safe


def test_try_fast_yaml_load_dispatches_to_rapidyaml_when_available():
    """``try_fast_yaml_load`` returns the rapidyaml result when ``_HAS_RYML`` is true."""
    pytest.importorskip("ryml")
    assert utils._HAS_RYML, "rapidyaml declared as a required dep but not detected"
    assert utils.try_fast_yaml_load(EXAMPLE_CONFIG) == utils._rapidyaml_load(
        EXAMPLE_CONFIG
    )
