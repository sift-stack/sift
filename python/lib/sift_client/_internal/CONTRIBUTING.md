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

New Sift types can be implemented in `sift_client/types`.

These types are used to define Pydantic models for all domain objects and to convert between protocol buffers and Python. Additional
update models can be implemented for performing updates with field masks.

All Sift types should inherit from `BaseType` and model updates from `ModelUpdate` in `sift_client/types/_base.py`

### High-Level APIs

New high-level APIs or resources are implemented in `sift_client/resources`.

These should be **purely async** implementations that interact with the Sift APi exclusively through the low-level clients.
Static and class methods should be avoided since these cannot have associated sync versions with the current implementation.

All high-level APIs should inherit from `ResourceBase` from `sift_client/resources/_base.py`.

#### Sync API Generation

To generate a sync API from an async API, add a `generate_sync_api` function call in `sift_client/resources/sync_stubs/__init__.py` and
run the `dev gen-stubs` script to generate the associated type stubs.

