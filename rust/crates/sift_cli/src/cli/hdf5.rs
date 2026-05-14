use std::fmt::{self, Display};

use clap::ValueEnum;

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum Hdf5Schema {
    OneD,
    TwoD,
    Compound,
}

impl Display for Hdf5Schema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneD => write!(f, "one-d"),
            Self::TwoD => write!(f, "two-d"),
            Self::Compound => write!(f, "compound"),
        }
    }
}
