use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use rustic_core::{Track, Album, Artist, Playlist};

pub use crate::ExtensionRuntime;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ExtensionConfigValue {
    Bool(bool),
    String(String),
    Float(f64),
    Int(i64),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtensionMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
}

pub trait ExtensionLibrary: Extension {
    fn new(config: HashMap<String, ExtensionConfigValue>) -> Self;

    fn metadata() -> ExtensionMetadata;
}

pub trait Extension: std::fmt::Debug + Send + Sync + ExtensionApi {
    fn setup(&mut self, runtime: &ExtensionRuntime) -> Result<(), failure::Error> {
        Ok(())
    }
}

#[async_trait]
pub trait ExtensionApi {
    async fn on_enable(&self) -> Result<(), failure::Error> {
        Ok(())
    }
    async fn on_disable(&self) -> Result<(), failure::Error> {
        Ok(())
    }

    async fn on_add_to_queue(&self, tracks: Vec<Track>) -> Result<Vec<Track>, failure::Error> {
        Ok(tracks)
    }

    async fn resolve_track(&self, track: Track) -> Result<Track, failure::Error> {
        Ok(track)
    }

    async fn resolve_album(&self, album: Album) -> Result<Album, failure::Error> {
        Ok(album)
    }

    async fn resolve_artist(&self, artist: Artist) -> Result<Artist, failure::Error> {
        Ok(artist)
    }

    async fn resolve_playlist(&self, playlist: Playlist) -> Result<Playlist, failure::Error> {
        Ok(playlist)
    }
}
