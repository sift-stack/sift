//! `sift_connect` is a convenience crate that makes it simple to create a gRPC connection to
//! Sift. Most users won't need to install this crate as it's re-exported in other crates. The
//! following is an example of how to create a gRPC connection to Sift using sane defaults.
//!
//! ```rust
//! use sift_connect::{Credentials, SiftChannelBuilder};
//! use std::env;
//!
//! let credentials = Credentials::Config {
//!     uri: env::var("SIFT_URI").unwrap(),
//!     apikey: env::var("SIFT_API_KEY").unwrap(),
//! };
//!
//! let grpc_conn = SiftChannelBuilder::new(credentials)
//!     .build()
//!     .unwrap();
//! ```

/// Concerned with establishing a connection to Sift's gRPC service.
pub mod grpc;
pub use grpc::*;

pub use sift_error::*;
