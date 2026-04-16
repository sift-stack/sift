from __future__ import annotations

from pathlib import Path

import h5py

from sift_client._internal.util.numpy_types import numpy_to_sift_type
from sift_client.sift_types.data_import import Hdf5DataColumn, Hdf5ImportConfig, TimeFormat

# Common HDF5 attribute names used to detect channel metadata.
_NAME_ATTRS = ["Name", "name", "Title", "title", "Sensor", "sensor", "Channel", "channel"]
_UNIT_ATTRS = ["Unit", "unit", "Units", "units"]
_DESCRIPTION_ATTRS = ["Description", "description"]


def _detect_attr(dataset: h5py.Dataset, candidates: list[str], default: str = "") -> str:
    """Return the first matching HDF5 attribute value, or *default*."""
    possible = [dataset.attrs.get(attr) for attr in candidates if dataset.attrs.get(attr)]
    return str(possible[0]) if possible else default


def detect_hdf5_config(file_path: str | Path) -> Hdf5ImportConfig:
    """Detect an HDF5 import config by inspecting the file's datasets.

    Traverses the HDF5 file and produces (time dataset, value dataset) pairs.
    For compound datasets with multiple fields, the first field is assumed to
    be time and remaining fields become value channels. For simple datasets,
    a root-level ``time`` dataset is used if present.
    """
    path = Path(file_path)

    with h5py.File(path, "r") as h5file:
        columns: list[Hdf5DataColumn] = []
        seen_names: set[str] = set()
        has_root_time = "time" in h5file

        def _visit(dataset_name: str, obj: object) -> None:
            if not isinstance(obj, h5py.Dataset):
                return

            # Skip root "time" dataset — it's used as the time source, not a value channel.
            if dataset_name == "time" and obj.parent == h5file:
                return

            n_fields = len(obj.dtype.names) if obj.dtype.names else 0

            if n_fields > 1:
                # Compound type: first field is time, remaining are value channels.
                for value_index in range(1, n_fields):
                    channel_name = _detect_attr(obj, _NAME_ATTRS, dataset_name)
                    if channel_name in seen_names:
                        channel_name = f"{channel_name}.{dataset_name}.{value_index}"

                    columns.append(
                        Hdf5DataColumn(
                            name=channel_name,
                            data_type=numpy_to_sift_type(obj.dtype[value_index]),
                            units=_detect_attr(obj, _UNIT_ATTRS),
                            description=_detect_attr(obj, _DESCRIPTION_ATTRS),
                            time_dataset=dataset_name,
                            value_dataset=dataset_name,
                            time_index=0,
                            value_index=0,
                            time_field=obj.dtype.names[0],
                            value_field=obj.dtype.names[value_index],
                        )
                    )
                    seen_names.add(channel_name)

            elif n_fields in (0, 1):
                # Single column. Use root "time" as time dataset if available.
                channel_name = _detect_attr(obj, _NAME_ATTRS, dataset_name)
                if channel_name in seen_names:
                    channel_name = f"{channel_name}.{dataset_name}"

                columns.append(
                    Hdf5DataColumn(
                        name=channel_name,
                        data_type=numpy_to_sift_type(obj.dtype),
                        units=_detect_attr(obj, _UNIT_ATTRS),
                        description=_detect_attr(obj, _DESCRIPTION_ATTRS),
                        time_dataset="time" if has_root_time else "",
                        value_dataset=dataset_name,
                        time_index=0,
                        value_index=0,
                    )
                )
                seen_names.add(channel_name)

        h5file.visititems(_visit)

        return Hdf5ImportConfig(
            asset_name="",
            time_format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS,
            data=columns,
        )
