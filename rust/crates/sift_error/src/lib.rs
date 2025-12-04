use std::{error::Error as StdError, fmt, result::Result as StdResult};

#[cfg(test)]
mod test;

/// Other Sift crates should just import this prelude to get everything necessary to construct
/// [Error] types.
pub mod prelude {
    pub use super::{Error, ErrorKind, Result, SiftError};
}

/// A `Result` that returns [Error] as the error-type.
pub type Result<T> = StdResult<T, Error>;
pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;

/// Trait that defines the behavior of errors that Sift manages.
pub trait SiftError<T, C>
where
    C: fmt::Display + Send + Sync + 'static,
{
    /// Adds context that is printed with the error.
    fn context(self, ctx: C) -> Result<T>;

    /// Like `context` but takes in a closure.
    fn with_context<F>(self, op: F) -> Result<T>
    where
        F: Fn() -> C;

    /// User-help text.
    fn help(self, txt: C) -> Result<T>;
}

/// Error type returned across all Sift crates.
#[derive(Debug)]
pub struct Error {
    context: Option<Vec<String>>,
    help: Option<String>,
    kind: ErrorKind,
    inner: Option<BoxedError>,
}

impl StdError for Error {}

impl Error {
    /// Initializes an [Error].
    pub fn new<E>(kind: ErrorKind, err: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        let inner = Box::new(err);
        Self {
            inner: Some(inner),
            kind,
            context: None,
            help: None,
        }
    }

    /// Initializes an [Error] with a generic message.
    pub fn new_msg<S: AsRef<str>>(kind: ErrorKind, msg: S) -> Self {
        Self {
            inner: None,
            kind,
            context: Some(vec![msg.as_ref().to_string()]),
            help: None,
        }
    }

    /// Initializes a general catch-all type of [Error]. Contributors should be careful not to use
    /// this unless strictly necessary.
    pub fn new_general<S: AsRef<str>>(msg: S) -> Self {
        Self::new_msg(ErrorKind::GeneralError, msg)
    }

    /// Used for user-errors that have to do with bad arguments.
    pub fn new_arg_error<S: AsRef<str>>(msg: S) -> Self {
        Self::new_msg(ErrorKind::ArgumentValidationError, msg)
    }

    /// Tonic response types usually return optional types that we need to handle; if responses are
    /// empty then this is the appropriate way to initialize an [Error] for that situation, though
    /// this has never been observed.
    pub fn new_empty_response<S: AsRef<str>>(msg: S) -> Self {
        Self {
            inner: None,
            kind: ErrorKind::EmptyResponseError,
            context: Some(vec![msg.as_ref().to_string()]),
            help: Some("please contact Sift".to_string()),
        }
    }

    /// Get the underlying error kind.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

/// Various categories of errors that can occur throughout Sift crates.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ErrorKind {
    /// Indicates that the error is due to a resource already existing.
    AlreadyExistsError,
    /// Indicates user-error having to do with bad arguments.
    ArgumentValidationError,
    /// Indicates that the program is unable to grab credentials from a user's `sift.toml` file.
    ConfigError,
    /// Inidicates that the program was unable to connect to Sift.
    GrpcConnectError,
    /// Indicates that the program was unable to retrieve the run being requested.
    RetrieveRunError,
    /// Indicates that the program was unable to retrieve the asset being requested.
    RetrieveAssetError,
    /// Indicates that the program was unable to update the asset being requested.
    UpdateAssetError,
    /// Indicates a failure to update a run.
    UpdateRunError,
    /// Indicates that the program was unable to retrieve the ingestion config being requested.
    RetrieveIngestionConfigError,
    /// Indicates that the program was unable to encode the message being requested.
    EncodeMessageError,
    /// Indicates a failure to create a run.
    CreateRunError,
    /// Indicates a failure to create an ingestion config.
    CreateIngestionConfigError,
    /// Indicates a failure to create a flow.
    CreateFlowError,
    /// Indicates a failure to find the requested resource, likely because it doesn't exist.
    NotFoundError,
    /// General I/O errors.
    IoError,
    /// Indicates that there was a conversion between numeric times.
    NumberConversionError,
    /// Indicates a failure to generated a particular time-type from arguments.
    TimeConversionError,
    /// General errors that can occur while streaming telemetry i.e. data ingestion.
    StreamError,
    /// Indicates that all retries were exhausted in the configure retry policy.
    RetriesExhausted,
    /// General errors that can occur while processing backups during streaming.
    BackupsError,
    /// Indicates that the user is making a change that is not backwards compatible with an
    /// existing ingestion config.
    IncompatibleIngestionConfigChange,
    /// Indicates that a user provided a flow-name that doesn't match any configured flow in the
    /// parent ingestion config.
    UnknownFlow,
    /// This really shouldn't happen.
    EmptyResponseError,
    /// When failing to decode protobuf from its wire format.
    ProtobufDecodeError,
    /// When backup checksums don't match.
    BackupIntegrityError,
    /// When backup file/buffer limit has been reached.
    BackupLimitReached,
    /// Errors with the SiftStream Metrics Server
    SiftStreamMetricsServerError,
    /// General errors that are rarely returned.
    GeneralError,
}

impl<T, C> SiftError<T, C> for Result<T>
where
    C: fmt::Display + Send + Sync + 'static,
{
    fn with_context<F>(self, op: F) -> Result<T>
    where
        F: Fn() -> C,
    {
        self.map_err(|mut err| {
            if let Some(context) = err.context.as_mut() {
                context.push(format!("{}", op()));
            } else {
                err.context = Some(vec![format!("{}", op())]);
            }
            err
        })
    }

    fn context(self, ctx: C) -> Self {
        self.map_err(|mut err| {
            if let Some(context) = err.context.as_mut() {
                context.push(format!("{ctx}"));
            } else {
                err.context = Some(vec![format!("{ctx}")]);
            }
            err
        })
    }

    fn help(self, txt: C) -> Self {
        self.map_err(|mut err| {
            err.help = Some(format!("{txt}"));
            err
        })
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExistsError => write!(f, "AlreadyExistsError"),
            Self::GrpcConnectError => write!(f, "GrpcConnectError"),
            Self::RetriesExhausted => write!(f, "RetriesExhausted"),
            Self::RetrieveAssetError => write!(f, "RetrieveAssetError"),
            Self::UpdateAssetError => write!(f, "UpdateAssetError"),
            Self::RetrieveRunError => write!(f, "RetrieveRunError"),
            Self::RetrieveIngestionConfigError => write!(f, "RetrieveIngestionConfigError"),
            Self::EncodeMessageError => write!(f, "EncodeMessageError"),
            Self::EmptyResponseError => write!(f, "EmptyResponseError"),
            Self::NotFoundError => write!(f, "NotFoundError"),
            Self::CreateRunError => write!(f, "CreateRunError"),
            Self::ArgumentValidationError => write!(f, "ArgumentValidationError"),
            Self::GeneralError => write!(f, "GeneralError"),
            Self::IoError => write!(f, "IoError"),
            Self::ConfigError => write!(f, "ConfigError"),
            Self::UpdateRunError => write!(f, "UpdateRunError"),
            Self::CreateIngestionConfigError => write!(f, "CreateIngestionConfigError"),
            Self::NumberConversionError => write!(f, "NumberConversionError"),
            Self::CreateFlowError => write!(f, "CreateFlowError"),
            Self::TimeConversionError => write!(f, "TimeConversionError"),
            Self::StreamError => write!(f, "StreamError"),
            Self::UnknownFlow => write!(f, "UnknownFlow"),
            Self::BackupsError => write!(f, "BackupsError"),
            Self::BackupIntegrityError => write!(f, "BackupIntegrityError"),
            Self::BackupLimitReached => write!(f, "BackupLimitReached"),
            Self::ProtobufDecodeError => write!(f, "ProtobufDecodeError"),
            Self::IncompatibleIngestionConfigChange => {
                write!(f, "IncompatibleIngestionConfigChange")
            }
            Self::SiftStreamMetricsServerError => write!(f, "SiftStreamMetricsServerError"),
        }
    }
}

const NEW_LINE_DELIMITER: &str = "\n   ";

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Error {
            context,
            kind,
            help,
            inner,
        } = self;

        let root_cause = inner.as_ref().map(|e| format!("{e}"));

        let (most_recent_cause, chain) = context.as_ref().map_or_else(
            || {
                let root = root_cause.clone().unwrap_or_default();
                (String::new(), format!("- {root}"))
            },
            |c| {
                let mut cause_iter = c.iter().rev();

                if let Some(first) = cause_iter.next() {
                    let mut cause_chain = cause_iter
                        .map(|s| format!("- {s}"))
                        .collect::<Vec<String>>()
                        .join(NEW_LINE_DELIMITER);

                    if let Some(root) = root_cause.clone() {
                        if cause_chain.is_empty() {
                            cause_chain = format!("- {root}");
                        } else {
                            cause_chain = format!("{cause_chain}{NEW_LINE_DELIMITER}- {root}");
                        }
                    }

                    (first.clone(), cause_chain)
                } else {
                    (
                        String::new(),
                        root_cause
                            .as_ref()
                            .map_or_else(String::new, |s| format!("- {s}")),
                    )
                }
            },
        );

        match help {
            Some(help_txt) if most_recent_cause.is_empty() => {
                writeln!(
                    f,
                    "[{kind}]\n\n[cause]:{NEW_LINE_DELIMITER}{chain}\n\n[help]:{NEW_LINE_DELIMITER}- {help_txt}"
                )
            }
            None if most_recent_cause.is_empty() => {
                writeln!(f, "[{kind}]\n\n[cause]:{NEW_LINE_DELIMITER}{chain}")
            }
            Some(help_txt) => {
                writeln!(
                    f,
                    "[{kind}]: {most_recent_cause}\n\n[cause]:{NEW_LINE_DELIMITER}{chain}\n\n[help]:{NEW_LINE_DELIMITER}- {help_txt}"
                )
            }
            None => {
                writeln!(
                    f,
                    "[{kind}]: {most_recent_cause}\n\n[cause]:{NEW_LINE_DELIMITER}{chain}"
                )
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self {
            context: None,
            help: None,
            inner: Some(Box::new(value)),
            kind: ErrorKind::IoError,
        }
    }
}
