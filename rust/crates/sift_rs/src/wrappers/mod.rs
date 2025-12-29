//! Wrapper modules for Sift's gRPC services.
//!
//! These modules provide convenient wrapper traits and implementations around the
//! raw gRPC service clients generated from protobuf definitions. The wrappers
//! provide:
//! - Simplified error handling with [`sift_error::Error`]
//! - Convenient methods for common operations
//! - Access to underlying gRPC clients via `Deref` and `DerefMut`

/// Offers a wrapper over Sift's assets API.
pub mod assets;

/// Offers a wrapper over Sift's ingestion configs API.
pub mod ingestion_configs;

/// Offers a wrapper over Sift's metadata API.
pub mod metadata;

/// Offers a wrapper over Sift's runs API.
pub mod runs;

/// Used to identify resources being queried
enum ResourceIdentifier {
    Id(String),
    ClientKey(String),
}
