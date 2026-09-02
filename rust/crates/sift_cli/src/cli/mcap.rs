use std::fmt::{self, Display};

use clap::ValueEnum;
use sift_rs::data_imports::v2::{
    McapComplexTypesImportMode as ProtoMcapComplexTypesImportMode,
    McapParseErrorPolicy as ProtoMcapParseErrorPolicy,
};

#[derive(Debug, Copy, Clone, ValueEnum, Default)]
pub enum McapParseErrorPolicy {
    /// Fail the import on any recoverable parse error
    #[default]
    FailOnError,
    /// Import what decoded; skipped topics and records surface as warnings
    IgnoreError,
}

impl From<McapParseErrorPolicy> for ProtoMcapParseErrorPolicy {
    fn from(policy: McapParseErrorPolicy) -> Self {
        match policy {
            McapParseErrorPolicy::FailOnError => Self::FailOnError,
            McapParseErrorPolicy::IgnoreError => Self::IgnoreError,
        }
    }
}

impl Display for McapParseErrorPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FailOnError => write!(f, "fail-on-error"),
            Self::IgnoreError => write!(f, "ignore-error"),
        }
    }
}

/// Specifies how to handle variable-cardinality fields i.e. dynamic and
/// bounded arrays.
#[derive(Debug, Copy, Clone, ValueEnum, Default, PartialEq, Eq)]
pub enum McapComplexTypesMode {
    /// Import them as both Arrow IPC bytes and JSON strings
    #[default]
    Both,
    /// Import them as JSON strings only
    String,
    /// Import them as Arrow IPC bytes only
    Bytes,
    /// Do not import them
    Ignore,
}

impl From<McapComplexTypesMode> for ProtoMcapComplexTypesImportMode {
    fn from(mode: McapComplexTypesMode) -> Self {
        match mode {
            McapComplexTypesMode::Both => Self::Both,
            McapComplexTypesMode::String => Self::String,
            McapComplexTypesMode::Bytes => Self::Bytes,
            McapComplexTypesMode::Ignore => Self::Ignore,
        }
    }
}

impl Display for McapComplexTypesMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Both => write!(f, "both"),
            Self::String => write!(f, "string"),
            Self::Bytes => write!(f, "bytes"),
            Self::Ignore => write!(f, "ignore"),
        }
    }
}
