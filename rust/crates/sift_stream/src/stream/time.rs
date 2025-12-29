use chrono::{DateTime, Local, Utc};
use pbjson_types::Timestamp;
use sift_error::prelude::*;
use std::ops::{Deref, DerefMut};

/// The primary time-type of the `sift_stream` crate.
///
/// This is a flexible wrapper over the underlying protobuf time-type that can be
/// constructed from a variety of different time representations. All times are
/// stored and transmitted as UTC.
///
/// # Example
///
/// ```
/// use sift_stream::TimeValue;
///
/// // Current time
/// let now = TimeValue::now();
///
/// // From timestamp
/// let time = TimeValue::try_from_timestamp_millis(1609459200000).unwrap();
///
/// // From RFC3339 string
/// let time = TimeValue::try_from_rfc3339("2021-01-01T00:00:00Z").unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct TimeValue(pub(crate) Timestamp);

/// Initializes with the current time.
impl Default for TimeValue {
    /// Creates a time-value that represents the current absolute time in UTC.
    fn default() -> Self {
        Self::from(Local::now().to_utc())
    }
}

impl TimeValue {
    /// Creates a time-value that represents the current absolute time in UTC.
    ///
    /// # Returns
    ///
    /// A `TimeValue` representing the current time.
    ///
    /// # Example
    ///
    /// ```
    /// use sift_stream::TimeValue;
    ///
    /// let now = TimeValue::now();
    /// ```
    pub fn now() -> Self {
        Self::default()
    }

    /// Creates a [`TimeValue`] from a second and nanosecond timestamp.
    ///
    /// # Arguments
    ///
    /// * `secs` - Seconds since Unix epoch (can be negative)
    /// * `nsecs` - Nanoseconds component (0-999,999,999)
    ///
    /// # Returns
    ///
    /// A `TimeValue` if the timestamp is valid, or an error if the timestamp
    /// is out of range.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::TimeConversionError`] if the timestamp is invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use sift_stream::TimeValue;
    ///
    /// let time = TimeValue::try_from_timestamp(1609459200, 0).unwrap();
    /// ```
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

    /// Creates a [`TimeValue`] from a millisecond timestamp.
    ///
    /// # Arguments
    ///
    /// * `millis` - Milliseconds since Unix epoch (can be negative)
    ///
    /// # Returns
    ///
    /// A `TimeValue` if the timestamp is valid, or an error if the timestamp
    /// is out of range.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::TimeConversionError`] if the timestamp is invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use sift_stream::TimeValue;
    ///
    /// let time = TimeValue::try_from_timestamp_millis(1609459200000).unwrap();
    /// ```
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

    /// Creates a [`TimeValue`] from a microsecond timestamp.
    ///
    /// # Arguments
    ///
    /// * `micros` - Microseconds since Unix epoch (can be negative)
    ///
    /// # Returns
    ///
    /// A `TimeValue` if the timestamp is valid, or an error if the timestamp
    /// is out of range.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::TimeConversionError`] if the timestamp is invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use sift_stream::TimeValue;
    ///
    /// let time = TimeValue::try_from_timestamp_micros(1609459200000000).unwrap();
    /// ```
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

    /// Creates a [`TimeValue`] from a nanosecond timestamp.
    ///
    /// Unlike the other timestamp constructors, this method does not return a `Result`
    /// because nanosecond timestamps are always valid (they cover the full range of
    /// representable times).
    ///
    /// # Arguments
    ///
    /// * `nanos` - Nanoseconds since Unix epoch (can be negative)
    ///
    /// # Returns
    ///
    /// A `TimeValue` representing the given timestamp.
    ///
    /// # Example
    ///
    /// ```
    /// use sift_stream::TimeValue;
    ///
    /// let time = TimeValue::from_timestamp_nanos(1609459200000000000);
    /// ```
    pub fn from_timestamp_nanos(nanos: i64) -> Self {
        TimeValue(Timestamp::from(DateTime::<Utc>::from_timestamp_nanos(
            nanos,
        )))
    }

    /// Creates a [`TimeValue`] from an RFC3339 datetime string.
    ///
    /// # Arguments
    ///
    /// * `val` - An RFC3339 formatted datetime string (e.g., "2021-01-01T00:00:00Z")
    ///
    /// # Returns
    ///
    /// A `TimeValue` if the string is valid, or an error if parsing fails.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::TimeConversionError`] if the string is not valid RFC3339.
    ///
    /// # Example
    ///
    /// ```
    /// use sift_stream::TimeValue;
    ///
    /// let time = TimeValue::try_from_rfc3339("2021-01-01T00:00:00Z").unwrap();
    /// ```
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
