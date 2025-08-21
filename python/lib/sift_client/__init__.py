"""
!!! warning
    The Sift Client is experimental and is subject to change.


# Sift Client Library

This library provides a high-level Python client for interacting with Sift APIs. It offers both synchronous and
asynchronous interfaces, strong type checking, and a Pythonic API design.

## Installation

```bash
pip install sift-stack-py
```

## Getting Started

### Initializing the Client

You can initialize the Sift client with your API key and service URLs:

```python
from sift_client import SiftClient
from datetime import datetime

# Initialize with individual parameters
client = SiftClient(
    api_key="your-api-key",
    grpc_url="your-sift-grpc-url",
    rest_url="your-sift-rest-url"
)

# Or use a connection configuration
from sift_client.transport import SiftConnectionConfig

config = SiftConnectionConfig(
    api_key="your-api-key",
    grpc_url="your-sift-grpc-url",
    rest_url="your-sift-rest-url"
)
client = SiftClient(connection_config=config)
```

The `SiftConnectionConfig` provides access to additional configuration options such as `use_ssl` and `cert_via_openssl`.

### Using Synchronous and Asynchronous APIs

The Sift client provides both synchronous and asynchronous versions of all APIs. You can choose the one that best fits
your application's needs.

#### Synchronous API

The synchronous API is perfect for scripts, notebooks, and applications that don't need asynchronous operation:

```python
# Get an asset by ID
asset = client.assets.get(asset_id="asset123")

# List assets with filtering
assets = client.assets.list_(
    name_contains="example",
    created_after=datetime(2023, 1, 1),
    include_archived=False
)

# Find a single asset matching criteria
asset = client.assets.find(name="my-asset")
```

#### Asynchronous API

The asynchronous API is ideal for high-performance applications and services that need to make concurrent API calls:

```python
import asyncio


async def get_asset_async():
    # Get an asset by ID asynchronously
    asset = await client.assets_async.get(asset_id="asset123")

    # Running Sync within async also works
    some_other_asset = client.assets.get(asset_id="asset456")

    return asset


# Run in an async context
asset = asyncio.run(get_asset_async())

```

### Working with Sift Types

Sift types (like `Asset`, `Run`, etc.) are immutable Pydantic models that provide a convenient interface for working
with Sift resources.

#### Accessing Properties

```python
# Get an asset
asset = client.assets.get(asset_id="asset123")

# Access properties
print(f"Asset name: {asset.name}")
print(f"Created on: {asset.created_date}")
print(f"Tags: {', '.join(asset.tags)}")
print(f"Is archived: {asset.is_archived}")
```

#### Using Methods on Sift Types

Sift types have convenient methods for common operations. These methods use the synchronous API internally.
**Using these methods will update the instance in-place.**

```python
# Get an asset
asset = client.assets.get(asset_id="asset123")

# Archive the asset
asset.archive(archive_runs=True)

# Update the asset
asset.update({
    "tags": ["updated", "example"]
})
```

> **Note:** Type methods only work with the synchronous API. If you need to use the asynchronous API, you should use the
> resource APIs directly.

#### Creating Update Models

For more complex updates, you can create update models (instead of a key-value dictionary):

```python
from sift_client.types.asset import AssetUpdate

# Create an update model
update = AssetUpdate(tags=["new", "tags"])

# Apply the update
asset = client.assets.update(asset="asset123", update=update)

# Or using the asset method
asset = client.assets.get(asset_id="asset123").update(update)
```

## Advanced Usage

### Working with Tags

Tags are a powerful way to organize and filter your assets:

```python
# Add tags when updating an asset
asset.update({
    "tags": ["production", "model-v1", "trained"]
})

# Filter assets by tags
production_assets = client.assets.list_(
    tags=["production"]
)
```

### Filtering Assets

The client provides various ways to filter different Sift types:

```python
# Filter by name (exact match)
assets = client.assets.list_(name="my-model")

# Filter by name (contains)
assets = client.assets.list_(name_contains="model")

# Filter by name (regex)
assets = client.assets.list_(name_regex="model-v[0-9]+")

# Filter by creation date
assets = client.assets.list_(
    created_after=datetime(2023, 1, 1),
    created_before=datetime(2023, 12, 31)
)

# Filter by modification date
assets = client.assets.list_(
    modified_after=datetime(2023, 6, 1)
)

# Include archived assets
all_assets = client.assets.list_(include_archived=True)

# Limit the number of results
recent_assets = client.assets.list_(
    limit=10,
    order_by="modified_date desc"
)
```


"""

from sift_client.client import SiftClient
from sift_client.transport import SiftConnectionConfig

__all__ = [
    "SiftClient",
    "SiftConnectionConfig",
]
