"""
Module containing optional metadata types to provide to Sift when uploading a file attachment.
Though optional, providing this information could help improve quality of renders on the Sift app.
"""

from __future__ import annotations

from datetime import datetime
from typing import Any, Optional, Type

from google.protobuf.timestamp_pb2 import Timestamp
from sift.remote_files.v1.remote_files_pb2 import (
    ImageMetadata as ImageMetadataPb,
)
from sift.remote_files.v1.remote_files_pb2 import (
    VideoMetadata as VideoMetadataPb,
)
from typing_extensions import Self

from sift_py._internal.convert.json import AsJson
from sift_py._internal.convert.protobuf import AsProtobuf


class Metadata(AsJson): ...


class VideoMetadata(AsProtobuf, Metadata):
    """
    Metadata for video media-types i.e. any mimetypes of the following pattern: `video/*`.
    """

    width: int
    height: int
    duration_seconds: float
    timestamp: Optional[datetime]

    def __init__(
        self, width: int, height: int, duration_seconds: float, timestamp: Optional[datetime] = None
    ):
        self.width = width
        self.height = height
        self.duration_seconds = duration_seconds
        self.timestamp = timestamp

    def as_pb(self, klass: Type[VideoMetadataPb]) -> VideoMetadataPb:
        if self.timestamp is not None:
            timestamp_pb = Timestamp()
            timestamp_pb.FromDatetime(self.timestamp)
        else:
            timestamp_pb = None

        return klass(
            width=self.width,
            height=self.height,
            duration_seconds=self.duration_seconds,
            timestamp=timestamp_pb,
        )

    @classmethod
    def from_pb(cls, message: VideoMetadataPb) -> Self:
        return cls(
            width=message.width,
            height=message.height,
            duration_seconds=message.duration_seconds,
            timestamp=message.timestamp.ToDateTime(),  # type: ignore
        )

    def as_json(self) -> Any:
        timestamp = None if self.timestamp is None else self.timestamp.isoformat()
        return {
            "height": self.height,
            "width": self.width,
            "duration_seconds": self.duration_seconds,
            "timestamp": timestamp,
        }


class ImageMetadata(AsProtobuf, Metadata):
    """
    Metadata for image media-types i.e. any mimetypes of the following pattern: `image/*`.
    """

    width: int
    height: int

    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height

    def as_pb(self, klass: Type[ImageMetadataPb]) -> ImageMetadataPb:
        return klass(
            width=self.width,
            height=self.height,
        )

    @classmethod
    def from_pb(cls, message: ImageMetadataPb) -> Self:
        return cls(
            width=message.width,
            height=message.height,
        )

    def as_json(self) -> Any:
        return {
            "height": self.height,
            "width": self.width,
        }
