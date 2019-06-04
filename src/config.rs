use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Clone, Default)]
pub struct Config {
    #[cfg(feature = "mpd")]
    pub mpd: Option<mpd_frontend::MpdConfig>,
    #[cfg(feature = "web-api")]
    pub http: Option<http_frontend::HttpConfig>,
    #[cfg(feature = "pocketcasts")]
    pub pocketcasts: Option<pocketcasts_provider::PocketcastsProvider>,
    #[cfg(feature = "soundcloud")]
    pub soundcloud: Option<soundcloud_provider::SoundcloudProvider>,
    #[cfg(feature = "spotify")]
    pub spotify: Option<spotify_provider::SpotifyProvider>,
    #[cfg(feature = "local-files")]
    pub local: Option<local_provider::LocalProvider>,
    pub library: Option<LibraryConfig>,
    #[serde(rename = "player", default = "default_backend")]
    pub players: Vec<PlayerBackendConfig>,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "store", rename_all = "lowercase")]
pub enum LibraryConfig {
    Memory,
    #[cfg(feature = "sqlite-store")]
    SQLite { path: String },
    #[cfg(feature = "sled-store")]
    Sled { path: String },
}

#[derive(Deserialize, Clone, PartialEq, Eq)]
pub struct PlayerBackendConfig {
    pub name: String,
    #[serde(default)]
    pub default: bool,
    #[serde(rename = "type")]
    pub backend_type: PlayerBackend,
}

#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PlayerBackend {
    GStreamer,
    Rodio,
}

fn default_backend() -> Vec<PlayerBackendConfig> {
    let config = PlayerBackendConfig {
        name: "default".to_string(),
        default: true,
        backend_type: PlayerBackend::GStreamer,
    };

    vec![config]
}

pub fn read_config() -> Config {
    let mut config_file = File::open("config.toml").unwrap();
    let mut config = String::new();
    config_file.read_to_string(&mut config).unwrap();
    toml::from_str(config.as_str()).unwrap()
}
