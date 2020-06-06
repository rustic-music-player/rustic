use std::collections::HashMap;

use maplit::hashmap;

use rustic_core::{Album, Artist, ProviderType, Track};

use crate::meta::*;

#[derive(Debug, Clone)]
pub struct GmusicTrack(gmusic::Track);

impl From<gmusic::Track> for GmusicTrack {
    fn from(track: gmusic::Track) -> Self {
        GmusicTrack(track)
    }
}

impl From<GmusicTrack> for Track {
    fn from(track: GmusicTrack) -> Self {
        let track = track.0;
        let mut meta = hashmap!(
            META_GMUSIC_TRACK_ID.into() => track.id.clone().into(),
        );
        if let Some(store_id) = track.store_id.clone() {
            meta.insert(META_GMUSIC_STORE_ID.into(), store_id.into());
        }
        if let Some(image) = track.album_art_ref.first() {
            meta.insert(META_GMUSIC_COVER_ART_URL.into(), image.url.clone().into());
        }
        let artist_uri = format!(
            "gmusic:artist:{}",
            &track
                .artist_id
                .first()
                .cloned()
                .unwrap_or_else(|| String::from("unknown"))
        );
        let album_uri = format!("gmusic:album:{}", track.album_id.unwrap_or_else(|| "unknown".into()));
        Track {
            id: None,
            title: track.title,
            artist: Some(Artist {
                id: None,
                name: track.artist,
                uri: artist_uri,
                image_url: None,
                meta: HashMap::new(),
                provider: ProviderType::GooglePlayMusic,
                albums: vec![],
                playlists: vec![],
            }),
            artist_id: None,
            album: Some(Album {
                id: None,
                title: track.album,
                uri: album_uri,
                provider: ProviderType::GooglePlayMusic,
                tracks: Vec::new(),
                image_url: track
                    .album_art_ref
                    .first()
                    .map(|art_ref| art_ref.url.clone()),
                artist: None,
                artist_id: None,
                meta: HashMap::new(),
            }),
            album_id: None,
            uri: format!("gmusic:track:{}", track.store_id.unwrap_or(track.id)),
            provider: ProviderType::GooglePlayMusic,
            duration: track
                .duration_millis
                .parse::<u64>()
                .ok()
                .map(|duration| duration / 1000),
            has_coverart: track.album_art_ref.first().is_some(),
            meta,
        }
    }
}
