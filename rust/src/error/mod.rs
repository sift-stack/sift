use std::{error::Error as StdError, fmt, result::Result as StdResult};

pub type Result<T> = StdResult<T, Error>;
pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;

pub(crate) trait SiftError<T, C>
where
    C: fmt::Display + Send + Sync + 'static,
{
    fn context(self, ctx: C) -> Result<T>;
    fn help(self, txt: C) -> Result<T>;
}

/// Error specific to this library
#[derive(Debug)]
pub struct Error {
    context: Option<String>,
    help: Option<String>,
    kind: ErrorKind,
    inner: BoxedError,
}

const SPACING: &str = "\n  ";

impl StdError for Error {}

impl Error {
    pub fn new_user_error<E>(err: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        let inner = Box::new(err);
        Self {
            inner,
            kind: ErrorKind::User,
            context: None,
            help: None,
        }
    }

    pub fn new_internal_error<E>(err: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        let inner = Box::new(err);
        Self {
            inner,
            kind: ErrorKind::Internal,
            context: None,
            help: None,
        }
    }

    pub fn into_inner(self) -> BoxedError {
        self.inner
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Internal,
    User,
}

impl<T, C> SiftError<T, C> for Result<T>
where
    C: fmt::Display + Send + Sync + 'static,
{
    fn context(self, ctx: C) -> Self {
        self.map_err(|mut err| {
            err.context = Some(format!("{ctx}"));
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
            Self::Internal => writeln!(f, "Internal error"),
            Self::User => writeln!(f, "User error"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Error {
            context,
            kind,
            help,
            inner,
        } = self;

        match (context, help) {
            (Some(ctx), Some(help_txt)) => {
                writeln!(
                    f,
                    "({kind}) {ctx}{SPACING}[cause]: {inner}{SPACING}[help]: {help_txt}"
                )
            }
            (Some(ctx), None) => {
                writeln!(f, "({kind}) {ctx}{SPACING}[cause]: {inner}{SPACING}")
            }
            (None, Some(help_txt)) => {
                writeln!(
                    f,
                    "({kind}){SPACING}[cause]: {inner}{SPACING}[help]: {help_txt}"
                )
            }
            (None, None) => {
                writeln!(f, "({kind}){SPACING}[cause]: {inner}")
            }
        }
    }
}
