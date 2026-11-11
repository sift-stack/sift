"""Build the Parquet upload and the code identity for a processed channel write."""

from __future__ import annotations

import hashlib
import sys
from pathlib import Path
from typing import TYPE_CHECKING, Mapping

import numpy as np
import pandas as pd

from sift_client.sift_types.data_import import (
    ParquetDataColumn,
    ParquetFlatDatasetImportConfig,
    ParquetTimeColumn,
    TimeFormat,
)
from sift_client.sift_types.processed_channel import ProcessedChannelCode

if TYPE_CHECKING:
    from sift.data_imports.v2.data_imports_pb2 import ParquetFlatDatasetConfig

TIME_COLUMN_PATH = "__sift_time__"
"""Path of the time column in the uploaded file. Not a channel name, so it can't collide."""


def write_processed_channel_parquet(
    df: pd.DataFrame,
    path: Path,
    *,
    time_column: str | None = None,
    units: Mapping[str, str] | None = None,
) -> tuple[ParquetFlatDatasetConfig, int, int]:
    """Write ``df`` to a Parquet file and describe it for ``CreateProcessedChannels``.

    The time comes from ``time_column`` when set, otherwise from the DataFrame's index. Every
    other column becomes one channel, named after the column, with its data type taken from the
    column's type.

    Args:
        df: The data to write.
        path: Where to write the Parquet file.
        time_column: A datetime column holding each row's time. Defaults to the index.
        units: Units by channel name.

    Returns:
        A tuple of (flat dataset config, footer offset, footer length).

    Raises:
        ValueError: If the DataFrame has no data columns, a time that isn't a datetime, a
            column of an unsupported type, or units for a column it doesn't have.
    """
    try:
        import pyarrow as pa
        import pyarrow.parquet as pq

        from sift_client._internal.util.parquet import _arrow_to_sift_type, _is_time_type
    except ImportError as e:
        raise RuntimeError(
            "pyarrow is required to upload processed channels. "
            "Install it via `pip install sift-stack-py[parquet]`."
        ) from e

    from sift_client._internal.util.file import extract_parquet_footer

    if time_column is not None:
        if time_column not in df.columns:
            raise ValueError(f"time_column '{time_column}' is not a column of the DataFrame.")
        times = df[time_column]
        data = df.drop(columns=[time_column])
    else:
        times = df.index.to_series()
        data = df

    if not pd.api.types.is_datetime64_any_dtype(times):
        source = f"time_column '{time_column}'" if time_column else "the DataFrame's index"
        raise ValueError(
            f"{source} must hold datetimes, but has dtype {times.dtype}. "
            "Convert it with pd.to_datetime, or pass time_column."
        )
    if times.isna().any():
        raise ValueError("Every row needs a time, but the time has missing values.")

    names = [str(c) for c in data.columns]
    if not names:
        raise ValueError("The DataFrame has no data columns to write.")
    if len(set(names)) != len(names):
        raise ValueError("Every data column needs a unique name, since each one is a channel.")
    unknown_units = set(units or {}) - set(names)
    if unknown_units:
        raise ValueError(f"units names columns the DataFrame doesn't have: {sorted(unknown_units)}")

    # Naive times are treated as UTC. Times are written as unix nanoseconds so every unit and
    # time zone pandas supports reads back the same.
    times_utc = pd.to_datetime(times, utc=True)
    nanos = (times_utc - pd.Timestamp(0, tz="UTC")).to_numpy() // np.timedelta64(1, "ns")
    table = pa.Table.from_pandas(
        data.set_axis(names, axis=1).reset_index(drop=True), preserve_index=False
    )
    table = table.add_column(0, TIME_COLUMN_PATH, pa.array(nanos, type=pa.int64()))

    data_columns = []
    for field in table.schema:
        if field.name == TIME_COLUMN_PATH:
            continue
        data_type = _arrow_to_sift_type(field.type)
        nested = (
            pa.types.is_struct(field.type)
            or pa.types.is_list(field.type)
            or pa.types.is_large_list(field.type)
            or pa.types.is_map(field.type)
        )
        if (
            data_type is None
            or nested
            or _is_time_type(field.type)
            or pa.types.is_duration(field.type)
        ):
            raise ValueError(
                f"Column '{field.name}' has type {field.type}, which can't be a processed "
                "channel. Use numbers, booleans, strings or bytes."
            )
        data_columns.append(
            ParquetDataColumn(
                name=field.name,
                path=field.name,
                data_type=data_type,
                units=(units or {}).get(field.name, ""),
            )
        )

    pq.write_table(table, path)
    footer_bytes, footer_offset = extract_parquet_footer(path)

    config = ParquetFlatDatasetImportConfig(
        asset_name="",
        time_column=ParquetTimeColumn(
            path=TIME_COLUMN_PATH, format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS
        ),
        data_columns=data_columns,
    )
    return config._to_proto().flat_dataset, footer_offset, len(footer_bytes)


def default_code() -> ProcessedChannelCode:
    """Identify the code that is uploading, for when the caller passes no ``code``.

    The ref is the running script's path, relative to its git repository and suffixed with the
    commit when it's in one. The hash is the SHA-256 of the script, so an edit that isn't
    committed still changes the write's fingerprint. In a notebook or REPL there is no script
    file, so the ref is ``"interactive"`` and there is no hash; pass ``code`` there to tell
    writes apart.
    """
    script = _main_script()
    if script is None:
        return ProcessedChannelCode(ref="interactive")

    content_hash = hashlib.sha256(script.read_bytes()).hexdigest()
    git = _git_location(script)
    if git is None:
        return ProcessedChannelCode(ref=str(script), hash=content_hash)
    relative_path, commit = git
    return ProcessedChannelCode(ref=f"{relative_path}@{commit}", hash=content_hash)


def _main_script() -> Path | None:
    main = sys.modules.get("__main__")
    file = getattr(main, "__file__", None)
    if not file:
        return None
    path = Path(file).resolve()
    return path if path.is_file() else None


def _git_location(script: Path) -> tuple[str, str] | None:
    """Return the script's path within its git repository and the commit checked out.

    Reads ``.git`` rather than running ``git``: forking while gRPC's threads are running makes
    gRPC log a warning, and the client has always started them by the time this runs.
    """
    root = next((d for d in script.parents if (d / ".git").exists()), None)
    if root is None:
        return None
    try:
        git_dir = _git_dir(root / ".git")
        commit = _head_commit(git_dir) if git_dir is not None else None
    except OSError:
        return None
    if commit is None:
        return None
    return script.relative_to(root).as_posix(), commit


def _git_dir(dot_git: Path) -> Path | None:
    if dot_git.is_dir():
        return dot_git
    # In a worktree or submodule, .git is a file naming the real git directory.
    text = dot_git.read_text().strip()
    if not text.startswith("gitdir:"):
        return None
    path = Path(text[len("gitdir:") :].strip())
    return path if path.is_absolute() else (dot_git.parent / path).resolve()


def _head_commit(git_dir: Path) -> str | None:
    head = (git_dir / "HEAD").read_text().strip()
    if not head.startswith("ref:"):
        return head or None
    ref = head[len("ref:") :].strip()
    # A worktree's branches live in the main repository's git directory.
    commondir = git_dir / "commondir"
    common = (git_dir / commondir.read_text().strip()).resolve() if commondir.is_file() else git_dir
    for base in (git_dir, common):
        if (base / ref).is_file():
            return (base / ref).read_text().strip() or None
    packed = common / "packed-refs"
    if packed.is_file():
        for line in packed.read_text().splitlines():
            commit, _, name = line.partition(" ")
            if name == ref:
                return commit
    return None
