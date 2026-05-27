"""HDF5 schema detection.

HDF5 files have no single canonical layout, so detection is parameterized
by an ``Hdf5Schema``: ``ONE_D`` (per-group time dataset + sibling 1D values,
with an ancestor walk-up), ``TWO_D`` (``[N, 2]`` datasets where col 0 is
time), or ``COMPOUND`` (struct-like datasets whose first field is time).
Each detector walks every dataset in the file recursively; datasets that
don't fit the chosen schema are not included in the resulting config."""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Callable

import h5py
import numpy as np

from sift_client._internal.util.numpy_types import numpy_to_sift_type
from sift_client.sift_types.data_import import (
    Hdf5DataColumn,
    Hdf5ImportConfig,
    Hdf5Schema,
    TimeFormat,
)

# Heuristic attribute names for channel metadata, in priority order. The
# first non-empty value found on a dataset wins; missing attributes resolve
# to empty strings.
_NAME_ATTRS = ("Name", "name", "Title", "title", "Sensor", "sensor", "Channel", "channel")
_UNIT_ATTRS = ("Unit", "unit", "Units", "units")
_DESCRIPTION_ATTRS = ("Description", "description")

# Per-group time dataset names, case-insensitive, in priority order.
_TIME_DATASET_NAMES = ("time", "timestamp", "timestamps", "ts")


@dataclass(frozen=True)
class _DetectedDataset:
    path: str
    """Slash-joined path from the file root, e.g. ``group_a/sub/value``."""
    group_path: str
    """Parent group path, ``""`` for root-level datasets."""
    name: str
    """Final path segment, e.g. ``value``."""
    dataset: h5py.Dataset


def _read_string_attr(dataset: h5py.Dataset, candidates: tuple[str, ...]) -> str:
    """Return the first non-empty string attribute among `candidates`."""
    for name in candidates:
        if name not in dataset.attrs:
            continue
        value = dataset.attrs[name]
        if isinstance(value, bytes):
            value = value.decode("utf-8", errors="replace")
        if isinstance(value, str) and value:
            return value
        # h5py returns multi-element string attrs as ndarrays; take the first.
        if isinstance(value, np.ndarray) and value.size > 0:
            first = value.flat[0]
            if isinstance(first, bytes):
                first = first.decode("utf-8", errors="replace")
            if isinstance(first, str) and first:
                return first
    return ""


def _read_channel_metadata(dataset: h5py.Dataset) -> tuple[str, str, str]:
    """Return ``(name, units, description)`` discovered from HDF5 attributes."""
    return (
        _read_string_attr(dataset, _NAME_ATTRS),
        _read_string_attr(dataset, _UNIT_ATTRS),
        _read_string_attr(dataset, _DESCRIPTION_ATTRS),
    )


def _is_compound(dataset: h5py.Dataset) -> bool:
    return dataset.dtype.names is not None and len(dataset.dtype.names) > 1


def _is_1d_non_compound(dataset: h5py.Dataset) -> bool:
    return not _is_compound(dataset) and len(dataset.shape) == 1


def _is_2d_n_by_2(dataset: h5py.Dataset) -> bool:
    return not _is_compound(dataset) and len(dataset.shape) == 2 and dataset.shape[1] == 2


def _path_to_channel_name(path: str) -> str:
    """Sift renders dotted names hierarchically, so ``group1/current`` becomes
    ``group1.current``, with ``current`` shown under a ``group1`` folder."""
    return path.replace("/", ".")


def _make_name_deduper() -> Callable[[str, str], str]:
    """Return a callable that resolves duplicate channel names by appending
    the dataset's dotted path. First claim of a name wins; later claims of
    the same name get the fallback suffix appended."""
    used: set[str] = set()

    def dedupe(base_name: str, fallback_suffix: str) -> str:
        name = f"{base_name}.{fallback_suffix}" if base_name in used else base_name
        used.add(name)
        return name

    return dedupe


def _collect_datasets(h5file: h5py.File) -> list[_DetectedDataset]:
    """Recursively walk every dataset in the file, preserving full paths."""
    out: list[_DetectedDataset] = []

    def visit(path: str, obj: object) -> None:
        if not isinstance(obj, h5py.Dataset):
            return
        slash = path.rfind("/")
        group_path = "" if slash < 0 else path[:slash]
        name = path if slash < 0 else path[slash + 1 :]
        out.append(_DetectedDataset(path=path, group_path=group_path, name=name, dataset=obj))

    h5file.visititems(visit)
    return out


def _identify_time_dataset(group: list[_DetectedDataset]) -> _DetectedDataset | None:
    """Pick the per-group time dataset by name, case-insensitive, in priority
    order. Only 1D non-compound datasets are eligible. Returns ``None`` if no
    candidate matches, in which case callers fall back to an ancestor group's
    time before giving up."""
    one_d = [d for d in group if _is_1d_non_compound(d.dataset)]
    for candidate in _TIME_DATASET_NAMES:
        for ds in one_d:
            if ds.name.lower() == candidate:
                return ds
    return None


def _group_by_parent(datasets: list[_DetectedDataset]) -> dict[str, list[_DetectedDataset]]:
    out: dict[str, list[_DetectedDataset]] = {}
    for ds in datasets:
        out.setdefault(ds.group_path, []).append(ds)
    return out


def _resolve_ancestor_time(group_path: str, per_group_time: dict[str, str]) -> str:
    """Return the closest-ancestor time dataset path for ``group_path``,
    walking up to the root. Empty string if no ancestor has one."""
    cursor: str | None = group_path
    while cursor is not None:
        found = per_group_time.get(cursor)
        if found:
            return found
        if cursor == "":
            return ""
        slash = cursor.rfind("/")
        cursor = "" if slash < 0 else cursor[:slash]
    return ""


def _build_one_d_configs(datasets: list[_DetectedDataset]) -> list[Hdf5DataColumn]:
    """1D non-compound schema: at each group, pick a time dataset (by name)
    and pair every other 1D dataset in that group as a value channel.
    Datasets that aren't 1D non-compound are not included."""
    columns: list[Hdf5DataColumn] = []
    dedupe = _make_name_deduper()

    one_d = [ds for ds in datasets if _is_1d_non_compound(ds.dataset)]
    grouped = _group_by_parent(one_d)

    # First pass: each group's own time dataset (if any).
    per_group_time: dict[str, str] = {}
    for group_path, group in grouped.items():
        time_ds = _identify_time_dataset(group)
        if time_ds is not None:
            per_group_time[group_path] = time_ds.path

    for group_path, group in grouped.items():
        own_time_path = per_group_time.get(group_path)
        time_path = own_time_path or _resolve_ancestor_time(group_path, per_group_time)
        for ds in group:
            if own_time_path and ds.path == own_time_path:
                continue
            name, units, description = _read_channel_metadata(ds.dataset)
            fallback = _path_to_channel_name(ds.path)
            columns.append(
                Hdf5DataColumn(
                    name=dedupe(name or fallback, fallback),
                    data_type=numpy_to_sift_type(ds.dataset.dtype),
                    units=units,
                    description=description,
                    time_dataset=time_path,
                    value_dataset=ds.path,
                    time_index=0,
                    value_index=0,
                )
            )

    return columns


def _build_two_d_configs(datasets: list[_DetectedDataset]) -> list[Hdf5DataColumn]:
    """2D schema: every dataset with shape ``[N, 2]`` becomes one channel
    (col 0 = time, col 1 = value). Other shapes are not included."""
    columns: list[Hdf5DataColumn] = []
    dedupe = _make_name_deduper()

    for ds in datasets:
        if not _is_2d_n_by_2(ds.dataset):
            continue
        name, units, description = _read_channel_metadata(ds.dataset)
        fallback = _path_to_channel_name(ds.path)
        columns.append(
            Hdf5DataColumn(
                name=dedupe(name or fallback, fallback),
                data_type=numpy_to_sift_type(ds.dataset.dtype),
                units=units,
                description=description,
                time_dataset=ds.path,
                value_dataset=ds.path,
                time_index=0,
                value_index=1,
            )
        )

    return columns


def _build_compound_configs(datasets: list[_DetectedDataset]) -> list[Hdf5DataColumn]:
    """Compound schema: every compound dataset becomes one channel per
    non-time member. First member is time. Non-compound datasets are not included."""
    columns: list[Hdf5DataColumn] = []
    dedupe = _make_name_deduper()

    for ds in datasets:
        if not _is_compound(ds.dataset):
            continue
        field_names = ds.dataset.dtype.names
        assert field_names is not None  # guaranteed by _is_compound
        time_field = field_names[0]
        value_fields = field_names[1:]
        name, units, description = _read_channel_metadata(ds.dataset)
        dataset_name = name or _path_to_channel_name(ds.path)

        for value_field in value_fields:
            base_name = f"{dataset_name}.{value_field}" if len(value_fields) > 1 else dataset_name
            fallback_suffix = f"{_path_to_channel_name(ds.path)}.{value_field}"
            columns.append(
                Hdf5DataColumn(
                    name=dedupe(base_name, fallback_suffix),
                    data_type=numpy_to_sift_type(ds.dataset.dtype[value_field]),
                    units=units,
                    description=description,
                    time_dataset=ds.path,
                    value_dataset=ds.path,
                    time_index=0,
                    value_index=0,
                    time_field=time_field,
                    value_field=value_field,
                )
            )

    return columns


_BUILDERS: dict[Hdf5Schema, Callable[[list[_DetectedDataset]], list[Hdf5DataColumn]]] = {
    Hdf5Schema.ONE_D: _build_one_d_configs,
    Hdf5Schema.TWO_D: _build_two_d_configs,
    Hdf5Schema.COMPOUND: _build_compound_configs,
}


def detect_hdf5_config(file_path: str | Path, schema: Hdf5Schema) -> Hdf5ImportConfig:
    """Detect an HDF5 import config under the given ``schema``. Datasets that
    don't fit the chosen schema are not included."""
    path = Path(file_path)
    with h5py.File(path, "r") as h5file:
        columns = _BUILDERS[schema](_collect_datasets(h5file))

    return Hdf5ImportConfig(
        asset_name="",
        time_format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS,
        data=columns,
    )
