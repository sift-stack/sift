"""Unit tests for building processed channel uploads."""

from __future__ import annotations

import subprocess
import sys
import types
from typing import TYPE_CHECKING

import pandas as pd
import pyarrow.parquet as pq
import pytest

from sift_client._internal.util.file import extract_parquet_footer
from sift_client._internal.util.processed_channels import (
    TIME_COLUMN_PATH,
    default_code,
    write_processed_channel_parquet,
)
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import TimeFormat

if TYPE_CHECKING:
    from pathlib import Path

TIMES = pd.to_datetime(["2026-09-29T13:00:00.0Z", "2026-09-29T13:00:01.5Z"])


def _git(cwd: Path, *args: str) -> str:
    result = subprocess.run(["git", *args], cwd=cwd, capture_output=True, text=True, check=True)
    return result.stdout.strip()


def _channels(config) -> dict[str, tuple[int, str]]:
    return {
        c.channel_config.name: (c.channel_config.data_type, c.path) for c in config.data_columns
    }


class TestWriteProcessedChannelParquet:
    def test_index_time_and_column_types(self, tmp_path: Path):
        df = pd.DataFrame(
            {
                "battery_health": [0.98, 0.97],
                "count": [1, 2],
                "ok": [True, False],
                "state": ["a", "b"],
            },
            index=TIMES,
        )
        path = tmp_path / "out.parquet"

        config, footer_offset, footer_length = write_processed_channel_parquet(
            df, path, units={"battery_health": "pct"}
        )

        assert config.time_column.path == TIME_COLUMN_PATH
        assert config.time_column.format == TimeFormat.ABSOLUTE_UNIX_NANOSECONDS.value
        assert _channels(config) == {
            "battery_health": (ChannelDataType.DOUBLE.value, "battery_health"),
            "count": (ChannelDataType.INT_64.value, "count"),
            "ok": (ChannelDataType.BOOL.value, "ok"),
            "state": (ChannelDataType.STRING.value, "state"),
        }
        units = {c.channel_config.name: c.channel_config.units for c in config.data_columns}
        assert units == {"battery_health": "pct", "count": "", "ok": "", "state": ""}

        footer_bytes, expected_offset = extract_parquet_footer(path)
        assert (footer_offset, footer_length) == (expected_offset, len(footer_bytes))

        table = pq.read_table(path)
        assert table.column(TIME_COLUMN_PATH).to_pylist() == [
            1790686800000000000,
            1790686801500000000,
        ]
        assert table.column("battery_health").to_pylist() == [0.98, 0.97]

    def test_time_column_is_not_a_channel(self, tmp_path: Path):
        df = pd.DataFrame({"t": TIMES, "battery_health": [0.98, 0.97]})

        config, _, _ = write_processed_channel_parquet(
            df, tmp_path / "out.parquet", time_column="t"
        )

        assert list(_channels(config)) == ["battery_health"]

    def test_naive_times_are_utc(self, tmp_path: Path):
        aware = pd.DataFrame({"v": [1.0, 2.0]}, index=TIMES)
        naive = pd.DataFrame({"v": [1.0, 2.0]}, index=TIMES.tz_localize(None))

        write_processed_channel_parquet(aware, tmp_path / "aware.parquet")
        write_processed_channel_parquet(naive, tmp_path / "naive.parquet")

        assert (
            pq.read_table(tmp_path / "aware.parquet").column(TIME_COLUMN_PATH).to_pylist()
            == pq.read_table(tmp_path / "naive.parquet").column(TIME_COLUMN_PATH).to_pylist()
        )

    def test_index_must_be_datetimes(self, tmp_path: Path):
        df = pd.DataFrame({"v": [1.0, 2.0]})

        with pytest.raises(ValueError, match="index must hold datetimes"):
            write_processed_channel_parquet(df, tmp_path / "out.parquet")

    def test_unknown_time_column(self, tmp_path: Path):
        df = pd.DataFrame({"v": [1.0, 2.0]}, index=TIMES)

        with pytest.raises(ValueError, match="time_column 'missing'"):
            write_processed_channel_parquet(df, tmp_path / "out.parquet", time_column="missing")

    def test_missing_times(self, tmp_path: Path):
        df = pd.DataFrame({"v": [1.0, 2.0]}, index=pd.DatetimeIndex(["2026-09-29", pd.NaT]))

        with pytest.raises(ValueError, match="missing values"):
            write_processed_channel_parquet(df, tmp_path / "out.parquet")

    def test_units_for_unknown_column(self, tmp_path: Path):
        df = pd.DataFrame({"v": [1.0, 2.0]}, index=TIMES)

        with pytest.raises(ValueError, match="units names columns"):
            write_processed_channel_parquet(df, tmp_path / "out.parquet", units={"w": "pct"})

    def test_no_data_columns(self, tmp_path: Path):
        df = pd.DataFrame({"t": TIMES})

        with pytest.raises(ValueError, match="no data columns"):
            write_processed_channel_parquet(df, tmp_path / "out.parquet", time_column="t")

    def test_duplicate_column_names(self, tmp_path: Path):
        df = pd.DataFrame([[1.0, 2.0], [3.0, 4.0]], columns=["v", "v"], index=TIMES)

        with pytest.raises(ValueError, match="unique name"):
            write_processed_channel_parquet(df, tmp_path / "out.parquet")

    @pytest.mark.parametrize(
        "values",
        [
            [[1, 2], [3]],
            [pd.Timedelta(1, "s"), pd.Timedelta(2, "s")],
            list(TIMES),
        ],
        ids=["list", "duration", "datetime"],
    )
    def test_unsupported_column_types(self, tmp_path: Path, values):
        df = pd.DataFrame({"v": values}, index=TIMES)

        with pytest.raises(ValueError, match="can't be a processed channel"):
            write_processed_channel_parquet(df, tmp_path / "out.parquet")

    def test_nothing_written_when_invalid(self, tmp_path: Path):
        df = pd.DataFrame({"v": [[1, 2], [3]]}, index=TIMES)
        path = tmp_path / "out.parquet"

        with pytest.raises(ValueError, match="can't be a processed channel"):
            write_processed_channel_parquet(df, path)

        assert not path.exists()


class TestDefaultCode:
    def test_interactive(self, monkeypatch):
        monkeypatch.setitem(sys.modules, "__main__", types.ModuleType("__main__"))

        code = default_code()

        assert code.ref == "interactive"
        assert code.hash is None

    @pytest.mark.parametrize("layout", ["loose_ref", "packed_ref", "detached", "worktree"])
    def test_script_in_git_repository(self, monkeypatch, tmp_path: Path, layout: str):
        repo = tmp_path / "repo"
        repo.mkdir()
        _git(repo, "init", "-q")
        (repo / "jobs").mkdir()
        (repo / "jobs" / "health.py").write_text("print('hi')\n")
        _git(repo, "add", ".")
        _git(repo, "-c", "user.name=t", "-c", "user.email=t@t", "commit", "-qm", "x")
        commit = _git(repo, "rev-parse", "HEAD")
        checkout = repo
        if layout == "packed_ref":
            _git(repo, "pack-refs", "--all")
        elif layout == "detached":
            _git(repo, "checkout", "-q", "--detach")
        elif layout == "worktree":
            checkout = tmp_path / "worktree"
            _git(repo, "worktree", "add", "-q", "-b", "other", str(checkout))
        main = types.ModuleType("__main__")
        main.__file__ = str(checkout / "jobs" / "health.py")
        monkeypatch.setitem(sys.modules, "__main__", main)

        code = default_code()

        assert code.ref == f"jobs/health.py@{commit}"
        assert code.hash is not None
        assert len(code.hash) == 64

    def test_script_outside_git_repository(self, monkeypatch, tmp_path: Path):
        script = tmp_path / "health.py"
        script.write_text("print('hi')\n")
        main = types.ModuleType("__main__")
        main.__file__ = str(script)
        monkeypatch.setitem(sys.modules, "__main__", main)

        code = default_code()

        assert code.ref == str(script.resolve())
        assert code.hash is not None

    def test_edit_changes_hash(self, monkeypatch, tmp_path: Path):
        script = tmp_path / "health.py"
        main = types.ModuleType("__main__")
        main.__file__ = str(script)
        monkeypatch.setitem(sys.modules, "__main__", main)

        script.write_text("a = 1\n")
        first = default_code()
        script.write_text("a = 2\n")
        second = default_code()

        assert first.ref == second.ref
        assert first.hash != second.hash
