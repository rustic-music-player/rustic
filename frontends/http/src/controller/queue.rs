use actix_web::{delete, error, get, post, web, HttpResponse, Responder, Result};
use serde::Deserialize;

use crate::app::ApiClient;

#[derive(Deserialize)]
pub struct AddToQueueQuery {
    cursor: String,
}

#[derive(Deserialize)]
pub struct QueueItemParams {
    index: usize,
}

#[derive(Deserialize)]
pub struct ReorderQueueItemParams {
    before: usize,
    after: usize,
}

#[get("/queue")]
pub async fn fetch(client: web::Data<ApiClient>) -> Result<impl Responder> {
    let tracks = client.get_queue(None).await?;

    Ok(web::Json(tracks))
}

#[post("/queue/track/{cursor}")]
pub async fn queue_track(
    client: web::Data<ApiClient>,
    params: web::Path<AddToQueueQuery>,
) -> Result<impl Responder> {
    let result = client.queue_track(None, &params.cursor).await?;

    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/queue/album/{cursor}")]
pub async fn queue_album(
    client: web::Data<ApiClient>,
    params: web::Path<AddToQueueQuery>,
) -> Result<impl Responder> {
    let result = client.queue_album(None, &params.cursor).await?;

    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/queue/playlist/{cursor}")]
pub async fn queue_playlist(
    client: web::Data<ApiClient>,
    params: web::Path<AddToQueueQuery>,
) -> Result<impl Responder> {
    let result = client.queue_playlist(None, &params.cursor).await?;

    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/queue/clear")]
pub async fn clear(client: web::Data<ApiClient>) -> Result<impl Responder> {
    client.clear_queue(None).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[delete("/queue/{index}")]
pub async fn remove_item(
    client: web::Data<ApiClient>,
    params: web::Path<QueueItemParams>,
) -> Result<impl Responder> {
    client.remove_queue_item(None, params.index).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/queue/reorder/{before}/{after}")]
pub async fn reorder_item(
    client: web::Data<ApiClient>,
    params: web::Path<ReorderQueueItemParams>,
) -> Result<impl Responder> {
    client
        .reorder_queue_item(None, params.before, params.after)
        .await?;

    Ok(HttpResponse::NoContent().finish())
}
