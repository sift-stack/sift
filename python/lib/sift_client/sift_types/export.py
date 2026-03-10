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
            channel_reference="velocity",                    # placeholder used as $velocity in the expression
            channel_identifier="vehicle.engine.velocity",    # the actual channel
        )

    Attributes:
        channel_reference: The placeholder name used in the expression (without the $ prefix).
        channel_identifier: The fully qualified channel name or channel ID.
    """

    channel_reference: str
    channel_identifier: str


class ExportCalculatedChannel(BaseModel):
    """An inline calculated channel to include in an export.

    Defines a formula-based channel that is computed at export time from existing channels.

    Example::

        CalculatedChannel(
            name="speed_doubled",
            expression="$velocity * 2",
            channel_references=[
                ChannelReference(channel_reference="velocity", channel_identifier="vehicle.engine.velocity"),
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
