use failure::Error;
use rustic_core::{Rustic, MultiQuery};
use std::sync::Arc;
use viewmodels::*;

pub fn fetch_album(album_id: usize, rustic: &Arc<Rustic>) -> Result<Option<AlbumModel>, Error> {
    let library = &rustic.library;
    let album = library.get_album(album_id)?.map(|album| AlbumModel::new(album, &rustic));

    Ok(album)
}

pub fn fetch_albums(rustic: &Arc<Rustic>) -> Result<Vec<AlbumModel>, Error> {
    let library = &rustic.library;
    let sw = stopwatch::Stopwatch::start_new();
    let albums = library
        .get_albums()?;
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
    let artists = library
        .get_artists()?;
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
    let playlists = library
        .get_playlists()?;
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
    query.joins(true);
    let tracks = library
        .query_tracks(query)?;
    debug!("Fetching tracks took {}ms", sw.elapsed_ms());
    let tracks = tracks
        .into_iter()
        .map(|track| TrackModel::new(track, &rustic))
        .collect();
    Ok(tracks)
}
