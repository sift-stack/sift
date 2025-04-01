//! `sift_connect` is a convenience crate that makes it simple to create a gRPC connection to
//! Sift. Most users won't need to install this crate as it's re-exported in other crates. The
//! following is an example of how to create a gRPC connection to Sift using sane defaults.
//!
//! ```no_run
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
//!
//! ## Config file
//!
//! Credentials can also be loaded from a `sift.toml` file stored in a the user's [local config
//! directory](https://docs.rs/dirs/6.0.0/dirs/fn.config_local_dir.html).
//!
//! The following is an example of a valid `sift.toml` file:
//!
//! ```text
//! uri = "http://example-sift-api.com"
//! apikey = "example-sift-api-key"
//!
//! [mission]
//! uri = "http://example-sift-api.com"
//! apikey = "my-other-sift-api-key"
//! ```
//!
//! The top-level TOML table is considered the default profile, with the `mission` table being a
//! named profile. To reference the default profile:
//!
//! ```no_run
//! use sift_connect::{Credentials, SiftChannelBuilder};
//! use std::env;
//!
//! let credentials = Credentials::Profile(None);
//!
//! let grpc_conn = SiftChannelBuilder::new(credentials)
//!     .build()
//!     .unwrap();
//! ```
//!
//! To reference the `mission` profile:
//!
//! ```no_run
//! use sift_connect::{Credentials, SiftChannelBuilder};
//! use std::env;
//!
//! let credentials = Credentials::Profile(Some("mission".into()));
//!
//! let grpc_conn = SiftChannelBuilder::new(credentials)
//!     .build()
//!     .unwrap();
//! ```

/// Concerned with establishing a connection to Sift's gRPC service.
pub mod grpc;
pub use grpc::{Credentials, SiftChannel, SiftChannelBuilder};

pub use sift_error::{Error, ErrorKind};
