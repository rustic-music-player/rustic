use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Clone, Default)]
pub struct Config {
    pub mpd: Option<mpd_frontend::MpdConfig>,
    pub http: Option<http_frontend::HttpConfig>,
    pub pocketcasts: Option<pocketcasts_provider::PocketcastsProvider>,
    pub soundcloud: Option<soundcloud_provider::SoundcloudProvider>,
    pub spotify: Option<spotify_provider::SpotifyProvider>,
    pub local: Option<local_provider::LocalProvider>,
    pub library: Option<LibraryConfig>,
    #[serde(rename = "player", default = "default_backend")]
    pub players: Vec<PlayerBackendConfig>,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "store", rename_all = "lowercase")]
pub enum LibraryConfig {
    Memory,
    SQLite { path: String },
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
