use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use failure::{Error, Fail};
use serde_derive::{Deserialize, Serialize};
use url::Url;

use async_trait::async_trait;

use crate::library::{Album, SharedLibrary, Track};
use crate::{Playlist, CredentialStore};

pub use self::explorer::Explorer;
pub use self::folder::ProviderFolder;
pub use self::item::{ProviderItem, ProviderItemType};
pub use self::sync_error::SyncError;

mod explorer;
mod folder;
mod item;
mod sync_error;

type SharedProvider = Arc<RwLock<Box<dyn ProviderInstance + Send + Sync>>>;
pub type SharedProviders = Vec<Provider>;

#[derive(Debug, Clone)]
pub struct Provider {
    title: String,
    pub uri_scheme: String,
    pub provider_type: ProviderType,
    pub provider: SharedProvider,
}

impl Provider {
    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub async fn get(&self) -> RwLockReadGuard<'_, Box<dyn ProviderInstance + Send + Sync>> {
        self.provider.read().await
    }

    pub async fn get_mut(&self) -> RwLockWriteGuard<'_, Box<dyn ProviderInstance + Send + Sync>> {
        self.provider.write().await
    }
}

impl From<Box<dyn ProviderInstance + Send + Sync>> for Provider {
    fn from(instance: Box<dyn ProviderInstance + Send + Sync>) -> Self {
        let title = instance.title().to_owned();
        let uri_scheme = instance.uri_scheme().to_owned();
        let provider_type = instance.provider();

        Provider {
            title,
            uri_scheme,
            provider_type,
            provider: Arc::new(RwLock::new(instance)),
        }
    }
}

pub struct SyncResult {
    pub tracks: usize,
    pub albums: usize,
    pub artists: usize,
    pub playlists: usize,
}

impl SyncResult {
    pub fn empty() -> Self {
        SyncResult {
            tracks: 0,
            albums: 0,
            artists: 0,
            playlists: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    Pocketcasts,
    Soundcloud,
    #[serde(rename = "gmusic")]
    GooglePlayMusic,
    Spotify,
    #[serde(rename = "local")]
    LocalMedia,
    Youtube,
}

#[async_trait]
pub trait ProviderInstance: Debug {
    async fn setup(&mut self, cred_store: &dyn CredentialStore) -> Result<(), Error>;
    fn title(&self) -> &'static str;
    fn uri_scheme(&self) -> &'static str;
    fn provider(&self) -> ProviderType;
    fn auth_state(&self) -> AuthState;
    async fn authenticate(&mut self, auth: Authentication, cred_store: &dyn CredentialStore) -> Result<(), Error>;
    async fn sync(&self, library: SharedLibrary) -> Result<SyncResult, Error>;
    fn root(&self) -> ProviderFolder;
    async fn navigate(&self, path: Vec<String>) -> Result<ProviderFolder, Error>;
    async fn search(&self, query: String) -> Result<Vec<ProviderItem>, Error>;
    async fn resolve_track(&self, uri: &str) -> Result<Option<Track>, Error>;
    async fn resolve_album(&self, uri: &str) -> Result<Option<Album>, Error>;
    async fn resolve_playlist(&self, uri: &str) -> Result<Option<Playlist>, Error>;
    async fn stream_url(&self, track: &Track) -> Result<String, Error>;
    async fn cover_art(&self, track: &Track) -> Result<Option<CoverArt>, Error>;
    async fn resolve_share_url(&self, url: Url) -> Result<Option<InternalUri>, Error>;
}

#[derive(Debug, Clone)]
pub enum AuthState {
    NoAuthentication,
    RequiresOAuth(String),
    RequiresPassword,
    Authenticated(Option<User>),
}

impl AuthState {
    pub fn is_authenticated(&self) -> bool {
        match self {
            AuthState::NoAuthentication => true,
            AuthState::Authenticated(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct User {
    pub display_name: Option<String>,
    pub email: Option<String>,
}

// TODO: what about refresh and auth token?
#[derive(Debug, Clone)]
pub enum Authentication {
    Token(String),
    TokenWithState(String, String),
    Password(String, String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CoverArt {
    Data { data: Vec<u8>, mime_type: String },
    Url(String),
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

// TODO: better name
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum InternalUri {
    Track(String),
    Album(String),
    Artist(String),
    Playlist(String),
}
