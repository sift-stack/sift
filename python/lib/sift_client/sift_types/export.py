from __future__ import annotations

from enum import Enum

from sift.exports.v1.exports_pb2 import ExportOutputFormat as ExportOutputFormatProto


class ExportOutputFormat(Enum):
    """Supported output formats for data exports.

    Attributes:
        CSV: Comma-separated values format.
        SUN: Sun (WinPlot) format (not used in certain environments).
        PARQUET: Apache Parquet columnar storage format.
    """

    CSV = ExportOutputFormatProto.EXPORT_OUTPUT_FORMAT_CSV
    SUN = ExportOutputFormatProto.EXPORT_OUTPUT_FORMAT_SUN
    PARQUET = ExportOutputFormatProto.EXPORT_OUTPUT_FORMAT_PARQUET
