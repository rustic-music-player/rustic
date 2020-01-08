use maplit::hashmap;

use rustic_core::{Provider, Track};

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
        Track {
            id: None,
            title: track.title,
            artist: None,
            artist_id: None,
            album: None,
            album_id: None,
            uri: format!("gmusic:track:{}", track.store_id.unwrap_or(track.id)),
            provider: Provider::GooglePlayMusic,
            duration: None,
            image_url: track.album_art_ref.first().map(|image| image.url.clone()),
            meta,
        }
    }
}
