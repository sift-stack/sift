use std::fmt::{self, Display};

use clap::ValueEnum;
use sift_rs::data_imports::v2::ParquetComplexTypesImportMode;

/// Specifies how to handle columns that are complex types i.e. maps, lists and structs.
#[derive(Debug, Clone, ValueEnum, Default)]
pub enum ComplexTypesMode {
    /// Ignore columns containing complex types
    #[default]
    Ignore,
    /// Import complex types as both Arrow bytes and JSON strings.
    Both,
    /// Import complex types as JSON strings
    String,
    /// Import complex types as Arrow bytes.
    Bytes,
}

impl From<ComplexTypesMode> for ParquetComplexTypesImportMode {
    fn from(mode: ComplexTypesMode) -> Self {
        match mode {
            ComplexTypesMode::Both => Self::Both,
            ComplexTypesMode::Bytes => Self::Bytes,
            ComplexTypesMode::Ignore => Self::Ignore,
            ComplexTypesMode::String => Self::String,
        }
    }
}

impl Display for ComplexTypesMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Both => write!(f, "both"),
            Self::Bytes => write!(f, "bytes"),
            Self::Ignore => write!(f, "ignore"),
            Self::String => write!(f, "string"),
        }
    }
}

/// Channel-per-row mode: tells the importer how each row is shaped.
#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum CprMode {
    /// File has [time, value]. All rows belong to one named channel.
    Single,
    /// File has [time, name_column, value_column]. Channels created per unique name.
    Multi,
}

impl Display for CprMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Single => write!(f, "single"),
            Self::Multi => write!(f, "multi"),
        }
    }
}
