use std::fmt::{self, Display};

use clap::ValueEnum;
use sift_rs::data_imports::v2::TdmsFallbackMethod as ProtoTdmsFallbackMethod;

#[derive(Debug, Copy, Clone, ValueEnum, Default)]
pub enum TdmsFallbackMethod {
    #[default]
    FailOnError,
    IgnoreError,
}

impl From<TdmsFallbackMethod> for ProtoTdmsFallbackMethod {
    fn from(fm: TdmsFallbackMethod) -> Self {
        match fm {
            TdmsFallbackMethod::FailOnError => Self::FailOnError,
            TdmsFallbackMethod::IgnoreError => Self::IgnoreError,
        }
    }
}

impl Display for TdmsFallbackMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FailOnError => write!(f, "fail-on-error"),
            Self::IgnoreError => write!(f, "ignore-error"),
        }
    }
}
