use std::fs::File;
use std::io::prelude::*;
#[cfg(feature = "google-cast-backend")]
use std::net::IpAddr;
use std::path::Path;

use failure::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default, rename = "credentials")]
    pub credential_store: CredentialStoreConfig,
    #[serde(default)]
    pub frontend: FrontendConfig,
    #[serde(default)]
    pub provider: ProviderConfig,
    #[serde(default)]
    pub library: LibraryConfig,
    #[serde(rename = "player", default = "default_backend")]
    pub players: Vec<PlayerBackendConfig>,
    #[serde(default)]
    pub extensions: ExtensionConfig,
    #[serde(default)]
    pub discovery: DiscoveryConfig,
    #[serde(default)]
    pub client: ClientConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            credential_store: CredentialStoreConfig::default(),
            frontend: FrontendConfig::default(),
            provider: ProviderConfig::default(),
            library: LibraryConfig::default(),
            players: default_backend(),
            extensions: ExtensionConfig::default(),
            discovery: DiscoveryConfig::default(),
            client: ClientConfig::default(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum CredentialStoreConfig {
    Keychain,
    File { path: String },
}

impl Default for CredentialStoreConfig {
    fn default() -> Self {
        CredentialStoreConfig::Keychain
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct FrontendConfig {
    #[cfg(feature = "mpd-frontend")]
    pub mpd: Option<rustic_mpd_frontend::MpdConfig>,
    #[cfg(feature = "http-frontend")]
    pub http: Option<rustic_http_frontend::HttpConfig>,
    #[cfg(feature = "iced-frontend")]
    #[serde(default)]
    pub iced: Option<IcedConfig>,
}

// TODO: fill with options and move to iced frontend crate
#[derive(Deserialize, Debug, Clone)]
pub struct IcedConfig {}

impl Default for FrontendConfig {
    fn default() -> Self {
        FrontendConfig {
            #[cfg(feature = "mpd-frontend")]
            mpd: None,
            #[cfg(feature = "http-frontend")]
            http: Some(rustic_http_frontend::HttpConfig::default()),
            #[cfg(feature = "iced-frontend")]
            iced: None,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ProviderConfig {
    #[cfg(feature = "pocketcasts-provider")]
    #[serde(default = "rustic_pocketcasts_provider::PocketcastsProvider::new")]
    pub pocketcasts: Option<rustic_pocketcasts_provider::PocketcastsProvider>,
    #[cfg(feature = "soundcloud-provider")]
    #[serde(default = "rustic_soundcloud_provider::SoundcloudProvider::new")]
    pub soundcloud: Option<rustic_soundcloud_provider::SoundcloudProvider>,
    #[cfg(feature = "spotify-provider")]
    #[serde(default = "rustic_spotify_provider::SpotifyProvider::new")]
    pub spotify: Option<rustic_spotify_provider::SpotifyProvider>,
    #[cfg(feature = "gmusic-provider")]
    #[serde(default = "rustic_gmusic_provider::GooglePlayMusicProvider::new")]
    pub gmusic: Option<rustic_gmusic_provider::GooglePlayMusicProvider>,
    #[cfg(feature = "local-files-provider")]
    #[serde(default = "rustic_local_provider::LocalProvider::new")]
    pub local: Option<rustic_local_provider::LocalProvider>,
    #[cfg(feature = "youtube-provider")]
    #[serde(default = "rustic_youtube_provider::YoutubeProvider::new")]
    pub youtube: Option<rustic_youtube_provider::YoutubeProvider>,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        ProviderConfig {
            #[cfg(feature = "pocketcasts-provider")]
            pocketcasts: rustic_pocketcasts_provider::PocketcastsProvider::new(),
            #[cfg(feature = "soundcloud-provider")]
            soundcloud: rustic_soundcloud_provider::SoundcloudProvider::new(),
            #[cfg(feature = "spotify-provider")]
            spotify: rustic_spotify_provider::SpotifyProvider::new(),
            #[cfg(feature = "gmusic-provider")]
            gmusic: rustic_gmusic_provider::GooglePlayMusicProvider::new(),
            #[cfg(feature = "local-files-provider")]
            local: rustic_local_provider::LocalProvider::new(),
            #[cfg(feature = "youtube-provider")]
            youtube: rustic_youtube_provider::YoutubeProvider::new(),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "store", rename_all = "lowercase")]
pub enum LibraryConfig {
    Memory,
    #[cfg(feature = "sqlite-store")]
    SQLite {
        path: String,
    },
    #[cfg(feature = "sled-store")]
    Sled {
        path: String,
    },
}

impl Default for LibraryConfig {
    fn default() -> Self {
        LibraryConfig::Memory
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PlayerBackendConfig {
    pub name: String,
    #[serde(default)]
    pub default: bool,
    #[serde(flatten)]
    pub backend_type: PlayerBackend,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum PlayerBackend {
    #[cfg(feature = "gstreamer-backend")]
    GStreamer,
    #[cfg(feature = "rodio-backend")]
    Rodio,
    #[cfg(feature = "google-cast-backend")]
    GoogleCast { ip: IpAddr },
}

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct ExtensionConfig {
    pub path: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DiscoveryConfig {
    #[cfg(feature = "google-cast-backend")]
    #[serde(default = "default_cast_discovery")]
    pub google_cast: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        DiscoveryConfig {
            #[cfg(feature = "google-cast-backend")]
            google_cast: default_cast_discovery(),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum ClientConfig {
    Native,
    // TODO: should we disable player setup, provider setup etc when using remote client?
    #[cfg(feature = "http-client")]
    Http {
        url: String,
    },
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig::Native
    }
}

#[cfg(any(feature = "rodio-backend", feature = "gstreamer-backend"))]
fn default_backend() -> Vec<PlayerBackendConfig> {
    #[cfg(feature = "rodio-backend")]
    #[allow(unused_variables)]
    let backend_type = PlayerBackend::Rodio;
    #[cfg(feature = "gstreamer-backend")]
    let backend_type = PlayerBackend::GStreamer;
    let config = PlayerBackendConfig {
        name: "default".to_string(),
        default: true,
        backend_type,
    };

    vec![config]
}

#[cfg(not(any(feature = "rodio-backend", feature = "gstreamer-backend")))]
fn default_backend() -> Vec<PlayerBackendConfig> {
    Vec::new()
}

#[cfg(feature = "google-cast-backend")]
fn default_cast_discovery() -> bool {
    true
}

pub fn read_config(path: &Path) -> Result<Config, Error> {
    if path.exists() {
        let mut config_file = File::open(path)?;
        let mut config = String::new();
        config_file.read_to_string(&mut config)?;
        let config = toml::from_str(config.as_str())?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}
