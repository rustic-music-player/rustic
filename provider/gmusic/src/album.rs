use std::collections::HashMap;

use maplit::hashmap;

use rustic_core::{Album, Artist, ProviderType, Track};

use crate::meta::*;
use crate::track::GmusicTrack;
use rustic_core::provider::ThumbnailState;

#[derive(Debug, Clone)]
pub struct GmusicAlbum(gmusic::Album);

impl From<gmusic::Album> for GmusicAlbum {
    fn from(album: gmusic::Album) -> Self {
        GmusicAlbum(album)
    }
}

impl From<GmusicAlbum> for Album {
    fn from(album: GmusicAlbum) -> Self {
        let album = album.0;
        Album {
            id: None,
            title: album.name,
            artist: Some(Artist {
                id: None,
                name: album.artist,
                uri: format!(
                    "gmusic:artist:{}",
                    album
                        .artist_id
                        .first()
                        .cloned()
                        .unwrap_or_else(|| String::from("unknown"))
                ),
                image_url: None,
                meta: HashMap::new(),
                provider: ProviderType::GooglePlayMusic,
                playlists: Vec::new(),
                albums: Vec::new(),
            }),
            artist_id: None,
            uri: format!("gmusic:album:{}", &album.id),
            provider: ProviderType::GooglePlayMusic,
            meta: hashmap!(
                META_GMUSIC_ALBUM_ID.into() => album.id.into()
            ),
            tracks: album
                .tracks
                .into_iter()
                .map(GmusicTrack::from)
                .map(Track::from)
                .collect(),
            thumbnail: album.album_art_ref.map(ThumbnailState::Url).unwrap_or_default(),
        }
    }
}
