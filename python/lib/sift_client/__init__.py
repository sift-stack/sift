"""Sift Client Library - Python client for interacting with Sift APIs.

## Overview

This library provides a high-level Python client for interacting with Sift APIs. It offers:

- **Synchronous and asynchronous interfaces** for all operations
- **Strong type checking** with Pydantic models
- **Pythonic API design** with intuitive method names
- **Comprehensive filtering** capabilities for queries
- **Automatic type conversion** between protobuf and Python types

## Installation

```bash
pip install sift-stack-py
```

## Quick Start

### Initialize the Client

```python
from sift_client import SiftClient

# Initialize with credentials
client = SiftClient(
    api_key="your-api-key",
    grpc_url="https://grpc-api.siftstack.com",
    rest_url="https://api.siftstack.com"
)
```

### Basic Operations

```python
# Get an asset
asset = client.assets.get(asset_id="asset123")

# List resources with filtering
runs = client.runs.list_(
    assets=[asset.id_],
    start_time_after=datetime.now() - timedelta(days=7),
    limit=10
)

# Update a resource
asset.update({"tags": ["production", "v2"]})

# Create a new resource
run = client.runs.create({
    "name": "Test Run",
    "asset_ids": [asset.id_],
    "start_time": datetime.now()
})
```

### Async Usage

```python
import asyncio

async def main():
    # Use async_ accessor for async operations
    asset = await client.async_.assets.get(asset_id="asset123")
    runs = await client.async_.runs.list_(limit=10)
    return asset, runs

result = asyncio.run(main())
```

## Key Components

### Resources

Resource APIs provide methods for interacting with Sift services. Each resource supports
operations like `get()`, `list_()`, `create()`, `update()`, and `archive()`.

**Available Resources:**

- `client.assets` - Manage physical or logical entities
- `client.runs` - Manage time-bounded operational periods
- etc.

See [resources](resources/) for detailed documentation and a complete list.

### Types

Sift types are immutable Pydantic models representing Sift objects. They provide
type-safe access to properties and convenience methods for common operations.

**Available Types:**

- `Asset`, `AssetUpdate` - Asset resources
- `Run`, `RunCreate`, `RunUpdate` - Run resources
- etc.

See [sift_types](sift_types/) for detailed documentation and a complete list.

## Examples

For complete examples, see the [examples](../../examples/) directory.

## Connection Configuration

For advanced connection options:

```python
from sift_client.transport import SiftConnectionConfig, GrpcConfig, RestConfig

config = SiftConnectionConfig(
    grpc_config=GrpcConfig(
        uri="https://grpc-api.siftstack.com",
        api_key="your-api-key",
        use_ssl=True
    ),
    rest_config=RestConfig(
        uri="https://api.siftstack.com",
        api_key="your-api-key"
    )
)

client = SiftClient(connection_config=config)
```

## Best Practices

1. **Use sync APIs** for notebooks, scripts, and simple applications
2. **Use async APIs** for high-performance services with concurrent operations
3. **Leverage filtering** to reduce data transfer and improve performance
4. **Reuse client instances** rather than creating new ones for each operation
5. **Use type hints** to get full IDE support and catch errors early
"""

import logging

from sift_client.client import SiftClient
from sift_client.transport import SiftConnectionConfig

__all__ = [
    "SiftClient",
    "SiftConnectionConfig",
]

logging.getLogger(__name__).addHandler(logging.NullHandler())
