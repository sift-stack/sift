"""Tests for data import helpers that do not need a client."""

from __future__ import annotations

from pathlib import Path

import pytest

from sift_client._internal.util.data_imports import _config_key_matches, _supplied_configs
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import (
    CsvDataColumn,
    CsvImportConfig,
    CsvTimeColumn,
    TimeFormat,
)


def _csv(name: str = "v") -> CsvImportConfig:
    return CsvImportConfig(
        asset_name="asset",
        time_column=CsvTimeColumn(column=1, format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS),
        data_columns=[CsvDataColumn(column=2, name=name, data_type=ChannelDataType.DOUBLE)],
    )


class TestConfigKeyMatches:
    def test_matches_the_path_or_a_bare_file_name(self):
        path = Path("dir/a.csv")

        assert _config_key_matches(path, path)
        assert _config_key_matches(path, "a.csv")
        assert not _config_key_matches(path, "other/a.csv")
        assert not _config_key_matches(path, "b.csv")


class TestSuppliedConfigs:
    def test_none_detects_every_path(self):
        paths = [Path("a.csv"), Path("b.csv")]

        assert _supplied_configs(paths, None) == {paths[0]: None, paths[1]: None}

    def test_one_config_applies_to_one_path(self):
        path = Path("a.csv")
        config = _csv()

        assert _supplied_configs([path], config) == {path: config}

    def test_one_config_rejects_several_paths(self):
        with pytest.raises(ValueError, match="one file"):
            _supplied_configs([Path("a.csv"), Path("b.csv")], _csv())

    def test_mapping_assigns_by_name_and_leaves_the_rest_to_detect(self):
        first = Path("dir/a.csv")
        second = Path("dir/b.csv")
        config = _csv("supplied")

        assigned = _supplied_configs([first, second], {"a.csv": config})

        assert assigned[first] is config
        assert assigned[second] is None

    def test_unknown_key_and_two_keys_for_one_path_raise(self):
        path = Path("a.csv")
        with pytest.raises(ValueError, match="does not match"):
            _supplied_configs([path], {"missing.csv": _csv()})
        with pytest.raises(ValueError, match="More than one config"):
            _supplied_configs([path], {path: _csv(), "a.csv": _csv()})
