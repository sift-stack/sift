import json

import pytest
from pytest_mock import MockFixture
from sift.remote_files.v1.remote_files_pb2 import GetRemoteFileResponse, RemoteFile

from sift_py._internal.test_util.channel import MockChannel
from sift_py.file_attachment.entity import Entity, EntityType
from sift_py.file_attachment.metadata import ImageMetadata
from sift_py.file_attachment.service import FileAttachmentService


class MockResponse:
    status_code: int
    text: str

    def __init__(self, status_code: int, text: str):
        self.status_code = status_code
        self.text = text

    def json(self):
        return json.loads(self.text)


class MockMultipartEncoder:
    @property
    def content_type(self):
        return "multipart/form-data"


def test_file_attachments_service_upload_validate_uri():
    mock_channel = MockChannel()

    svc = FileAttachmentService(
        mock_channel,
        {
            "uri": "https://some_uri.com",
            "apikey": "123123123",
        },
    )

    assert svc is not None

    svc = FileAttachmentService(
        mock_channel,
        {
            "uri": "some_uri.com",
            "apikey": "123123123",
        },
    )

    assert svc is not None


def test_file_attachments_service_upload_validate_path(mocker: MockFixture):
    mock_channel = MockChannel()

    mock_path_is_file = mocker.patch("sift_py.file_attachment._internal.upload.Path.is_file")
    mock_path_is_file.return_value = False

    with pytest.raises(Exception, match="does not point to a regular file"):
        svc = FileAttachmentService(
            mock_channel,
            {
                "uri": "some_uri.com",
                "apikey": "123123123",
            },
        )

        svc.upload_attachment(
            path="some_image.png.gz",
            entity=Entity(
                entity_id="123-123-123",
                entity_type=EntityType.ANNOTATION_LOG,
            ),
            metadata=ImageMetadata(
                width=16,
                height=9,
            ),
        )


def test_file_attachments_service_upload_validate_mimetype(mocker: MockFixture):
    mock_channel = MockChannel()

    mock_path_is_file = mocker.patch("sift_py.file_attachment._internal.upload.Path.is_file")
    mock_path_is_file.return_value = True

    with pytest.raises(Exception, match="MIME"):
        svc = FileAttachmentService(
            mock_channel,
            {
                "uri": "some_uri.com",
                "apikey": "123123123",
            },
        )

        svc.upload_attachment(
            path="some_image.asdlkjfh",
            entity=Entity(
                entity_id="123-123-123",
                entity_type=EntityType.ANNOTATION_LOG,
            ),
            metadata=ImageMetadata(
                width=16,
                height=9,
            ),
        )


def test_file_attachments_service_upload_returns_remote_file(mocker: MockFixture):
    mock_channel = MockChannel()

    mock_path_is_file = mocker.patch("sift_py.file_attachment._internal.upload.Path.is_file")
    mock_path_is_file.return_value = True

    mocker.patch(
        "sift_py.file_attachment._internal.upload.open",
        mocker.mock_open(read_data=b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR"),
    )

    mock_multipart_encoder = mocker.patch(
        "sift_py.file_attachment._internal.upload.MultipartEncoder"
    )
    mock_multipart_encoder.return_value = MockMultipartEncoder()

    mock_requests_post = mocker.patch("sift_py.rest.requests.Session.post")
    mock_requests_post.return_value = MockResponse(
        status_code=200, text=json.dumps({"remoteFile": {"remoteFileId": "abc"}})
    )

    svc = FileAttachmentService(
        mock_channel,
        {
            "uri": "some_uri.com",
            "apikey": "123123123",
        },
    )

    mock_get_remote_file = mocker.patch.object(
        svc._remote_file_service_stub,
        "GetRemoteFile",
        return_value=GetRemoteFileResponse(remote_file=RemoteFile(remote_file_id="abc")),
    )

    remote_file = svc.upload_attachment(
        path="some_image.png.gz",
        entity=Entity(
            entity_id="123-123-123",
            entity_type=EntityType.ANNOTATION_LOG,
        ),
        metadata=ImageMetadata(
            width=16,
            height=9,
        ),
    )
    mock_get_remote_file.assert_called_once()
    mock_multipart_encoder.assert_called_once()
    mock_requests_post.assert_called_once()

    assert remote_file.remote_file_id == "abc"
