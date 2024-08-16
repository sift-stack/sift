"""
Module containing optional metadata types to provide to Sift when uploading a file attachment.
Though optional, providing this information could help improve quality of renders on the Sift app.
"""

from __future__ import annotations

from typing import Any, Type

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

    def __init__(self, width: int, height: int, duration_seconds: float):
        self.width = width
        self.height = height
        self.duration_seconds = duration_seconds

    def as_pb(self, klass: Type[VideoMetadataPb]) -> VideoMetadataPb:
        return klass(
            width=self.width,
            height=self.height,
            duration_seconds=self.duration_seconds,
        )

    @classmethod
    def from_pb(cls, message: VideoMetadataPb) -> Self:
        return cls(
            width=message.width,
            height=message.height,
            duration_seconds=message.duration_seconds,
        )

    def as_json(self) -> Any:
        return {
            "height": self.height,
            "width": self.width,
            "duration_seconds": self.duration_seconds,
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
