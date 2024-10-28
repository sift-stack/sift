"""
This module contains services to facilitate uploading and downloading file attachments.
It also provides utilities to easily query all file attachments for a given entity
which could be a run, annotation, or annotation logs. File attachment deletion is also supported.

Once files have been attached, they should be viewable on the Sift application, attached to their
respective entities. Below are various examples on how to leverage the `sift_py.file_attachment.service.FileAttachmentService`.

## Initializing the file attachment service

Unlike other services throughout `sift_py`, the `sift_py.file_attachment.service.FileAttachmentService` does rely on both
REST and gRPC APIs, so with that in mind we can initialize our service like so:

```python
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.file_attachment.service import FileAttachmentService
from sift_py.file_attachment.entity import Entity, EntityType
from sift_py.file_attachment.metadata import ImageMetadata
from sift_py.rest import SiftRestConfig

from sift.remote_files.v1.remote_files_pb2 import GetRemoteFileRequest
from sift.remote_files.v1.remote_files_pb2_grpc import RemoteFileServiceStub

rest_config: SiftRestConfig = {
    # Be sure to exclude the "https://" or "http://" scheme out of the uri
    "uri": rest_base_uri,
    "apikey": apikey,
}

sift_channel_config = SiftChannelConfig(uri=grpc_base_uri, apikey=apikey)

with use_sift_channel(sift_channel_config) as channel:
    file_attachment_service = FileAttachmentService(channel, rest_config)
    ...
```

With the service initialized we can now interact with the file attachments API.

## Various Examples

For demonstrative purposes we will upload an `mp4` file and attach to a run of `run_id`.
Once it is uploaded we will query all file attachments for a particular run and re-download
what we just uploaded.

```python

from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.file_attachment.service import FileAttachmentService
from sift_py.file_attachment.entity import Entity, EntityType
from sift_py.file_attachment.metadata import VideoMetadata
from sift_py.rest import SiftRestConfig

from sift.remote_files.v1.remote_files_pb2 import GetRemoteFileRequest
from sift.remote_files.v1.remote_files_pb2_grpc import RemoteFileServiceStub

...

with use_sift_channel(sift_channel_config) as channel:
    file_attachment_service = FileAttachmentService(channel, rest_config)

    run = entity=Entity(
        entity_id=run_id, # some arbitrary run ID that refers to an existing run
        entity_type=EntityType.RUN,
    )

    # uploading the file attachment and attaching it to a run of `run_id`.
    remote_file = file_attachment_service.upload_attachment(
        path="path/to/foo.mp4",
        entity=run,
        # Metatadata.. optional but recommended for optimal viewing in the application
        metadata=VideoMetadata(height=2160, width=3840, duration_seconds=5.5, timestamp=datetime(2024, 10, 19, 2, 22, 22),
        description="thrusters getting too hot" ,
    )

    # retrieving all of the file attachments for our run
    all_file_attachments = file_attachment_service.retrieve_attachments(run)

    # downloading our file_attachment and saving it to our current working dir
    file_attachment_service.download_attachment(remote_file)

    # downloading our file_attachment and saving it somewhere else with a different name
    file_attachment_service.download_attachment(remote_file, "somewhere/else/foo.mp4")

    # deleting out file attachment from Sift
    file_attachment_service.delete_file_attachments(remote_file_1, remote_file_2, remote_file_etc)
```

"""
