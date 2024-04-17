# Sift Protobufs

This repository contains protobufs (protocol buffers) for clients of Sift to interact directly with our gRPC service. Documentation for our protobufs can be found [here](https://docs.siftstack.com/ingestion/api).
If certain protobufs are not yet available for a particular API, please refer to the documentation for our [REST API](https://docs.siftstack.com/api-docs/rest) which may expose those services via gRPC gateway.

Please note that some protobufs that appear in our [protobuf documentation](https://docs.siftstack.com/ingestion/api) may not appear in this repository. Those protobufs are either actively being ported over or are
in the process of being deprecated.

For manual installation instructions for a particular programming language, click on one of the following links:
- [Go](/docs/go.md)
- [Rust](/docs/rust.md)
- [Python](/docs/python.md)

Please keep in mind that the manual installation instructions aims to be general and do not need to be strictly followed. Users are encouraged to modify any of the steps or proceed with a custom setup if it better suits the needs of their project.

For usage examples you may also refer to the [examples](examples/) directory which demonstrates basic usage of the code generated from the protobufs.

If there aren't instructions for your particular programming language consider one of the following options:
- Request for the Sift team to include instructions for your language of choice. Keep in mind that there are varying degrees of support for each language throughout the protobuf ecosystem. Depending on the language, support might be totally unavailable.
- Compile the protobufs manually.
- Use our [REST API](https://docs.siftstack.com/api-docs/rest).

In the near future we will plan to provide more installation options.
