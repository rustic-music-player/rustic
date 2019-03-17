use std::collections::HashMap;

use rspotify::spotify::model::album::{FullAlbum, SimplifiedAlbum};
use serde_derive::{Serialize, Deserialize};
use rustic_core::library::Album;
use rustic_core::provider;
use crate::util::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotifyFullAlbum(FullAlbum);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotifySimplifiedAlbum(SimplifiedAlbum);

impl From<SpotifyFullAlbum> for Album {
    fn from(album: SpotifyFullAlbum) -> Self {
        let album = album.0;
        let artist = artists_to_artist(album.artists);
        Album {
            id: None,
            title: album.name,
            artist_id: None,
            artist,
            provider: provider::Provider::Spotify,
            image_url: convert_images(&album.images),
            uri: format!("spotify://album/{}", album.id),
            meta: HashMap::new(),
        }
    }
}

impl From<SpotifySimplifiedAlbum> for Album {
    fn from(album: SpotifySimplifiedAlbum) -> Self {
        let album = album.0;
        let artist = artists_to_artist(album.artists);
        Album {
            id: None,
            title: album.name,
            artist_id: None,
            artist,
            provider: provider::Provider::Spotify,
            image_url: convert_images(&album.images),
            uri: format!("spotify://album/{}", album.id),
            meta: HashMap::new(),
        }
    }
}

impl From<FullAlbum> for SpotifyFullAlbum {
    fn from(album: FullAlbum) -> Self {
        SpotifyFullAlbum(album)
    }
}

impl From<SimplifiedAlbum> for SpotifySimplifiedAlbum {
    fn from(album: SimplifiedAlbum) -> Self {
        SpotifySimplifiedAlbum(album)
    }
}
