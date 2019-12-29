use std::sync::Arc;

use failure::Error;

use cursor::from_cursor;
use rustic_core::{Playlist, Rustic, SingleQuery, Track};
use viewmodels::TrackModel;

pub fn fetch(rustic: &Arc<Rustic>) -> Result<Vec<TrackModel>, Error> {
    let player = rustic
        .get_default_player()
        .ok_or_else(|| format_err!("Missing default player"))?;
    let tracks = player
        .get_queue()
        .into_iter()
        .map(|track| TrackModel::new(track, &rustic))
        .collect();
    Ok(tracks)
}

pub fn queue_track(cursor: &str, rustic: &Arc<Rustic>) -> Result<Option<()>, Error> {
    let uri = from_cursor(cursor)?;
    debug!("adding track to queue {}", uri);
    let track: Option<Track> = rustic.query_track(SingleQuery::uri(uri))?;
    match track {
        Some(track) => {
            let player = rustic
                .get_default_player()
                .ok_or_else(|| format_err!("Missing default player"))?;
            player.queue_single(&track);

            Ok(Some(()))
        }
        None => Ok(None),
    }
}

pub fn queue_playlist(cursor: &str, rustic: &Arc<Rustic>) -> Result<Option<()>, Error> {
    let library = &rustic.library;
    let uri = from_cursor(cursor)?;
    debug!("adding playlist to queue {}", uri);
    let playlist: Option<Playlist> = library.query_playlist(SingleQuery::uri(uri))?;
    match playlist {
        Some(playlist) => {
            let player = rustic
                .get_default_player()
                .ok_or_else(|| format_err!("Missing default player"))?;
            player.queue_multiple(&playlist.tracks);

            Ok(Some(()))
        }
        None => Ok(None),
    }
}

pub fn clear(rustic: &Arc<Rustic>) -> Result<(), Error> {
    debug!("Clearing queue");
    let player = rustic
        .get_default_player()
        .ok_or_else(|| format_err!("Missing default player"))?;
    player.clear_queue();
    Ok(())
}
