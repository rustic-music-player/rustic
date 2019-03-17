use failure::Fail;
use std::fmt;
use std::sync;

#[derive(Debug, Fail)]
pub enum SyncError {
    ConfigurationError,
    LibraryAccessError, //(sync::PoisonError<sync::MutexGuard<'_, Library>>)
}

impl<T> From<sync::PoisonError<T>> for SyncError {
    fn from(_: sync::PoisonError<T>) -> SyncError {
        SyncError::LibraryAccessError
    }
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SyncError::ConfigurationError => write!(f, "Configuration Error"),
            SyncError::LibraryAccessError => write!(f, "Library Access Error"),
        }
    }
}
