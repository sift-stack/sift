use super::prelude::*;
use indoc::indoc;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::time::Duration;

#[test]
pub fn test_error_formatting() {
    let inner_error = IoError::new(IoErrorKind::NotFound, "I am the root cause");
    let sift_error: Result<()> = Err(Error::new(ErrorKind::IoError, inner_error));
    let actual = format!("{}", sift_error.err().unwrap());
    let expected = indoc! {"
        [IoError]

        [cause]:
           - I am the root cause
       "};
    assert_eq!(expected, actual, "empty context not formatted correctly");

    let inner_error = IoError::new(IoErrorKind::NotFound, "I am the root cause");
    let sift_error: Result<()> =
        Err(Error::new(ErrorKind::IoError, inner_error)).help("I am the help text");
    let actual = format!("{}", sift_error.unwrap_err());
    let expected = indoc! {"
        [IoError]

        [cause]:
           - I am the root cause

        [help]:
           - I am the help text
       "};
    assert_eq!(expected, actual, "empty context not formatted correctly");

    let inner_error = IoError::new(IoErrorKind::NotFound, "I am the root cause");
    let sift_error: Result<()> =
        Err(Error::new(ErrorKind::IoError, inner_error)).context("I should be first");
    let actual = format!("{}", sift_error.unwrap_err());
    let expected = indoc! {"
        [IoError]: I should be first

        [cause]:
           - I am the root cause
       "};
    assert_eq!(expected, actual, "single context not formatted correctly");

    let inner_error = IoError::new(IoErrorKind::NotFound, "I am the root cause");
    let sift_error: Result<()> = Err(Error::new(ErrorKind::IoError, inner_error))
        .context("I should be third")
        .context("I should be second")
        .context("I should be first")
        .help("I am the help text");
    let actual = format!("{}", sift_error.unwrap_err());
    let expected = indoc! {"
        [IoError]: I should be first

        [cause]:
           - I should be second
           - I should be third
           - I am the root cause

        [help]:
           - I am the help text
       "};
    assert_eq!(expected, actual, "error not formatted correctly");
}

#[test]
pub fn test_rate_limited_error_with_delay() {
    let delay = Duration::from_secs(2);
    let err = Error::new_rate_limited(IoError::other("Rate limit exceeded"), Some(delay));

    assert!(err.is_rate_limited());
    assert_eq!(err.retry_delay(), Some(delay));
    assert_eq!(
        err.kind(),
        ErrorKind::RateLimitedError {
            retry_delay: Some(delay)
        }
    );

    let actual = format!("{err}");
    let expected = indoc! {"
        [RateLimitedError]: request was rate limited by Sift; retry after 2s

        [cause]:
           - Rate limit exceeded
       "};
    assert_eq!(
        expected, actual,
        "rate-limited error not formatted correctly"
    );
}

#[test]
pub fn test_rate_limited_error_without_delay() {
    let err = Error::new_rate_limited(IoError::other("Rate limit exceeded"), None);

    assert!(err.is_rate_limited());
    assert_eq!(err.retry_delay(), None);

    let actual = format!("{err}");
    let expected = indoc! {"
        [RateLimitedError]: request was rate limited by Sift

        [cause]:
           - Rate limit exceeded
       "};
    assert_eq!(
        expected, actual,
        "rate-limited error not formatted correctly"
    );
}

#[test]
pub fn test_rate_limited_error_keeps_added_context() {
    let err: Result<()> = Err(Error::new_rate_limited(
        IoError::other("Rate limit exceeded"),
        Some(Duration::from_millis(500)),
    ))
    .context("failed to create flow");
    let err = err.unwrap_err();

    assert_eq!(err.retry_delay(), Some(Duration::from_millis(500)));

    let actual = format!("{err}");
    let expected = indoc! {"
        [RateLimitedError]: failed to create flow

        [cause]:
           - request was rate limited by Sift; retry after 500ms
           - Rate limit exceeded
       "};
    assert_eq!(
        expected, actual,
        "rate-limited error context not formatted correctly"
    );
}

#[test]
pub fn test_non_rate_limited_errors_have_no_retry_delay() {
    let err = Error::new(ErrorKind::CreateFlowError, IoError::other("too large"));
    assert!(!err.is_rate_limited());
    assert_eq!(err.retry_delay(), None);

    let err = Error::new_msg(ErrorKind::NotFoundError, "missing");
    assert!(!err.is_rate_limited());
    assert_eq!(err.retry_delay(), None);
}

#[test]
pub fn test_rate_limited_error_kind_equality() {
    let none = ErrorKind::RateLimitedError { retry_delay: None };
    let one_sec = ErrorKind::RateLimitedError {
        retry_delay: Some(Duration::from_secs(1)),
    };

    assert_eq!(none, ErrorKind::RateLimitedError { retry_delay: None });
    assert_ne!(none, one_sec);
    assert_ne!(one_sec, ErrorKind::GeneralError);
    assert_eq!(format!("{one_sec}"), "RateLimitedError");
}
