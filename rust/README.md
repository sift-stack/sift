# Sift Rust

[![Crates.io](https://img.shields.io/crates/v/sift_rs.svg)](https://crates.io/crates/sift_rs)
[![docs](https://readthedocs.org/projects/pip/badge/)](https://docs.siftstack.com/sift_py/sift_py.html)

This library offers a Rust API on top of Sift's protocol buffers to ergonomically interface with the Sift gRPC API.

## Installation

```
$ cargo add sift_rs
```

## Examples

Various examples can be found in the [examples](https://github.com/sift-stack/sift/tree/main/rust/examples) directory. To run any of those examples clone this repo do the following:

```
$ SIFT_URI=<sift uri> SIFT_API_KEY=<api key> cargo run --example hello_sift 
```
