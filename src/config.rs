use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use failure::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
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
}

#[derive(Deserialize, Debug, Clone)]
pub struct FrontendConfig {
    #[cfg(feature = "mpd")]
    pub mpd: Option<rustic_mpd_frontend::MpdConfig>,
    #[cfg(feature = "web-api")]
    pub http: Option<rustic_http_frontend::HttpConfig>,
}

impl Default for FrontendConfig {
    fn default() -> Self {
        FrontendConfig {
            #[cfg(feature = "mpd")]
            mpd: None,
            #[cfg(feature = "web-api")]
            http: Some(rustic_http_frontend::HttpConfig::default()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct ProviderConfig {
    #[cfg(feature = "pocketcasts")]
    pub pocketcasts: Option<rustic_pocketcasts_provider::PocketcastsProvider>,
    #[cfg(feature = "soundcloud")]
    pub soundcloud: Option<rustic_soundcloud_provider::SoundcloudProvider>,
    #[cfg(feature = "spotify")]
    pub spotify: Option<rustic_spotify_provider::SpotifyProvider>,
    #[cfg(feature = "gmusic")]
    pub gmusic: Option<rustic_gmusic_provider::GooglePlayMusicProvider>,
    #[cfg(feature = "local-files")]
    pub local: Option<rustic_local_provider::LocalProvider>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            frontend: FrontendConfig::default(),
            provider: ProviderConfig::default(),
            library: LibraryConfig::default(),
            players: default_backend(),
            extensions: ExtensionConfig::default(),
            discovery: DiscoveryConfig::default(),
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
    #[cfg(feature = "gstreamer")]
    GStreamer,
    #[cfg(feature = "rodio")]
    Rodio,
    #[cfg(feature = "google-cast")]
    GoogleCast { ip: IpAddr },
}

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct ExtensionConfig {
    pub path: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DiscoveryConfig {
    #[cfg(feature = "google-cast")]
    #[serde(default = "default_cast_discovery")]
    pub google_cast: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        DiscoveryConfig {
            #[cfg(feature = "google-cast")]
            google_cast: default_cast_discovery(),
        }
    }
}

fn default_backend() -> Vec<PlayerBackendConfig> {
    #[cfg(feature = "gstreamer")]
    #[allow(unused_variables)]
    let backend_type = PlayerBackend::GStreamer;
    #[cfg(feature = "rodio")]
    let backend_type = PlayerBackend::Rodio;
    let config = PlayerBackendConfig {
        name: "default".to_string(),
        default: true,
        backend_type,
    };

    vec![config]
}

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
