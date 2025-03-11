use chrono::{DateTime, Utc};
use pbjson_types::Timestamp;
use sift_error::prelude::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct TimeValue(pub(crate) Timestamp);

impl TimeValue {
    pub fn try_from_timestamp(secs: i64, nsecs: u32) -> Result<Self> {
        DateTime::<Utc>::from_timestamp(secs, nsecs)
            .map(|t| TimeValue(Timestamp::from(t)))
            .ok_or_else(|| {
                Error::new_msg(
                    ErrorKind::TimeConversionError,
                    "failed to create a UTC date-time from the provided timestamp",
                )
            })
    }

    pub fn try_from_timestamp_millis(millis: i64) -> Result<Self> {
        DateTime::<Utc>::from_timestamp_millis(millis)
            .map(|t| TimeValue(Timestamp::from(t)))
            .ok_or_else(|| {
                Error::new_msg(
                    ErrorKind::TimeConversionError,
                    "failed to create a UTC date-time from the provided timestamp-milliseconds",
                )
            })
    }

    pub fn try_from_timestamp_micros(micros: i64) -> Result<Self> {
        DateTime::<Utc>::from_timestamp_micros(micros)
            .map(|t| TimeValue(Timestamp::from(t)))
            .ok_or_else(|| {
                Error::new_msg(
                    ErrorKind::TimeConversionError,
                    "failed to create a UTC date-time from the provided timestamp-microseconds",
                )
            })
    }

    pub fn from_timestamp_nanos(nanos: i64) -> Self {
        TimeValue(Timestamp::from(DateTime::<Utc>::from_timestamp_nanos(
            nanos,
        )))
    }

    pub fn try_from_rfc3339<S: AsRef<str>>(val: S) -> Result<Self> {
        DateTime::parse_from_rfc3339(val.as_ref())
            .map(|d| TimeValue(Timestamp::from(d.to_utc())))
            .map_err(|e| Error::new(ErrorKind::TimeConversionError, e))
            .context("encountered invalid RFC3339 datetime string")
    }
}

impl From<DateTime<Utc>> for TimeValue {
    fn from(value: DateTime<Utc>) -> Self {
        Self(Timestamp::from(value))
    }
}

impl Deref for TimeValue {
    type Target = Timestamp;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TimeValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
