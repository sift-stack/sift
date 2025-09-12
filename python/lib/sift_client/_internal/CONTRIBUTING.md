# Contributing

## Style Guidelines

### Docstrings

Google style docstrings: https://google.github.io/styleguide/pyguide.html

### Type Hinting

Use Python 3.10+ style type hinting, but maintain backwards compatibility with Python 3.8. This
is possible by using `eval_type_backport` which converts 3.10+ type hints to 3.8+ type hints by
overriding the behavior of `typine._val_type`.

* Always use `from __future__ import annotations` at the top of the file to help with Python 3.8 backwards
  compatibility.
* Use 3.10+ type hints style for built-in types. That is use `list` instead of `List` and `dict` instead of `Dict`, use
  `|` instead of `Union` etc.
* Instead of `Optional[str]` use `str | None`. PEP 484 now discourages implicit `None` in type hints.

### Keyword Only Arguments

To improve continued compatibility with user-code, user-facing methods that use optional arguments should use
keyword-only arguments. This allows us to evolve method signatures without breaking backwards compatibility.

This can be done by adding a `*` to the argument list, e.g.:

```python
def foo(self, *, b=None):
    pass
```

Users will then be required to do `obj.foo(b=1)` instead of `obj.foo(1)`.

## Implementing New Resources (High-Level API)

See `sift_client/_internal/README.md` for more details.

### Low-Level Client
If required, implement a low-level client for any new gRPC services in `sift_client/_internal/low_level_wrappers`.

These should be **purely async** implementations that wrap gRPC stubs. Rather than implementing logic, these should focus on mapping
between protocol buffers and Python objects and vice versa.

All low-level clients should implement `LowLevelClientBase` from `sift_client/_internal/low_level_wrappers/base.py`.

### Sift Types

New Sift types can be implemented in `sift_client/sift_types`.

These types are used to define Pydantic models for all domain objects and to convert between protocol buffers and Python. Additional
update and create models can be implemented for performing updates with field masks.

All Sift types should inherit from `BaseType` in `sift_client/sift_types/_base.py`

#### Create/Update Pydantic Model Inheritance Pattern

The Sift client uses a composition-based inheritance pattern for Pydantic models to avoid complex multiple inheritance issues:

1. **Base Classes** (`sift_client/sift_types/_base.py`):
   - `ModelCreateUpdateBase`: Base class containing shared functionality for proto conversion and field mapping
   - `ModelCreate`: Inherits from `ModelCreateUpdateBase` with generic typing for creation operations
   - `ModelUpdate`: Inherits from `ModelCreateUpdateBase` with additional field mask support for updates

2. **Domain-Specific Base Classes**:
   Create a base class that inherits from `ModelCreateUpdateBase` and contains:
   - All shared field definitions
   - Shared `_to_proto_helpers` configuration for complex proto mappings
   - Common validation logic using `@model_validator`
   It may not always make sense to implement a base class if there is little/no overlap in fields or protos.

3. **Create and Update Models**:
   - `{Domain}Create`: Inherits from both `{Domain}Base` and `ModelCreate[{CreateProto}]`
     - Include create only fields and validators
   - `{Domain}Update`: Inherits from both `{Domain}Base` and `ModelUpdate[{UpdateProto}]`
     - Include update only fields and validators

#### Proto Mapping Helpers

Use `MappingHelper` for complex proto field mappings when the Pydantic model doesn't match the proto model exactly:
- `proto_attr_path`: Dot-separated path to the proto field
- `update_field`: Field name for update masks (optional)
- `converter`: Function/class to convert the value (optional)

#### Validation Guidelines

- Use `@model_validator(mode="after")` for cross-field validation
- Prefix validation method names with `_` (e.g., `_validate_time_fields`) since these don't need to be user visible
- Keep validation logic in the base class when shared between create/update
- Add specific validation in create/update classes as needed

### High-Level APIs

New high-level APIs or resources are implemented in `sift_client/resources`.

These should be **purely async** implementations that interact with the Sift APi exclusively through the low-level clients.
Static and class methods should be avoided since these cannot have associated sync versions with the current implementation.

All high-level APIs should inherit from `ResourceBase` from `sift_client/resources/_base.py`.

#### Resource Method Patterns

Resource classes should implement consistent patterns for common operations. Use the helper methods from `ResourceBase` to build standard filter arguments.

**Important:** Arguments that represent another Sift Type should always accept both the object instance and its ID string. This provides flexibility for users who may have either form.

Examples:
```python
# Accept either Asset object or asset ID string
async def update(self, asset: Asset | str, ...) -> Asset:
```

##### Standard Method Signatures

**`get(resource_id: str) -> {Type}`**
- Single required positional argument for the resource ID
- Returns the specific resource instance

**`list_(...) -> list[{Type}]`**
- Use `list_` (with underscore) to avoid conflicts with Python's built-in `list`
- Standard filter arguments in consistent order (as applicable:)
  1. Name filters: `name`, `name_contains`, `name_regex`
  2. Self IDs: Resource-specific ID filters (e.g., `run_ids`, `asset_ids`, `client_keys`)
  3. Created/modified ranges: `created_after`, `created_before`, `modified_after`, `modified_before`
  4. Created/modified users: `created_by`, `modified_by`
  5. Metadata: `metadata`, `tags`
  6. Resource-specific filters: Domain-specific filters (e.g., `assets`, `duration_less_than`, `start_time_after`)
  7. Common filters: `description_contains`, `include_archived`, `filter_query`
  8. Ordering and pagination: `order_by`, `limit`, `page_size`, `page_token`

**`find(...) -> {Type} | None`**
- Similar signature to `list_` but returns single result or None
- Should use the same filter arguments as `list_`

**`create(create: {Type}Create | dict, **kwargs) -> {Type}`**
- Accept both Pydantic model and dict
- Additional keyword arguments for operation-specific options

**`update({resource}: str | {Type}, update: {Type}Update | dict, **kwargs) -> {Type}`**
- First argument accepts either ID string or resource instance
- Update model as second argument
- Additional keyword arguments for operation-specific options

##### Using ResourceBase Helper Methods

The `ResourceBase` class provides helper methods to build consistent CEL filter expressions:

```python
def list_(
    self,
    *,
    asset_ids: list[str] | None = None,
    name: str | None = None,
    name_contains: str | None = None,
    name_regex: str | re.Pattern | None = None,
    created_after: datetime | None = None,
    created_before: datetime | None = None,
    modified_after: datetime | None = None,
    modified_before: datetime | None = None,
    created_by: str | None = None,
    modified_by: str | None = None,
    description_contains: str | None = None,
    tags: list[str] | None = None,
    metadata: list[Any] | None = None,
    include_archived: bool = False,
    filter_query: str | None = None,
    page_size: int | None = None,
    page_token: str | None = None,
) -> list[Asset]:
    filter_parts = [
        *self._build_name_cel_filters(
            name=name,
            name_contains=name_contains,
            name_regex=name_regex,
        ),
        *self._build_time_cel_filters(
            created_after=created_after,
            created_before=created_before,
            modified_after=modified_after,
            modified_before=modified_before,
            created_by=created_by,
            modified_by=modified_by,
        ),
        *self._build_tags_metadata_cel_filters(tags=tags, metadata=metadata),
        *self._build_common_cel_filters(
            description_contains=description_contains,
            include_archived=include_archived,
            filter_query=filter_query,
        ),
    ]
    if asset_ids:
        filter_parts.append(cel.in_("asset_id", asset_ids))
    
    # Build filter and call low-level client
    filter_expr = cel.and_(*filter_parts) if filter_parts else None
    # ... rest of implementation
```

##### Available Helper Methods

- `_build_name_cel_filters()`: Handles `name`, `name_contains`, `name_regex`
- `_build_time_cel_filters()`: Handles time-based filters and user filters
- `_build_tags_metadata_cel_filters()`: Handles `tags` and `metadata` filters
- `_build_common_cel_filters()`: Handles `description_contains`, `include_archived`, `filter_query`


#### Sync API Generation

To generate a sync API from an async API, add a `generate_sync_api` function call in `sift_client/resources/sync_stubs/__init__.py` and
run the `dev gen-stubs` script to generate the associated type stubs.

