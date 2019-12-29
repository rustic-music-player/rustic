use std::str::from_utf8;
use std::sync::Arc;

use failure::Error;

use cursor::from_cursor;
use rustic_core::{MultiQuery, QueryJoins, Rustic, SingleQuery};
use viewmodels::*;

pub fn fetch_album(cursor: &str, rustic: &Arc<Rustic>) -> Result<Option<AlbumModel>, Error> {
    let library = &rustic.library;
    let sw = stopwatch::Stopwatch::start_new();

    let uri = from_cursor(cursor)?;
    let mut query = SingleQuery::uri(uri.to_string());
    query.join_all();
    let album = library
        .query_album(query)?
        .map(|album| AlbumModel::new(album, &rustic));
    debug!("Fetching album took {}ms", sw.elapsed_ms());

    Ok(album)
}

pub fn fetch_albums(rustic: &Arc<Rustic>) -> Result<Vec<AlbumModel>, Error> {
    let library = &rustic.library;

    let sw = stopwatch::Stopwatch::start_new();
    let mut query = MultiQuery::new();
    query.join_artists();
    let albums = library.query_albums(query)?;
    debug!("Fetching albums took {}ms", sw.elapsed_ms());

    let albums = albums
        .into_iter()
        .map(|album| AlbumModel::new(album, &rustic))
        .collect();

    Ok(albums)
}

pub fn fetch_artists(rustic: &Arc<Rustic>) -> Result<Vec<ArtistModel>, Error> {
    let library = &rustic.library;

    let sw = stopwatch::Stopwatch::start_new();
    let artists = library.query_artists(MultiQuery::new())?;
    debug!("Fetching artists took {}ms", sw.elapsed_ms());

    let artists = artists
        .into_iter()
        .map(|artist| ArtistModel::new(artist, &rustic))
        .collect();
    Ok(artists)
}

pub fn fetch_playlists(rustic: &Arc<Rustic>) -> Result<Vec<PlaylistModel>, Error> {
    let library = &rustic.library;
    let sw = stopwatch::Stopwatch::start_new();
    let mut query = MultiQuery::new();
    query.join_tracks();
    let playlists = library.query_playlists(query)?;
    debug!("Fetching playlists took {}ms", sw.elapsed_ms());
    playlists
        .into_iter()
        .map(|playlist| PlaylistModel::new(playlist, &rustic))
        .collect()
}

pub fn fetch_tracks(rustic: &Arc<Rustic>) -> Result<Vec<TrackModel>, Error> {
    let library = &rustic.library;
    let sw = stopwatch::Stopwatch::start_new();
    let mut query = MultiQuery::new();
    query.join_artists();
    let tracks = library.query_tracks(query)?;
    debug!("Fetching tracks took {}ms", sw.elapsed_ms());
    let tracks = tracks
        .into_iter()
        .map(|track| TrackModel::new(track, &rustic))
        .collect();
    Ok(tracks)
}
