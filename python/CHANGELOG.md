# Change Log
All notable changes to this project will be documented in this file.
 
This project adheres to [Semantic Versioning](http://semver.org/).

## [v0.1.0-rc.2] - July 1, 2024

Summary of changes:
- [Introduced automated ingestion request buffering to improve performance](https://github.com/sift-stack/sift/pull/65)
- [Added support for multi-config ingestion and creating new flows at run-time](https://github.com/sift-stack/sift/pull/66)
- Added methods that combine request creation and ingestion into a single-step.
- Updates to documentation.

For in-depth documentation please see the [documentation section of the README](https://github.com/sift-stack/sift/tree/main/python#documentation) for instructions
on how to build the documentation locally.

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
