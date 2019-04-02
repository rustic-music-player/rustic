use commands::MpdCommand;
use failure::Error;
use rustic_core::{Rustic, MultiQuery};
use song::MpdSong;
use std::sync::Arc;

pub struct ListPlaylistInfoCommand {
    name: String,
}

impl ListPlaylistInfoCommand {
    pub fn new(name: String) -> ListPlaylistInfoCommand {
        ListPlaylistInfoCommand { name }
    }
}

impl MpdCommand<Vec<MpdSong>> for ListPlaylistInfoCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<Vec<MpdSong>, Error> {
        let playlists = app.library.query_playlists(MultiQuery::new())?;
        let playlist = playlists
            .iter()
            .find(|playlist| playlist.title == self.name);
        match playlist {
            Some(playlist) => {
                let tracks = playlist.tracks.iter().cloned().map(MpdSong::from).collect();
                Ok(tracks)
            }
            None => Ok(vec![]),
        }
    }
}
