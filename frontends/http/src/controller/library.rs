use actix_web::{error, HttpRequest, Json, Path, Result, State};
use handler::library as library_handler;
use rustic_core::Rustic;
use std::sync::Arc;
use viewmodels::*;

#[derive(Deserialize)]
pub struct GetAlbumQuery {
    album_id: usize,
}

pub fn get_album(req: (State<Arc<Rustic>>, Path<GetAlbumQuery>)) -> Result<Json<AlbumModel>> {
    let (rustic, params) = req;
    let album = library_handler::fetch_album(params.album_id, &rustic)?;
    match album {
        Some(album) => Ok(Json(album)),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

pub fn get_albums(req: &HttpRequest<Arc<Rustic>>) -> Result<Json<Vec<AlbumModel>>> {
    let rustic = req.state();
    let albums = library_handler::fetch_albums(&rustic)?;

    Ok(Json(albums))
}

pub fn get_artists(req: &HttpRequest<Arc<Rustic>>) -> Result<Json<Vec<ArtistModel>>> {
    let rustic = req.state();
    let artists = library_handler::fetch_artists(&rustic)?;

    Ok(Json(artists))
}

pub fn get_playlists(req: &HttpRequest<Arc<Rustic>>) -> Result<Json<Vec<PlaylistModel>>> {
    let rustic = req.state();
    let playlists = library_handler::fetch_playlists(&rustic)?;

    Ok(Json(playlists))
}

pub fn get_tracks(req: &HttpRequest<Arc<Rustic>>) -> Result<Json<Vec<TrackModel>>> {
    let rustic = req.state();
    let tracks = library_handler::fetch_tracks(&rustic)?;

    Ok(Json(tracks))
}
