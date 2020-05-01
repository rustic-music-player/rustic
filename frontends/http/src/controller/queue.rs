use actix_web::{delete, error, get, post, web, HttpResponse, Responder, Result};
use serde::{Deserialize};

use crate::app::ApiState;

#[derive(Deserialize)]
pub struct AddToQueueQuery {
    cursor: String,
}

#[derive(Deserialize)]
pub struct QueueItemParams {
    index: usize,
}

#[get("/queue")]
pub async fn fetch(data: web::Data<ApiState>) -> Result<impl Responder> {
    let tracks = data.client.get_queue(None).await?;

    Ok(web::Json(tracks))
}

#[post("/queue/track/{cursor}")]
pub async fn queue_track(
    data: web::Data<ApiState>,
    params: web::Path<AddToQueueQuery>,
) -> Result<impl Responder> {
    let result = data.client.queue_track(None, &params.cursor).await?;

    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/queue/album/{cursor}")]
pub async fn queue_album(
    data: web::Data<ApiState>,
    params: web::Path<AddToQueueQuery>,
) -> Result<impl Responder> {
    let result = data.client.queue_album(None, &params.cursor).await?;

    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/queue/playlist/{cursor}")]
pub async fn queue_playlist(
    data: web::Data<ApiState>,
    params: web::Path<AddToQueueQuery>,
) -> Result<impl Responder> {
    let result = data.client.queue_playlist(None, &params.cursor).await?;

    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/queue/clear")]
pub async fn clear(data: web::Data<ApiState>) -> Result<impl Responder> {
    data.client.clear_queue(None).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[delete("/queue/{index}")]
pub async fn remove_item(
    params: web::Path<QueueItemParams>,
    data: web::Data<ApiState>,
) -> Result<impl Responder> {
    data.client.remove_queue_item(None, params.index).await?;

    Ok(HttpResponse::NoContent().finish())
}
