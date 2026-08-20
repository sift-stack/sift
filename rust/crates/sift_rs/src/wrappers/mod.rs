//! Wrapper modules for Sift's gRPC services.
//!
//! These modules provide convenient wrapper traits and implementations around the
//! raw gRPC service clients generated from protobuf definitions. The wrappers
//! provide:
//! - Simplified error handling with [`sift_error::Error`]
//! - Convenient methods for common operations
//! - Access to underlying gRPC clients via `Deref` and `DerefMut`

pub use sift_connect::{MAX_DECODING_MESSAGE_SIZE, ServiceOptions};

/// Applies [`ServiceOptions`] to a generated tonic client. A macro rather than a generic fn
/// because tonic exposes these as inherent methods, not a trait.
macro_rules! configured_client {
    ($client:ident, $channel:expr, $options:expr) => {
        $client::new($channel).max_decoding_message_size($options.max_decoding_message_size)
    };
}

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
