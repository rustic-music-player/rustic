use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use failure::{Error, Fail};
use serde_derive::{Deserialize, Serialize};

use crate::library::{SharedLibrary, Track};

pub use self::explorer::Explorer;
pub use self::folder::ProviderFolder;
pub use self::item::{ProviderItem, ProviderItemType};
pub use self::sync_error::SyncError;

mod explorer;
mod folder;
mod item;
mod sync_error;

pub type SharedProvider = Arc<RwLock<Box<dyn ProviderInstance + Send + Sync>>>;
pub type SharedProviders = Vec<SharedProvider>;

pub struct SyncResult {
    pub tracks: usize,
    pub albums: usize,
    pub artists: usize,
    pub playlists: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Provider {
    Pocketcasts,
    Soundcloud,
    GooglePlayMusic,
    Spotify,
    #[serde(rename = "local")]
    LocalMedia,
}

pub trait ProviderInstance: Debug {
    fn setup(&mut self) -> Result<(), Error>;
    fn title(&self) -> &'static str;
    fn uri_scheme(&self) -> &'static str;
    fn provider(&self) -> Provider;
    fn sync(&mut self, library: SharedLibrary) -> Result<SyncResult, Error>;
    fn root(&self) -> ProviderFolder;
    fn navigate(&self, path: Vec<String>) -> Result<ProviderFolder, Error>;
    fn search(&self, query: String) -> Result<Vec<ProviderItem>, Error>;
    fn resolve_track(&self, uri: &str) -> Result<Option<Track>, Error>;
    fn stream_url(&self, track: &Track) -> Result<String, Error>;
    fn cover_art(&self, track: &Track) -> Result<Option<CoverArt>, Error>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CoverArt {
    Data {
        data: Vec<u8>,
        mime_type: String
    },
    Url(String)
}

impl From<String> for CoverArt {
    fn from(url: String) -> Self {
        CoverArt::Url(url)
    }
}

#[derive(Debug, Fail)]
pub enum NavigationError {
    #[fail(display = "Path not found")]
    PathNotFound,
    #[fail(display = "can't fetch")]
    FetchError,
}
