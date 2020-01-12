use std::cmp::Ordering;
use std::sync::Arc;

use cursor::to_cursor;
use rustic_core::library::Track;
use rustic_core::provider::Provider;
use rustic_core::Rustic;
use viewmodels::AlbumModel;
use viewmodels::ArtistModel;

#[derive(Clone, Debug, Serialize, Eq)]
pub struct TrackModel {
    pub cursor: String,
    pub title: String,
    pub artist: Option<ArtistModel>,
    pub album: Option<AlbumModel>,
    pub provider: Provider,
    pub coverart: Option<String>,
    pub duration: Option<u64>,
}

impl TrackModel {
    pub fn new(track: Track, app: &Arc<Rustic>) -> TrackModel {
        let cursor = to_cursor(&track.uri);
        TrackModel {
            cursor: cursor.clone(),
            title: track.title,
            provider: track.provider,
            coverart: if track.has_coverart { Some(format!("/api/tracks/{}/coverart", &cursor)) } else { None },
            duration: track.duration,
            artist: track.artist.map(|artist| ArtistModel::new(artist, app)),
            album: track.album.map(|album| AlbumModel::new(album, app)),
        }
    }
}

impl PartialEq for TrackModel {
    fn eq(&self, other: &Self) -> bool {
        self.cursor == other.cursor
    }
}

impl PartialOrd for TrackModel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TrackModel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.title.to_lowercase().cmp(&other.title.to_lowercase())
    }
}
