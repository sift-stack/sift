# Sift

[![Build status](https://github.com/sift-stack/sift/actions/workflows/rust_ci.yaml/badge.svg)](https://github.com/sift-stack/sift/actions)
[![Build status](https://github.com/sift-stack/sift/actions/workflows/python_ci.yaml/badge.svg)](https://github.com/sift-stack/sift/actions)
[![Build status](https://github.com/sift-stack/sift/actions/workflows/go_ci.yaml/badge.svg)](https://github.com/sift-stack/sift/actions)
[![Build status](https://github.com/sift-stack/sift/actions/workflows/protos_ci.yaml/badge.svg)](https://github.com/sift-stack/sift/actions)
[![pypi](https://img.shields.io/pypi/v/sift-stack-py)](https://pypi.org/project/sift-stack-py/)
[![Crates.io](https://img.shields.io/crates/v/sift_rs.svg)](https://crates.io/crates/sift_rs)
[![Crates.io](https://img.shields.io/crates/v/sift_stream.svg)](https://crates.io/crates/sift_stream)
[![PkgGoDev](https://pkg.go.dev/badge/mod/github.com/sif-stack/sift/go)](https://pkg.go.dev/github.com/sift-stack/sift/go) 

This repository contains client libraries and protocol buffers to interact with Sift's API in various languages. Each client library contains pre-compiled protocol buffers, but should you wish
to compile the protocol buffers yourself there are instructions on how to go about it in the [Manual Protobuf Compilation](#manual-protobuf-compilation) section.

## Table of Contents

* [Installation](#installation)
  - [Installation via Package Managers](#installation-via-package-managers)
      - [Go](#go)
      - [Rust](#rust)
      - [Python](#python)
  - [Manual Protobuf Compilation](#manual-protobuf-compilation)
* [Examples](#examples)

## Installation

The Sift client library can be installed using the package managers for the languages that are currently supported. If you are using a language that isn't officially supported see
the [Manual Protobuf Compilation](#manual-protobuf-compilation) section.

### Installation via Package Managers

The following demonstrates how to install the Sift client library for each supported language.

#### Python

```
$ pip install sift-stack-py
```

#### Rust

```
$ cargo add sift_rs sift_stream
```

#### Go

```
$ go get github.com/sift-stack/sift/go
```

### Manual Protobuf Compilation

For manual installation instructions for a particular supported programming language, click on one of the following links:
- [Go](/docs/go.md)
- [Rust](/docs/rust.md)
- [Python](/docs/python.md)
- [C++](/cpp/README.md)

Please keep in mind that the manual installation instructions aims to be general and do not need to be strictly followed. Users are encouraged to modify any of the steps or proceed with a custom setup if it better suits the needs of their project.
