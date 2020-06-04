use log::error;
use rustic_core::provider;

#[derive(Debug)]
pub struct SoundcloudError(soundcloud::Error);

impl From<soundcloud::Error> for SoundcloudError {
    fn from(error: soundcloud::Error) -> Self {
        SoundcloudError(error)
    }
}

impl From<SoundcloudError> for provider::SyncError {
    fn from(error: SoundcloudError) -> Self {
        error!("{:?}", error);
        provider::SyncError::ConfigurationError
    }
}

impl From<SoundcloudError> for provider::NavigationError {
    fn from(error: SoundcloudError) -> Self {
        error!("{:?}", error);
        provider::NavigationError::FetchError
    }
}
