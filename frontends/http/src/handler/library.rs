use failure::Error;
use rustic_core::Rustic;
use std::sync::Arc;
use viewmodels::*;

pub fn fetch_album(album_id: usize, rustic: &Arc<Rustic>) -> Result<Option<AlbumModel>, Error> {
    let library = &rustic.library;
    let album: Option<AlbumModel> = library.get_album(album_id).and_then(|album| match album {
        Some(album) => Ok(Some(AlbumModel::new_with_joins(album, &rustic)?)),
        None => Ok(None),
    })?;

    Ok(album)
}

pub fn fetch_albums(rustic: &Arc<Rustic>) -> Result<Vec<AlbumModel>, Error> {
    let library = &rustic.library;
    library
        .get_albums()?
        .into_iter()
        .map(|album| AlbumModel::new_with_joins(album, &rustic))
        .collect()
}

pub fn fetch_artists(rustic: &Arc<Rustic>) -> Result<Vec<ArtistModel>, Error> {
    let library = &rustic.library;
    library
        .get_artists()?
        .into_iter()
        .map(|artist| ArtistModel::new_with_joins(artist, &rustic))
        .collect()
}

pub fn fetch_playlists(rustic: &Arc<Rustic>) -> Result<Vec<PlaylistModel>, Error> {
    let library = &rustic.library;
    library
        .get_playlists()?
        .into_iter()
        .map(|playlist| PlaylistModel::from(playlist, &rustic))
        .collect()
}

pub fn fetch_tracks(rustic: &Arc<Rustic>) -> Result<Vec<TrackModel>, Error> {
    let library = &rustic.library;
    library
        .get_tracks()?
        .into_iter()
        .map(|track| TrackModel::new_with_joins(track, &rustic))
        .collect()
}
