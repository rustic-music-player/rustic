use std::collections::HashMap;
use owoify_rs::{Owoifiable, OwoifyLevel};
use rayon::prelude::*;

use rustic_extension_api::*;
use rustic_core::{Track, Album, Artist, Playlist};

pub struct UwuExtension {
    level: OwoifyLevel
}

impl UwuExtension {
    fn owoify_tracks(&self, tracks: &mut [Track]) {
        tracks.par_iter_mut()
            .for_each(|track| {
                track.title = track.title.owoify(&self.level);
                if let Some(artist) = track.artist.as_mut() {
                    artist.name = artist.name.owoify(&self.level);
                }
                if let Some(album) = track.album.as_mut() {
                    album.title = album.title.owoify(&self.level);
                }
            });
    }
}

impl std::fmt::Debug for UwuExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UwuExtension").finish()
    }
}

impl ExtensionLibrary for UwuExtension {
    fn new(config: HashMap<String, ExtensionConfigValue>) -> Self {
        let level = match config.get("level") {
            Some(value) if value.is_string("owo") => OwoifyLevel::Owo,
            Some(value) if value.is_string("uwu") => OwoifyLevel::Uwu,
            Some(value) if value.is_string("uvu") => OwoifyLevel::Uvu,
            None => OwoifyLevel::Uwu,
            Some(value) => {
                // TODO: warn
                println!("invalid level value {:?}. Allowed values are: owo, uwu, uvu", value);
                OwoifyLevel::Uwu
            }
        };
        UwuExtension { level }
    }

    fn metadata() -> ExtensionMetadata {
        ExtensionMetadata {
            id: String::from("uwu"),
            name: String::from("UwU"),
            version: crate_version!(),
        }
    }
}

impl Extension for UwuExtension {}

#[async_trait::async_trait]
impl ExtensionApi for UwuExtension {
    async fn resolve_track(&self, mut track: Track) -> Result<Track, failure::Error> {
        track.title = track.title.owoify(&self.level);
        if let Some(artist) = track.artist.as_mut() {
            artist.name = artist.name.owoify(&self.level);
        }
        if let Some(album) = track.album.as_mut() {
            album.title = album.title.owoify(&self.level);
        }
        Ok(track)
    }

    async fn resolve_album(&self, mut album: Album) -> Result<Album, failure::Error> {
        album.title = album.title.owoify(&self.level);
        if let Some(artist) = album.artist.as_mut() {
            artist.name = artist.name.owoify(&self.level);
        }
        self.owoify_tracks(&mut album.tracks);
        Ok(album)
    }

    async fn resolve_artist(&self, mut artist: Artist) -> Result<Artist, failure::Error> {
        artist.name = artist.name.owoify(&self.level);
        Ok(artist)
    }

    async fn resolve_playlist(&self, mut playlist: Playlist) -> Result<Playlist, failure::Error> {
        playlist.title = playlist.title.owoify(&self.level);
        self.owoify_tracks(&mut playlist.tracks);
        Ok(playlist)
    }
}

host_extension!(UwuExtension);
