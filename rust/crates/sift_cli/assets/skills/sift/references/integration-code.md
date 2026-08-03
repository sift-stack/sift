# Writing integration code against Sift

When a user wants help integrating their own systems with Sift, consult the
references below and write code against `sift_client` for Python or
`sift_stream` for Rust. Use the examples in those docs as the starting point
rather than inventing API shapes. When writing code to interact with Sift,
infer based on the user's environment which language is preferred. If both
Python and Rust exist, ask the user which language to author code in.

Look up any detail you are unsure of with `search_docs` before you write the
call. It serves the full REST and gRPC API reference.

## References

- **REST API** — the full API surface: https://docs.siftstack.com/api/rest
- **Python library** — module `sift_client`:
  https://sift-stack.github.io/sift/python/latest/reference/sift_client/
  The older `sift_py` module is deprecated. Reach for it only as a last resort
  when `sift_client` lacks a needed capability.
- **Python ingestion examples**:
  https://sift-stack.github.io/sift/python/latest/examples/ingestion/
- **Rust streaming library** — `sift_stream`, for high-throughput streaming
  ingestion: https://docs.rs/sift_stream/latest/sift_stream/
