use sift_connect::SiftChannel;

pub mod builder;

pub mod channel;

/// Implementations for streaming modes.
pub mod mode;

pub mod time;

pub struct SiftStream<M: SiftStreamMode> {
    grpc_channel: SiftChannel,
    mode: M,
}

pub trait SiftStreamMode {}
