use std::sync::Arc;

use failure::Error;
use itertools::Itertools;

use cursor::from_cursor;
use rustic_core::provider::CoverArt;
use rustic_core::{MultiQuery, QueryJoins, Rustic, SingleQuery};
use viewmodels::*;

pub fn fetch_album(cursor: &str, rustic: &Arc<Rustic>) -> Result<Option<AlbumModel>, Error> {
    let sw = stopwatch::Stopwatch::start_new();

    let uri = from_cursor(cursor)?;
    let mut query = SingleQuery::uri(uri);
    query.join_all();
    let album = rustic
        .query_album(query)?
        .map(|album| AlbumModel::new(album));
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
        .map(|album| AlbumModel::new(album))
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
        .map(|artist| ArtistModel::new(artist))
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
    let playlists = playlists
        .into_iter()
        .map(|playlist| PlaylistModel::new(playlist))
        .sorted() // TODO: sorting should probably happen in library
        .collect();

    Ok(playlists)
}

pub fn fetch_playlist(cursor: &str, rustic: &Arc<Rustic>) -> Result<Option<PlaylistModel>, Error> {
    let sw = stopwatch::Stopwatch::start_new();

    let uri = from_cursor(cursor)?;
    let mut query = SingleQuery::uri(uri);
    query.join_all();
    let playlist = rustic
        .query_playlist(query)?
        .map(|playlist| PlaylistModel::new(playlist));
    debug!("Fetching playlist took {}ms", sw.elapsed_ms());

    Ok(playlist)
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
        .map(|track| TrackModel::new(track))
        .collect();
    Ok(tracks)
}

pub fn fetch_track(cursor: &str, rustic: &Arc<Rustic>) -> Result<Option<TrackModel>, Error> {
    let uri = from_cursor(cursor)?;
    let query = SingleQuery::uri(uri);
    let track = rustic.query_track(query)?;
    let track = track.map(|track| TrackModel::new(track));

    Ok(track)
}

pub fn get_coverart_for_track(
    cursor: &str,
    rustic: &Arc<Rustic>,
) -> Result<Option<CoverArt>, Error> {
    let uri = from_cursor(cursor)?;
    let query = SingleQuery::uri(uri);
    let track = rustic.query_track(query)?;

    if let Some(track) = track {
        let cover_art = rustic.cover_art(&track)?;

        Ok(cover_art)
    } else {
        Ok(None)
    }
}
