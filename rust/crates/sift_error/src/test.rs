use super::prelude::*;
use indoc::indoc;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

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
