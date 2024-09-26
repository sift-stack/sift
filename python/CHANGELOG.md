# Change Log
All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](http://semver.org/).

## [v0.2.1] - September 26, 2024

Summary of changes:
- [add missing double channel computation](https://github.com/sift-stack/sift/pull/92)
- [update protos](https://github.com/sift-stack/sift/pull/94)
- [update protos](https://github.com/sift-stack/sift/pull/95)
- [csv example ingestion](https://github.com/sift-stack/sift/pull/96)
- [update example](https://github.com/sift-stack/sift/pull/97)
- [Add multiple CSV ingestion example](https://github.com/sift-stack/sift/pull/98)
- [Add metadata arg to use_sift_channel](https://github.com/sift-stack/sift/pull/99)

## [v0.2.0] - September 3, 2024

Summary of changes:
- [Add pydantic models for the sift grafana plugin queries](https://github.com/sift-stack/sift/pull/78)
- [Lowered minimum version requirements for various dependencies and dependency cleanup](https://github.com/sift-stack/sift/pull/77)
- [Added py.typed to modules that utilize type annotations](https://github.com/sift-stack/sift/pull/81)
- [Added services derived from protobuf for the following APIs](https://github.com/sift-stack/sift/pull/84)
    * Remote files
    * Rule versions
    * Saved searches
- [Added helpful validations when processing Sift API URLs](https://github.com/sift-stack/sift/pull/87)
- [Added support for gRPC keep-alive](https://github.com/sift-stack/sift/pull/86)
- [Added service to upload and download file attachments](https://github.com/sift-stack/sift/pull/85)
- [Added timer-based flushing for buffered ingestion and custom error-handling](https://github.com/sift-stack/sift/pull/88)
- [Added functionality to generate a channel value from a channel config](https://github.com/sift-stack/sift/pull/89)

The following section will cover some of the more notable features in depth. Please refer to the [documentation](https://docs.siftstack.com/sift_py/sift_py.html) for
even more detail.

### Table of Contents
- [Keepalive](#keepalive)
- [File attachments](#file-attachments)
- [Timer based flushing for buffered ingestion](#timer-based-flushing-for-buffered-ingestion)
- [Custom error handling for buffered ingestion](#custom-error-handling-for-buffered-ingestion)

### Keepalive

Long-lived connections are generally are at risk of being closed due to idle timeouts if they are idle for a particular duration. In other words, if there
is no data being exchanged on the connection for a duration specified by a load balancer's idle timeout, the connection will be closed. In order to
ensure that this does not happen you can opt into enabling the HTTP/2 PING-based keepalive mechanism either on your own gRPC channel or the one
that Sift provides. To configure keep-alive using the Sift-provided gRPC channel:

```python
from sift_py.grpc.transport import use_sift_channel, SiftChannelConfig

sift_channel_config = SiftChannelConfig(
    uri=uri,
    apikey=apikey,
    enable_keepalive=True,
)

with use_sift_channel(sift_channel_config) as channel:
    ...
```

This uses default values set in the `sift_py.grpc.keepalive` module. If you'd like to configure your own keepalive parameters you could
also do the following:

```python
from sift_py.grpc.transport import use_sift_channel, SiftChannelConfig
from sift_py.grpc.keepalive import KeepaliveConfig

sift_channel_config = SiftChannelConfig(
    uri=uri,
    apikey=apikey,
    enable_keepalive=KeepaliveConfig(
        keepalive_time_ms=keepalive_time_ms,
        keepalive_timeout_ms=keepalive_timeout_ms,
        keepalive_permit_without_calls=keepalive_permit_without_calls,
        max_pings_without_data=max_pings_without_data,
    ),
)

with use_sift_channel(sift_channel_config) as channel:
    ...
```

### File attachments

There is now a `sift_py.file_attachment` module that contains utilities to programmatically upload and download file attachments.
Files currently can be attached to various entities such as runs, annotations, and annotation logs. Various video and image formats
are currently supported. Below is an example demonstrating how to upload a file attachment and programmatically downloading it.

```python
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.file_attachment.service import FileAttachmentService
from sift_py.file_attachment.entity import Entity, EntityType
from sift_py.file_attachment.metadata import VideoMetadata
from sift_py.rest import SiftRestConfig

from sift.remote_files.v1.remote_files_pb2 import GetRemoteFileRequest
from sift.remote_files.v1.remote_files_pb2_grpc import RemoteFileServiceStub

# Get API credentials setup
...

with use_sift_channel(sift_channel_config) as channel:
    file_attachment_service = FileAttachmentService(channel, rest_config)

    run = entity=Entity(
        entity_id=run_id, # some arbitrary run ID that refers to an existing run
        entity_type=EntityType.RUN,
    )

    # uploading the file attachment and attaching it to a run of `run_id`
    remote_file = file_attachment_service.upload_attachment(
        path="path/to/foo.mp4",
        entity=run,
        # Metatadata.. optional but recommended for optimal viewing in the application
        metadata=VideoMetadata(height=2160, width=3840, duration_seconds=5.5),
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

### Timer based flushing for buffered ingestion

Previously the [buffered ingestion API](https://docs.siftstack.com/sift_py/sift_py.html#buffered-ingestion) flushes a buffer whenever any one
of the following conditions are met:
- The caller manually calls `flush`
- The buffer gets filled
- The `with`-block associated with the buffered ingestion service as a context-manager goes out of scope.
- An exception is raised inside of the aforementioned `with`-block.

Note that the last two points only apply if the buffered ingestion service is used as a context manager like so:

```python
with ingestion_service.buffered_ingestion() as buffered_ingestion:
    ...
```

Now there is support to periodically flush the buffer regardless of whether or not the buffer is filled. To configure a timer to flush periodically:

```python
with ingestion_service.buffered_ingestion(flush_interval_sec=3.2) as buffered_ingestion:
    ...
```

This will configure the buffered ingestion service to flush its buffer every `3.2` seconds. If the buffer happens to be filled before the timer elapses,
then the timer will reset.

### Custom error handling for buffered ingestion

Previously when using the buffered ingestion service as a context manager, any exception that gets raised within the `with`-block will result
in the service attempting to flush one more time. The caller can now customize error-handling behavior by passing in a function handler that takes
in three arguments: the error that's raised, the buffer containing the remaining requests that weren't ingested, and a function that when called will
attempt to flush the buffer.


```python
# Custom code to run when error
def on_error_calback(err, buffer, flush):
    # Maybe try to save contents of buffer to disk
    ...
    # Try once more to flush the buffer
    flush()

with ingestion_service.buffered_ingestion(on_error=on_error_calback) as buffered_ingestion:
    ...
```

## [v0.1.1] - July 17, 2024

Summary of changes:
- Extend support for Python 3.8

## [v0.1.0] - July 12, 2024

Summary of changes:
- Promote to `v0.1.0` from release candidate state. 
- [Add module to download telemetry](https://github.com/sift-stack/sift/pull/72)

In addition to the changes above, documentation is now also available online:
- [Online documentation](https://docs.siftstack.com/sift_py/sift_py.html)
- [Section on how to download telemetry](https://docs.siftstack.com/sift_py/sift_py/data.html)

## [v0.1.0-rc.3] - July 3, 2024

Summary of changes:
- [Automated gRPC retries if an unexpected gRPC status code is returned and if a connection unexpectedly terminates](https://github.com/sift-stack/sift/pull/70)

The following are some gRPC error codes that can happen due to external factors that Sift doesn't directly control:
- `UNKNOWN`
- `UNAVAILABLE`
- `ABORTED`
- `DEADLINE_EXCEEDED`

They are the source of common disruptions, particularly during ingestion, and so this mechanism will automatically retry failed RPCs over an existing connection
or will establish a new one if necessary.

## [v0.1.0-rc.2] - July 1, 2024

Summary of changes:
- [Introduced automated ingestion request buffering to improve performance](https://github.com/sift-stack/sift/pull/65)
- [Added support for multi-config ingestion and creating new flows at run-time](https://github.com/sift-stack/sift/pull/66)
- Added methods that combine request creation and ingestion into a single-step.
- Updates to documentation.

For in-depth documentation please see the [documentation section of the README](https://github.com/sift-stack/sift/tree/main/python#documentation) for instructions
on how to build the documentation locally.

### Table of Contents
- [Combining Request Generation and Ingestion into a Single Step](#combining-request-generation-and-ingestion-into-a-single-step)
- [Request Buffering](#request-buffering)
- [Creating New Flows on the Fly](#creating-new-flows-on-the-fly)
- [Multi-config Ingestion](#multi-config-ingestion)

### Combining Request Generation and Ingestion into a Single Step

Previously to ingest a single request you needed to do the following:

```python
request = ingestion_service.try_create_ingestion_request(
    flow_name="logs",
    timestamp=timestamp,
    channel_values=[
        {
            "channel_name": "log",
            "value": string_value("some log"),
        },
    ],
)
ingestion_service.ingest(request)
```

Now you can combine both steps using either of the following APIs:
- [try_ingest_flows](https://github.com/sift-stack/sift/blob/e7e59e63344059fb232ce883d269c479e1857f09/python/lib/sift_py/ingestion/service.py#L145)
- [ingest_flows](https://github.com/sift-stack/sift/blob/e7e59e63344059fb232ce883d269c479e1857f09/python/lib/sift_py/ingestion/service.py#L138)

```python
ingestion_service.try_ingest_flows({
    "flow_name": "log",
    "timestamp": timestamp,
    "channel_values": [
        {
            "channel_name": "log",
            "value": string_value("some string")
        },
    ],
})
```

You can also send multiple flows:

```python
# Send data for both logs and readings flows
ingestion_service.try_ingest_flows(
    {
        "flow_name": "readings",
        "timestamp": datetime.now(timezone.utc),
        "channel_values": [
            {
                "channel_name": "velocity",
                "component": "mainmotor",
                "value": double_value(10),
            },
            {
                "channel_name": "voltage",
                "value": int32_value(5),
            },
            {
                "channel_name": "vehicle_state",
                "value": enum_value(2),
            },
        ],
    },
    {
        "flow_name": "logs",
        "timestamp": datetime.now(timezone.utc),
        "channel_values": [
            {
                "channel_name": "logs",
                "value": string_value("INFO: some message")
            },
        ],
    },
)
```

### Request Buffering

Requests are now automatically buffered using the [buffered ingestion API](https://github.com/sift-stack/sift/blob/1fbd2eb02d484b277a8b799940587b0f11e291da/python/lib/sift_py/ingestion/service.py#L152).
Using this may significantly improve performance as it allows serialization and ingestion to occur in batches:

```python
# Defaults to a buffer size of `sift_py.ingestion.buffer.DEFAULT_BUFFER_SIZE` requests.
with ingestion_service.buffered_ingestion() as buffered_ingestion:
    buffered_ingestion.try_ingest_flows(*lots_of_flows)
    buffered_ingestion.try_ingest_flows(*lots_more_flows)

# Custom buffer size of 750 requests
with ingestion_service.buffered_ingestion(750) as buffered_ingestion:
    buffered_ingestion.try_ingest_flows(*lots_of_flows)
    buffered_ingestion.try_ingest_flows(*lots_more_flows)
```

Once the with-block ends the remaining requests will be automatically flushed and ingested, but flushing may also
be done manually:

```python
with ingestion_service.buffered_ingestion() as buffered_ingestion:
    buffered_ingestion.try_ingest_flows(*lots_of_flows)
    buffered_ingestion.flush()
```

Contrast this with regular ingestion:

```python
ingestion_service.try_ingest_flows(*lots_of_flows)
ingestion_service.try_ingest_flows(*lots_more_flows)
```

### Creating New Flows on the Fly

If there is a flow you need to create on the fly which wasn't declared in your initial telemetry config,
you may use either of the following APIs:
- `try_create_flow`
- `create_flow`

```python
new_flow_config = FlowConfig(
    name="my_new_flow", channels=[ChannelConfig("new_channel", ChannelDataType.DOUBLE)]
)
ingestion_service.try_create_flow(new_flow_config)
```

### Multi-config Ingestion

There is now an ergonomic utility class, [IngestionServicesManager](https://github.com/sift-stack/sift/blob/e7e59e63344059fb232ce883d269c479e1857f09/python/lib/sift_py/ingestion/manager.py#L15),
that allows users to manage telemetry for multiple configs:

```python
manager = IngestionServicesManager.from_telementry_configs(grpc_channel, {
    "config_a": config_a,
    "config_b": config_b,
})

with manager.ingestion_service("config_a") as config_a:
    config_a.try_ingest_flow(...)

with manager.ingestion_service("config_b") as config_b:
    config_b.try_ingest_flow(...)
```
