pub mod stream;
pub use stream::{
    builder::{IngestionConfigSelector, RunSelector, SiftStreamBuilder},
    channel::{ChannelValue, Value},
    mode::ingestion_config::{Flow, IngestionConfigMode},
    time::TimeValue,
    RetryPolicy, SiftStream,
};

pub use sift_connect::*;
pub use sift_error::*;
pub use sift_rs::*;

pub(crate) mod backup;

/// # Internal Notes
///
/// [prost::Message] has methods to encode/decode protobuf messages with their length delimiters
/// but we need to be able to read the wire-format for multiple protobufs from a file and thus
/// require garauntees for how the length-delimiter is handled. The only garauntee we can get is if
/// we handle it ourselves.
pub(crate) mod pbutil;
