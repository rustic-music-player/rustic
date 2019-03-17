use commands::MpdCommand;
use failure::Error;
use rustic_core::{Playlist, Rustic};
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct PlaylistEntry {
    playlist: String,
    #[serde(rename = "Last-Modified")]
    last_modified: String,
}

impl From<Playlist> for PlaylistEntry {
    fn from(playlist: Playlist) -> PlaylistEntry {
        PlaylistEntry {
            playlist: playlist.title,
            last_modified: "2017-12-23T17:15:13Z".to_owned(),
        }
    }
}

pub struct ListPlaylistsCommand {}

impl ListPlaylistsCommand {
    pub fn new() -> ListPlaylistsCommand {
        ListPlaylistsCommand {}
    }
}

impl MpdCommand<Vec<PlaylistEntry>> for ListPlaylistsCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<Vec<PlaylistEntry>, Error> {
        let playlists = app
            .library
            .get_playlists()?
            .into_iter()
            .map(PlaylistEntry::from)
            .collect();
        Ok(playlists)
    }
}
