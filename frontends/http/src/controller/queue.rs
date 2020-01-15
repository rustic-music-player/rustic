use actix_web::{error, get, post, web, HttpResponse, Responder, Result};

use app::ApiState;
use handler::queue as queue_handler;

#[derive(Deserialize)]
pub struct AddToQueueQuery {
    cursor: String,
}

#[get("/queue")]
pub fn fetch(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    let tracks = queue_handler::fetch(&rustic)?;

    Ok(web::Json(tracks))
}

#[post("/queue/track/{cursor}")]
pub fn queue_track(
    data: web::Data<ApiState>,
    params: web::Path<AddToQueueQuery>,
) -> Result<impl Responder> {
    let rustic = &data.app;
    let result = queue_handler::queue_track(&params.cursor, &rustic)?;
    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/queue/album/{cursor}")]
pub fn queue_album(
    data: web::Data<ApiState>,
    params: web::Path<AddToQueueQuery>,
) -> Result<impl Responder> {
    let rustic = &data.app;
    let result = queue_handler::queue_album(&params.cursor, &rustic)?;
    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/queue/playlist/{cursor}")]
pub fn queue_playlist(
    data: web::Data<ApiState>,
    params: web::Path<AddToQueueQuery>,
) -> Result<impl Responder> {
    let rustic = &data.app;
    let result = queue_handler::queue_playlist(&params.cursor, &rustic)?;
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
