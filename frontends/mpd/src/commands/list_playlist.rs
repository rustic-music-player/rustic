use commands::MpdCommand;
use failure::Error;
use rustic_core::{MultiQuery, Rustic, Track};
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct PlaylistItem {
    file: String,
}

impl From<Track> for PlaylistItem {
    fn from(track: Track) -> PlaylistItem {
        PlaylistItem { file: track.uri }
    }
}

pub struct ListPlaylistCommand {
    name: String,
}

impl ListPlaylistCommand {
    pub fn new(name: String) -> ListPlaylistCommand {
        ListPlaylistCommand { name }
    }
}

impl MpdCommand<Vec<PlaylistItem>> for ListPlaylistCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<Vec<PlaylistItem>, Error> {
        let playlists = app.library.query_playlists(MultiQuery::new())?;
        let playlist = playlists
            .iter()
            .find(|playlist| playlist.title == self.name);
        match playlist {
            Some(playlist) => {
                let tracks = playlist
                    .tracks
                    .iter()
                    .cloned()
                    .map(PlaylistItem::from)
                    .collect();
                Ok(tracks)
            }
            None => Ok(vec![]),
        }
    }
}
