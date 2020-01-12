use std::sync::Arc;

use failure::_core::cmp::Ordering;

use cursor::to_cursor;
use rustic_core::library::Playlist;
use rustic_core::provider::Provider;
use rustic_core::Rustic;
use viewmodels::TrackModel;

#[derive(Clone, Debug, Serialize, Eq)]
pub struct PlaylistModel {
    pub cursor: String,
    pub title: String,
    pub tracks: Vec<TrackModel>,
    pub provider: Provider,
}

impl PlaylistModel {
    pub fn new(playlist: Playlist, app: &Arc<Rustic>) -> PlaylistModel {
        let tracks = playlist
            .tracks
            .into_iter()
            .map(|track| TrackModel::new(track, app))
            .collect();

        PlaylistModel {
            cursor: to_cursor(&playlist.uri),
            title: playlist.title,
            tracks,
            provider: playlist.provider,
        }
    }
}

impl PartialEq for PlaylistModel {
    fn eq(&self, other: &Self) -> bool {
        self.cursor == other.cursor
    }
}

impl PartialOrd for PlaylistModel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlaylistModel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.title.to_lowercase().cmp(&other.title.to_lowercase())
    }
}