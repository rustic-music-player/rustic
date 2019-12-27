use actix_web::{error, get, post, web, HttpResponse, Responder, Result};
use app::ApiState;
use handler::queue as queue_handler;

#[derive(Deserialize)]
pub struct QueueTrackQuery {
    track_id: usize,
}

#[derive(Deserialize)]
pub struct QueuePlaylistQuery {
    playlist_id: usize,
}

#[get("/queue")]
pub fn fetch(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    let tracks = queue_handler::fetch(&rustic)?;

    Ok(web::Json(tracks))
}

#[post("/queue/track/{track_id}")]
pub fn queue_track(
    data: web::Data<ApiState>,
    params: web::Path<QueueTrackQuery>,
) -> Result<impl Responder> {
    let rustic = &data.app;
    let result = queue_handler::queue_track(params.track_id, &rustic)?;
    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/queue/playlist/{playlist_id}")]
pub fn queue_playlist(
    data: web::Data<ApiState>,
    params: web::Path<QueuePlaylistQuery>,
) -> Result<impl Responder> {
    let rustic = &data.app;
    let result = queue_handler::queue_playlist(params.playlist_id, &rustic)?;
    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/queue/clear")]
pub fn clear(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    queue_handler::clear(&rustic)?;
    Ok(HttpResponse::NoContent().finish())
}
