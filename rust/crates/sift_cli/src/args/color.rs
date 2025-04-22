use clap::ValueEnum;
use std::fmt;

/// Controls output colorization
#[derive(Debug, Default, Clone, ValueEnum)]
pub enum Color {
    /// Disable colorization if 'NO_COLOR' is not blank or if stdout is not a TTY, otherwise enable
    #[default]
    Auto,

    /// Always colorize output
    Always,

    /// Disable output colorization
    Never,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Always => write!(f, "always"),
            Self::Never => write!(f, "never"),
        }
    }
}
