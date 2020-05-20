use std::fmt;

#[derive(Debug)]
pub(crate) enum FFIError {
    MissingArgument(&'static str),
    StringError(std::str::Utf8Error),
    Error(failure::Error),
}

impl From<failure::Error> for FFIError {
    fn from(err: failure::Error) -> Self {
        FFIError::Error(err)
    }
}

impl From<std::str::Utf8Error> for FFIError {
    fn from(err: std::str::Utf8Error) -> Self {
        FFIError::StringError(err)
    }
}

impl fmt::Display for FFIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for FFIError {}
