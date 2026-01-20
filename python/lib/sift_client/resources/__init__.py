"""Sift Resources - API interfaces for interacting with Sift services.

This module provides high-level API interfaces for interacting with Sift resources.
Each resource API provides methods for common operations like listing, getting, creating,
updating, and archiving resources.

## Overview

Resource APIs are the primary way to interact with Sift services. They provide:

- **Type-safe methods** with full IDE autocomplete support
- **Automatic type conversion** between protobuf and Python types
- **Flexible filtering** using CEL (Common Expression Language) queries
- **Both sync and async** versions of most APIs
- **Consistent interface** across all resource types

## Synchronous vs Asynchronous APIs

All resource APIs are available in both synchronous and asynchronous versions:

- **Synchronous APIs** (e.g., `AssetsAPI`) - Ideal for scripts, notebooks, and simple applications
- **Asynchronous APIs** (e.g., `AssetsAPIAsync`) - Ideal for high-performance applications with concurrent operations

### Example Usage

```python
from sift_client import SiftClient

client = SiftClient(api_key="...", grpc_url="...", rest_url="...")

# Synchronous API usage
asset = client.assets.get(asset_id="asset123")
runs = client.runs.list_(assets=[asset.id_], limit=10)

# Asynchronous API usage
async def get_data():
    asset = await client.async_.assets.get(asset_id="asset123")
    runs = await client.async_.runs.list_(assets=[asset.id_], limit=10)
    return asset, runs
```

## Common Methods

Most resource APIs provide a consistent set of methods:

### Query Methods

- `get()` - Retrieve a single resource by ID or unique identifier
- `list_()` - Retrieve multiple resources with optional filtering
- `find()` - Find a single resource matching criteria (raises error if multiple found)

### Modification Methods

- `create()` - Create a new resource (where applicable)
- `update()` - Update an existing resource
- `archive()` - Archive a resource (soft delete)
- `unarchive()` - Restore an archived resource

## Filtering and Querying

Resource APIs support powerful filtering capabilities:

### Common Filters

- **Name filters**: `name`, `name_contains`, `name_regex`
- **Time filters**: `created_after`, `created_before`, `modified_after`, `modified_before`
- **User filters**: `created_by`, `modified_by`
- **Metadata filters**: `tags`, `metadata`
- **Archive filters**: `include_archived`

### Resource-Specific Filters

Each resource API may have additional filters:

- **Runs**: `start_time_after`, `duration_greater_than`, `is_stopped`
- **Channels**: `asset`, `run`
- **Rules**: `asset_ids`, `asset_tag_ids`

### Example: Advanced Filtering

```python
from datetime import datetime, timedelta

# Complex run query
runs = client.runs.list_(
    assets=["asset123"],
    start_time_after=datetime.now() - timedelta(days=7),
    duration_greater_than=timedelta(hours=1),
    is_stopped=True,
    tags=["production"],
    order_by="start_time desc",
    limit=20
)

# Channel search with regex
channels = client.channels.list_(
    asset="asset123",
    name_regex="sensor_[0-9]+_temp",
    limit=100
)
```

## Data Retrieval

The `ChannelsAPI` provides methods for retrieving time-series data:

```python
from datetime import datetime, timedelta

# Get channel data as pandas DataFrames
channels = client.channels.list_(asset="asset123", limit=5)
data = client.channels.get_data(
    channels=channels,
    run="run123",
    start_time=datetime.now() - timedelta(hours=1),
    end_time=datetime.now(),
    limit=10000
)

# data is a dict mapping channel names to DataFrames
for channel_name, df in data.items():
    print(f"{channel_name}: {len(df)} data points")
    print(df.head())
```

## Async Context Usage

When using async APIs, ensure proper async context:

```python
import asyncio
from sift_client import SiftClient

async def main():
    client = SiftClient(api_key="...", grpc_url="...", rest_url="...")

    # Use async_ accessor for async APIs
    assets = await client.async_.assets.list_(limit=10)

    # Concurrent operations
    asset_task = client.async_.assets.get(asset_id="asset123")
    runs_task = client.async_.runs.list_(limit=10)

    asset, runs = await asyncio.gather(asset_task, runs_task)

    return asset, runs

# Run the async function
result = asyncio.run(main())
```
"""

from sift_client.resources.assets import AssetsAPIAsync
from sift_client.resources.calculated_channels import CalculatedChannelsAPIAsync
from sift_client.resources.channels import ChannelsAPIAsync
from sift_client.resources.file_attachments import FileAttachmentsAPIAsync
from sift_client.resources.ingestion import IngestionAPIAsync, TracingConfig
from sift_client.resources.jobs import JobsAPIAsync
from sift_client.resources.ping import PingAPIAsync
from sift_client.resources.reports import ReportsAPIAsync
from sift_client.resources.rules import RulesAPIAsync
from sift_client.resources.runs import RunsAPIAsync
from sift_client.resources.tags import TagsAPIAsync
from sift_client.resources.test_results import TestResultsAPIAsync

# ruff: noqa All imports needs to be imported before sync_stubs to avoid circular import
from sift_client.resources.sync_stubs import (
    AssetsAPI,
    CalculatedChannelsAPI,
    ChannelsAPI,
    JobsAPI,
    PingAPI,
    ReportsAPI,
    RulesAPI,
    RunsAPI,
    TagsAPI,
    TestResultsAPI,
    FileAttachmentsAPI,
)

import sys

if "pytest" in sys.modules:
    # These are not test classes, so we need to set __test__ to False to avoid pytest warnings.
    # Do this here because for some reason our docs generation doesn't like it when done in the classes themselves.
    TestResultsAPI.__test__ = False  # type: ignore
    TestResultsAPIAsync.__test__ = False  # type: ignore

__all__ = [
    "AssetsAPI",
    "AssetsAPIAsync",
    "CalculatedChannelsAPI",
    "CalculatedChannelsAPIAsync",
    "ChannelsAPI",
    "ChannelsAPIAsync",
    "FileAttachmentsAPI",
    "FileAttachmentsAPIAsync",
    "IngestionAPIAsync",
    "JobsAPI",
    "JobsAPIAsync",
    "PingAPI",
    "PingAPIAsync",
    "ReportsAPI",
    "ReportsAPIAsync",
    "RulesAPI",
    "RulesAPIAsync",
    "RunsAPI",
    "RunsAPIAsync",
    "TagsAPI",
    "TagsAPIAsync",
    "TestResultsAPI",
    "TestResultsAPIAsync",
    "TracingConfig",
]
