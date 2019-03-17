use failure::Error;
use rustic_core::{Playlist, Rustic, Track};
use std::sync::Arc;
use viewmodels::TrackModel;

pub fn fetch(rustic: &Arc<Rustic>) -> Result<Vec<TrackModel>, Error> {
    let player = rustic.get_default_player().ok_or(format_err!("Missing default player"))?;
    player
        .get_queue()
        .into_iter()
        .map(|track| TrackModel::new_with_joins(track, &rustic))
        .collect()
}

pub fn queue_track(track_id: usize, rustic: &Arc<Rustic>) -> Result<Option<()>, Error> {
    let library = &rustic.library;
    debug!("adding track to queue {}", track_id);
    let track: Option<Track> = library.get_track(track_id)?;
    match track {
        Some(track) => {
            let player = rustic.get_default_player().ok_or(format_err!("Missing default player"))?;
            player.queue_single(&track);

            Ok(Some(()))
        }
        None => Ok(None),
    }
}

pub fn queue_playlist(playlist_id: usize, rustic: &Arc<Rustic>) -> Result<Option<()>, Error> {
    let library = &rustic.library;
    debug!("adding playlist to queue {}", playlist_id);
    let playlist: Option<Playlist> = library.get_playlist(playlist_id)?;
    match playlist {
        Some(playlist) => {
            let player = rustic.get_default_player().ok_or(format_err!("Missing default player"))?;
            player.queue_multiple(&playlist.tracks);

            Ok(Some(()))
        }
        None => Ok(None),
    }
}

pub fn clear(rustic: &Arc<Rustic>) -> Result<(), Error> {
    debug!("Clearing queue");
    let player = rustic.get_default_player().ok_or(format_err!("Missing default player"))?;
    player.clear_queue();
    Ok(())
}
