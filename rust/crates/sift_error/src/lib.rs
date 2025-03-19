use std::{error::Error as StdError, fmt, result::Result as StdResult};

#[cfg(test)]
mod test;

pub mod prelude {
    pub use super::{Error, ErrorKind, Result, SiftError};
}

pub type Result<T> = StdResult<T, Error>;
pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;

pub trait SiftError<T, C>
where
    C: fmt::Display + Send + Sync + 'static,
{
    fn with_context<F>(self, op: F) -> Result<T>
    where
        F: Fn() -> C;

    fn context(self, ctx: C) -> Result<T>;
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

    pub fn new_msg<S: AsRef<str>>(kind: ErrorKind, msg: S) -> Self {
        Self {
            inner: None,
            kind,
            context: Some(vec![msg.as_ref().to_string()]),
            help: None,
        }
    }

    pub fn new_general<S: AsRef<str>>(msg: S) -> Self {
        Self::new_msg(ErrorKind::GeneralError, msg)
    }

    pub fn new_arg_error<S: AsRef<str>>(msg: S) -> Self {
        Self::new_msg(ErrorKind::ArgumentValidationError, msg)
    }

    pub fn new_empty_response<S: AsRef<str>>(msg: S) -> Self {
        Self {
            inner: None,
            kind: ErrorKind::EmptyResponseError,
            context: Some(vec![msg.as_ref().to_string()]),
            help: Some("please context Sift".to_string()),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ErrorKind {
    ArgumentValidationError,
    ConfigError,
    GrpcConnectError,
    RetrieveRunError,
    UpdateRunError,
    RetrieveIngestionConfigError,
    CreateRunError,
    CreateIngestionConfigError,
    CreateFlowError,
    NotFoundError,
    IoError,
    NumberConversionError,
    TimeConversionError,
    StreamError,
    BackupsError,
    ProtobufError,

    /// This really shouldn't happen
    EmptyResponseError,

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
            Self::GrpcConnectError => write!(f, "GrpcConnectError"),
            Self::RetrieveRunError => write!(f, "RetrieveRunError"),
            Self::RetrieveIngestionConfigError => write!(f, "RetrieveIngestionConfigError"),
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
            Self::BackupsError => write!(f, "BackupsError"),
            Self::ProtobufError => write!(f, "ProtobufError"),
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
                writeln!(f, "[{kind}]\n\n[cause]:{NEW_LINE_DELIMITER}{chain}\n\n[help]:{NEW_LINE_DELIMITER}- {help_txt}")
            }
            None if most_recent_cause.is_empty() => {
                writeln!(f, "[{kind}]\n\n[cause]:{NEW_LINE_DELIMITER}{chain}")
            }
            Some(help_txt) => {
                writeln!(f, "[{kind}]: {most_recent_cause}\n\n[cause]:{NEW_LINE_DELIMITER}{chain}\n\n[help]:{NEW_LINE_DELIMITER}- {help_txt}")
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
