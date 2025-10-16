use std::fmt::{self, Display};

use clap::ValueEnum;
use sift_rs::data_imports::v2::TimeFormat as ProtoTimeFormat;

#[derive(Debug, Copy, Clone, ValueEnum, Default)]
pub enum TimeFormat {
    #[default]
    AbsoluteRfc3339,
    AbsoluteDatetime,
    AbsoluteUnixSeconds,
    AbsoluteUnixMilliseconds,
    AbsoluteUnixMicroseconds,
    AbsoluteUnixNanoseconds,
    RelativeNanoseconds,
    RelativeMicroseconds,
    RelativeMilliseconds,
    RelativeSeconds,
    RelativeMinutes,
    RelativeHours,
}

impl From<TimeFormat> for ProtoTimeFormat {
    fn from(tf: TimeFormat) -> Self {
        match tf {
            TimeFormat::RelativeNanoseconds => Self::RelativeNanoseconds,
            TimeFormat::RelativeMicroseconds => Self::RelativeMicroseconds,
            TimeFormat::RelativeMilliseconds => Self::RelativeMilliseconds,
            TimeFormat::RelativeSeconds => Self::RelativeSeconds,
            TimeFormat::RelativeMinutes => Self::RelativeMinutes,
            TimeFormat::RelativeHours => Self::RelativeHours,
            TimeFormat::AbsoluteRfc3339 => Self::AbsoluteRfc3339,
            TimeFormat::AbsoluteDatetime => Self::AbsoluteDatetime,
            TimeFormat::AbsoluteUnixSeconds => Self::AbsoluteUnixSeconds,
            TimeFormat::AbsoluteUnixMilliseconds => Self::AbsoluteUnixMilliseconds,
            TimeFormat::AbsoluteUnixMicroseconds => Self::AbsoluteUnixMicroseconds,
            TimeFormat::AbsoluteUnixNanoseconds => Self::AbsoluteUnixNanoseconds,
        }
    }
}

impl Display for TimeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AbsoluteRfc3339 => write!(f, "absolute-rfc3339"),
            Self::AbsoluteDatetime => write!(f, "absolute-datetime"),
            Self::AbsoluteUnixSeconds => write!(f, "absolute-unix-seconds"),
            Self::AbsoluteUnixMilliseconds => write!(f, "absolute-unix-milliseconds"),
            Self::AbsoluteUnixMicroseconds => write!(f, "absolute-unix-microseconds"),
            Self::AbsoluteUnixNanoseconds => write!(f, "absolute-unix-nanoseconds"),
            Self::RelativeNanoseconds => write!(f, "relative-nanoseconds"),
            Self::RelativeMicroseconds => write!(f, "relative-microseconds"),
            Self::RelativeMilliseconds => write!(f, "relative-milliseconds"),
            Self::RelativeSeconds => write!(f, "relative-seconds"),
            Self::RelativeMinutes => write!(f, "relative-minutes"),
            Self::RelativeHours => write!(f, "relative-hours"),
        }
    }
}
