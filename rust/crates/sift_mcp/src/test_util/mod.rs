//! Test support: an in-memory gRPC channel and `mockall` mocks of the Sift services.

/// Mock implementations of the Sift protobuf services.
pub mod mock;

/// An in-memory gRPC channel to connect a service under test to a mock server.
pub mod grpc;
