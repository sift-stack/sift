from __future__ import annotations

from enum import Enum

from pydantic import BaseModel
from sift.exports.v1.exports_pb2 import ExportOutputFormat as ExportOutputFormatProto


class ExportOutputFormat(Enum):
    """Supported output formats for data exports.

    Attributes:
        CSV: Comma-separated values format.
        SUN: winplot format.
    """

    CSV = ExportOutputFormatProto.EXPORT_OUTPUT_FORMAT_CSV
    SUN = ExportOutputFormatProto.EXPORT_OUTPUT_FORMAT_SUN


class ChannelReference(BaseModel):
    """Maps a placeholder variable in a calculated channel expression to an actual channel.

    Example::

        ChannelReference(
            channel_reference="$1",                                       # must match the placeholder exactly as it appears in the expression
            channel_identifier="cbddaf97-3332-4666-80f2-a19be6a77eef",   # channel UUID
        )

    Attributes:
        channel_reference: The placeholder as it appears in the expression, i.e. $1, $2, etc.
        channel_identifier: The channel UUID.
    """

    channel_reference: str
    channel_identifier: str


class ExportCalculatedChannel(BaseModel):
    """An inline calculated channel to include in an export.

    Defines a formula-based channel that is computed at export time from existing channels.

    Example::

        ExportCalculatedChannel(
            name="speed_doubled",
            expression="$1 * 2",
            channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier="<channel-uuid>"),
            ],
            units="m/s",
        )

    Attributes:
        name: Display name for the calculated channel in the export.
        expression: The formula to compute, using $placeholder syntax for channel references.
        channel_references: Mappings from expression placeholders to actual channels.
        units: Optional unit label for the calculated channel.
    """

    name: str
    expression: str
    channel_references: list[ChannelReference]
    units: str | None = None
