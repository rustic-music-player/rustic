use actix_web::{delete, error, get, HttpResponse, post, put, Responder, Result, web};
use serde::Deserialize;

use rustic_api::cursor::from_cursor;

use crate::app::ApiClient;

#[derive(Deserialize)]
pub struct PlayerParams {
    player_cursor: String,
}

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
pub async fn fetch_default(client: web::Data<ApiClient>) -> Result<impl Responder> {
    let tracks = client.get_queue(None).await?;

    Ok(web::Json(tracks))
}

#[get("/queue/{player_cursor}")]
pub async fn fetch(client: web::Data<ApiClient>, params: web::Path<PlayerParams>) -> Result<impl Responder> {
    let player_id = from_cursor(&params.player_cursor)?;
    let tracks = client.get_queue(Some(&player_id)).await?;

    Ok(web::Json(tracks))
}

#[post("/queue/track/{cursor}")]
pub async fn queue_track_default(
    client: web::Data<ApiClient>,
    params: web::Path<AddToQueueQuery>,
) -> Result<impl Responder> {
    let result = client.queue_track(None, &params.cursor).await?;

    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/queue/{player_cursor}/track/{cursor}")]
pub async fn queue_track(
    client: web::Data<ApiClient>,
    params: web::Path<AddToQueueQuery>,
    player: web::Path<PlayerParams>,
) -> Result<impl Responder> {
    let player_id = from_cursor(&player.player_cursor)?;
    let result = client.queue_track(Some(&player_id), &params.cursor).await?;

    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/queue/album/{cursor}")]
pub async fn queue_album_default(
    client: web::Data<ApiClient>,
    params: web::Path<AddToQueueQuery>,
) -> Result<impl Responder> {
    let result = client.queue_album(None, &params.cursor).await?;

    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/queue/{player_cursor}/album/{cursor}")]
pub async fn queue_album(
    client: web::Data<ApiClient>,
    params: web::Path<AddToQueueQuery>,
    player: web::Path<PlayerParams>,
) -> Result<impl Responder> {
    let player_id = from_cursor(&player.player_cursor)?;
    let result = client.queue_album(Some(&player_id), &params.cursor).await?;

    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/queue/playlist/{cursor}")]
pub async fn queue_playlist_default(
    client: web::Data<ApiClient>,
    params: web::Path<AddToQueueQuery>,
) -> Result<impl Responder> {
    let result = client.queue_playlist(None, &params.cursor).await?;

    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/queue/{player_cursor}/playlist/{cursor}")]
pub async fn queue_playlist(
    client: web::Data<ApiClient>,
    params: web::Path<AddToQueueQuery>,
    player: web::Path<PlayerParams>
) -> Result<impl Responder> {
    let player_id = from_cursor(&player.player_cursor)?;
    let result = client.queue_playlist(Some(&player_id), &params.cursor).await?;

    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/queue/clear")]
pub async fn clear_default(client: web::Data<ApiClient>) -> Result<impl Responder> {
    client.clear_queue(None).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/queue/{player_cursor}/clear")]
pub async fn clear(client: web::Data<ApiClient>, player: web::Path<PlayerParams>) -> Result<impl Responder> {
    let player_id = from_cursor(&player.player_cursor)?;
    client.clear_queue(Some(&player_id)).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[put("/queue/select/{index}")]
pub async fn select_item_default(
    client: web::Data<ApiClient>,
    params: web::Path<QueueItemParams>,
) -> Result<impl Responder> {
    client.select_queue_item(None, params.index).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[put("/queue/{player_cursor}/select/{index}")]
pub async fn select_item(
    client: web::Data<ApiClient>,
    params: web::Path<QueueItemParams>,
    player: web::Path<PlayerParams>,
) -> Result<impl Responder> {
    let player_id = from_cursor(&player.player_cursor)?;
    client.select_queue_item(Some(&player_id), params.index).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[delete("/queue/{index}")]
pub async fn remove_item_default(
    client: web::Data<ApiClient>,
    params: web::Path<QueueItemParams>,
) -> Result<impl Responder> {
    client.remove_queue_item(None, params.index).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[delete("/queue/{player_cursor}/{index}")]
pub async fn remove_item(
    client: web::Data<ApiClient>,
    params: web::Path<QueueItemParams>,
    player: web::Path<PlayerParams>,
) -> Result<impl Responder> {
    let player_id = from_cursor(&player.player_cursor)?;
    client.remove_queue_item(Some(&player_id), params.index).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/queue/reorder/{before}/{after}")]
pub async fn reorder_item_default(
    client: web::Data<ApiClient>,
    params: web::Path<ReorderQueueItemParams>,
) -> Result<impl Responder> {
    client
        .reorder_queue_item(None, params.before, params.after)
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/queue/{player_cursor}/reorder/{before}/{after}")]
pub async fn reorder_item(
    client: web::Data<ApiClient>,
    params: web::Path<ReorderQueueItemParams>,
    player: web::Path<PlayerParams>,
) -> Result<impl Responder> {
    let player_id = from_cursor(&player.player_cursor)?;
    client
        .reorder_queue_item(Some(&player_id), params.before, params.after)
        .await?;

    Ok(HttpResponse::NoContent().finish())
}
