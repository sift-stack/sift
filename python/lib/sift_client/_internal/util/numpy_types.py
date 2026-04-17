from __future__ import annotations

import numpy as np

from sift_client.sift_types.channel import ChannelDataType

NUMPY_TO_SIFT_TYPE: dict[type, ChannelDataType] = {
    np.bool_: ChannelDataType.BOOL,
    np.int8: ChannelDataType.INT_32,
    np.int16: ChannelDataType.INT_32,
    np.int32: ChannelDataType.INT_32,
    np.int64: ChannelDataType.INT_64,
    np.uint8: ChannelDataType.UINT_32,
    np.uint16: ChannelDataType.UINT_32,
    np.uint32: ChannelDataType.UINT_32,
    np.uint64: ChannelDataType.UINT_64,
    np.float32: ChannelDataType.FLOAT,
    np.float64: ChannelDataType.DOUBLE,
    np.datetime64: ChannelDataType.INT_64,
    np.complex64: ChannelDataType.FLOAT,
    np.complex128: ChannelDataType.DOUBLE,
    np.str_: ChannelDataType.STRING,
    # HDF5/TDMS fixed-length strings are stored as np.bytes_; use STRING, not
    # BYTES (np.void below handles truly opaque binary data).
    np.bytes_: ChannelDataType.STRING,
    # Numpy uses object dtype for variable-length strings; TDMS/HDF5 files
    # cannot produce non-string object arrays.
    np.object_: ChannelDataType.STRING,
    np.void: ChannelDataType.BYTES,
}


def numpy_to_sift_type(dtype: np.dtype) -> ChannelDataType:
    """Map a numpy dtype to a Sift ChannelDataType."""
    sift_type = NUMPY_TO_SIFT_TYPE.get(dtype.type)
    if sift_type is None:
        raise ValueError(f"Unsupported numpy dtype: {dtype}")
    return sift_type
