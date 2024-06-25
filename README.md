# Sift

[![Build status](https://github.com/sift-stack/sift/actions/workflows/rust_ci.yaml/badge.svg)](https://github.com/sift-stack/sift/actions)
[![Build status](https://github.com/sift-stack/sift/actions/workflows/python_ci.yaml/badge.svg)](https://github.com/sift-stack/sift/actions)
[![Build status](https://github.com/sift-stack/sift/actions/workflows/go_ci.yaml/badge.svg)](https://github.com/sift-stack/sift/actions)
[![Build status](https://github.com/sift-stack/sift/actions/workflows/protos_ci.yaml/badge.svg)](https://github.com/sift-stack/sift/actions)
[![pypi](https://img.shields.io/pypi/pyversions/sift-stack-py)](https://pypi.org/project/sift-stack-py/)

This repository contains utilities to interface with Sift's API. Currently, the primary way to interact with our API is through the code generated via our protobufs (protocol buffers). Comprehensive documentation
for our protobufs can be found at [this link](https://docs.siftstack.com/ingestion/api) or in the actual proto files themselves as doc-comments.
If certain protobufs are not yet available for a particular API, please refer to the documentation for our [REST API](https://docs.siftstack.com/api-docs/rest) which may expose those services via gRPC gateway.

Please note that some protobufs that appear in our [protobuf documentation](https://docs.siftstack.com/ingestion/api) may not appear in this repository. Those protobufs are either actively being ported over or are
in the process of being deprecated.

## Table of Contents

* [Installation](#installation)
  - [Installation via Package Managers](#installation-via-package-managers)
      - [Go](#go)
      - [Rust](#rust)
      - [Python](#python)
  - [Manual Installation](#manual-installation)
* [Examples](#examples)

## Installation

To install Sift's client utilities for the languages we currently support, you can either use your language's package manager or proceed with a manual installation process if you have particular requirements for
how the protobufs should be generated.

### Installation via Package Managers

The following demonstrates how to install Sift's client utilities for each supported language using their respective official package managers. The source is currently hosted only on Github, but in the future we may move
these to the official package repositories for each language.

#### Go

```
$ go get github.com/sift-stack/sift/go@main && go mod tidy
```

#### Rust

```
$ cargo add --git https://github.com/sift-stack/sift sift
```

#### Python

```
$ pip install 'git+https://github.com/sift-stack/sift.git#subdirectory=python'
```

### Manual Installation

For manual installation instructions for a particular supported programming language, click on one of the following links:
- [Go](/docs/go.md)
- [Rust](/docs/rust.md)
- [Python](/docs/python.md)

Please keep in mind that the manual installation instructions aims to be general and do not need to be strictly followed. Users are encouraged to modify any of the steps or proceed with a custom setup if it better suits the needs of their project.

If there aren't instructions for your particular programming language consider one of the following options:
- Request for the Sift team to include instructions for your language of choice. Keep in mind that there are varying degrees of support for each language throughout the protobuf ecosystem. Depending on the language, support might be totally unavailable.
- Compile the protobufs manually.
- Use our [REST API](https://docs.siftstack.com/api-docs/rest).

In the near future we will plan to provide more installation options.

## Examples

For usage examples you may also refer to the [examples](examples/) directory which demonstrates basic usage of the code generated from the protobufs.
