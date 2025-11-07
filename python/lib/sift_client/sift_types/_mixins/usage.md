# AttachmentsMixin Usage

The `AttachmentsMixin` provides a standardized way to add file attachment functionality to Sift types.

## Features

The mixin adds two methods to any class that uses it:

1. **`async get_attachments()`** - Async method to fetch attachments
2. **`attachments`** - Sync property that fetches attachments using the client's event loop

## Usage

### For Sift Type Models

The mixin automatically determines the entity type based on the class name. Simply inherit from `AttachmentsMixin`:

```python
from sift_client.sift_types._mixins.file_attachments import AttachmentsMixin


class Asset(BaseType[AssetProto, "Asset"], AttachmentsMixin):
    """Asset model - automatically mapped to ENTITY_TYPE_ASSET."""

    # ... rest of the model fields
```

### Adding New Models

To add attachment support to a new model:

1. Inherit from `AttachmentsMixin`
2. Add the class name mapping to `AttachmentsMixin._ENTITY_TYPE_MAP` in `_mixins.py`

```python
# In _mixins.py
_ENTITY_TYPE_MAP: ClassVar[dict[str, str]] = {
    "Asset": "ENTITY_TYPE_ASSET",
    "Run": "ENTITY_TYPE_RUN",
    "TestReport": "ENTITY_TYPE_TEST_REPORT",
    "MyNewModel": "ENTITY_TYPE_MY_NEW_MODEL",  # Add your model here
}
```

### Examples

#### Async Usage

```python
# In an async context
asset = await client.assets.get(asset_id="...")
attachments = await asset.get_attachments()
for file in attachments:
    print(f"File: {file.file_name}")
```

#### Sync Usage (Property)

```python
# In a sync context
asset = client.assets.get(asset_id="...")
attachments = asset.attachments  # Uses the property
for file in attachments:
    print(f"File: {file.file_name}")
```

## Current Implementations

The following Sift types currently use `AttachmentsMixin`:

- **Asset** - `ENTITY_TYPE_ASSET`
- **Run** - `ENTITY_TYPE_RUN`
- **TestReport** - `ENTITY_TYPE_TEST_REPORT`

## How It Works

1. The mixin maintains a `_ENTITY_TYPE_MAP` dictionary mapping class names to entity types
2. When `get_attachments()` is called, it looks up the class name in the map
3. The async method uses the low-level `RemoteFilesLowLevelClient` to query for files
4. It builds a CEL filter based on the entity ID and the mapped entity type
5. The sync property `attachments` uses `asyncio.run_coroutine_threadsafe()` to run the async method on the client's dedicated event loop
6. This ensures all async operations happen in the same event loop context (see architecture notes)

## Architecture Notes

The mixin follows the established pattern where:
- Low-level clients are purely async
- High-level APIs provide both sync and async versions
- Sync methods use `asyncio.run_coroutine_threadsafe()` with the client's dedicated event loop
- This prevents "Task got Future attached to a different loop" errors
