## References
../README.md
../CONTRIBUTING.md

## Language specific references
* [Python](../python/README.md)
* [Go](../go/README.md)
* [Rust](../rust/README.md)
* [c++](../cpp/README.md)


## Feature Development

The current python client library is a work in progress and is planned to be expanded to follow RFC [RFC 114_ High-level Python Client Library.md](./RFC 114_ High-level Python Client Library.md). 
This document should be used to understand the planned requirements for the python client library.

## Language Specific Guidelines

### Python

* Use Google style docstrings.
* Use Python 3.10 style type hinting. That is use `list` instead of `List` and `dict` instead of `Dict`, use `|` instead of `Union` etc.
* Use `from __future__ import annotations` at top of file to ensure backwards compatibility with Python 3.8.

#### sift_client structure

sift_client/
├── __init__.py             # Public exports
├── client.py               # Main SiftClient class
├── config.py               # Configuration defaults
├── errors.py               # Custom exceptions
├── types/                  # Public domain objects
│   ├── base.py
│   ├── asset.py
│   ├── run.py
│   └── ...
├── resources/              # High-level API implementations
│   ├── base.py
│   ├── assets.py
│   ├── runs.py
│   ├── views.py
│   ├── data.py
│   └── ...
├── transport/              # Connection handling
│   ├── base.py
│   ├── grpc_transport.py
│   └── rest_transport.py
├── _internal/              # Everything internal
│   ├── utils/              # Internal utilities
│   └── low_level_wrappers/ # Low-level API wrappers
└── tests/                  # Tests

    