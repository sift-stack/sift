# sift_stream

[![Crates.io](https://img.shields.io/crates/v/sift_stream.svg)](https://crates.io/crates/sift_stream)
[![docs.rs](https://img.shields.io/docsrs/sift_stream)](https://docs.rs/sift_stream/latest/sift_stream/)

The `sift_stream` crate is primarily focused on streaming telemetry to Sift in a robust manner.

Here are some features highlights:
- Builtin retries with default or custom retry policies in the case of a Sift outage or a
  client-side network outage.
- Periodic checkpointing to confirm that all data within a particular period has been received
  by Sift.
- Optional automated backups to mitigate data-loss in the case of misc. outages.
- Optional tracing/logging to monitor the health of your stream and view various ingestion
  performance metrics.

See the [examples](./examples/) directory for demonstrations on how to stream data to Sift using
`sift_stream`.
