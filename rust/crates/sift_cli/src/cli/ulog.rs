use std::fmt::{self, Display};

use clap::ValueEnum;
use sift_rs::data_imports::v2::UlogParseErrorPolicy as ProtoUlogParseErrorPolicy;

#[derive(Debug, Copy, Clone, ValueEnum, Default)]
pub enum UlogParseErrorPolicy {
    /// Fail the import on any recoverable parse error
    #[default]
    FailOnError,
    /// Skip records that fail to parse and continue importing
    IgnoreError,
}

impl From<UlogParseErrorPolicy> for ProtoUlogParseErrorPolicy {
    fn from(policy: UlogParseErrorPolicy) -> Self {
        match policy {
            UlogParseErrorPolicy::FailOnError => Self::FailOnError,
            UlogParseErrorPolicy::IgnoreError => Self::IgnoreError,
        }
    }
}

impl Display for UlogParseErrorPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FailOnError => write!(f, "fail-on-error"),
            Self::IgnoreError => write!(f, "ignore-error"),
        }
    }
}
