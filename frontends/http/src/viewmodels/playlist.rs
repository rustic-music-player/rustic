use std::sync::Arc;

use cursor::to_cursor;
use rustic_core::library::Playlist;
use rustic_core::provider::Provider;
use rustic_core::Rustic;
use viewmodels::TrackModel;

#[derive(Clone, Debug, Serialize)]
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
