# Sift Python Client Library
Welcome to the official Python client library for Sift! This library provides a high-level Python API on top of Sift's protocol buffers, designed to ergonomically interface with the Sift gRPC API and simplify the process of streaming data.

Sift provides official client libraries for select languages, designed to simplify the process of streaming data over gRPC. These client libraries utilize ingestion-config-based streaming to facilitate data transmission.

Check out the [repository](https://github.com/sift-stack/sift) for a list of all available client libraries.

## Installation

To install the Sift Python library:

```bash
pip install sift-stack-py
```

## API Documentation

This documentation covers two Python APIs for interacting with Sift:

### Sift Py API

The original low-level Python API that provides direct access to Sift's protocol buffer interfaces. 

Browse the [**Sift Py API**][sift_py] section for complete reference documentation.

**Use this API if you need:**

- Direct protocol buffer access
- Fine-grained control over gRPC connections  
- Legacy compatibility with existing code

### Sift Client API (New)   

!!! warning
    The Sift Client is experimental and is subject to change.


The modern, high-level client library that provides all the ergonomic features missing from the original API. This new client offers intuitive Python interfaces, strong type safety, automatic connection management, and both synchronous and asynchronous support. 

Explore the [**Sift Client API (New)**][sift_client] section for the complete API reference.

**Key improvements over Sift Py:**

- **Ergonomic Design** - Pythonic interfaces instead of raw protocol buffers
- **Type Safety** - Full type hints and Pydantic model validation
- **Dual APIs** - Both sync and async support for all operations
- **Auto Connection Management** - No manual gRPC connection handling
- **Rich Object Models** - Immutable types with convenient methods
- **Modern Patterns** - Context managers, iterators, and Python best practices


## Getting help

- **API Reference** - Browse the complete API documentation in the navigation
- **Examples** - Check out code examples throughout the documentation
- **GitHub** - Visit our [repository](https://github.com/sift-stack/sift) for issues and contributions

## What's next?

Ready to dive deeper? Explore the API documentation to learn about:

- **Sift Resources** - Creating, updating, and organizing your assets and other data
- **Data Streaming** - Efficient methods for ingesting data
- **Advanced Filtering** - Powerful query capabilities
- **Error Handling** - Best practices for robust applications
- **Performance Optimization** - Tips for high-throughput scenarios

Get started by exploring the API reference in the navigation menu!
