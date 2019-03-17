use actix_web::{error, HttpRequest, Json, Path, Result, State};
use handler::queue as queue_handler;
use rustic_core::Rustic;
use std::sync::Arc;
use viewmodels::TrackModel;

#[derive(Deserialize)]
pub struct QueueTrackQuery {
    track_id: usize,
}

#[derive(Deserialize)]
pub struct QueuePlaylistQuery {
    playlist_id: usize,
}

pub fn fetch(req: &HttpRequest<Arc<Rustic>>) -> Result<Json<Vec<TrackModel>>> {
    let rustic = req.state();
    let tracks = queue_handler::fetch(&rustic)?;

    Ok(Json(tracks))
}

pub fn queue_track(req: (State<Arc<Rustic>>, Path<QueueTrackQuery>)) -> Result<Json<()>> {
    let (rustic, params) = req;
    let result = queue_handler::queue_track(params.track_id, &rustic)?;
    match result {
        Some(_) => Ok(Json(())),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

pub fn queue_playlist(req: (State<Arc<Rustic>>, Path<QueuePlaylistQuery>)) -> Result<Json<()>> {
    let (rustic, params) = req;
    let result = queue_handler::queue_playlist(params.playlist_id, &rustic)?;
    match result {
        Some(_) => Ok(Json(())),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

pub fn clear(req: &HttpRequest<Arc<Rustic>>) -> Result<Json<()>> {
    let rustic = req.state();
    queue_handler::clear(&rustic)?;
    Ok(Json(()))
}
