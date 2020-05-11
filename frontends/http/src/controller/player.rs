use actix_web::{get, post, web, HttpResponse, Responder, Result};
use serde::{Deserialize};
use crate::app::ApiState;
use crate::cursor::from_cursor;

#[derive(Deserialize)]
pub struct PlayerQuery {
    player: String,
}

#[get("/players")]
pub async fn get_players(data: web::Data<ApiState>) -> Result<impl Responder> {
    let players = data.client.get_players().await?;

    Ok(web::Json(players))
}

#[get("/player")]
pub async fn default_player_state(data: web::Data<ApiState>) -> Result<impl Responder> {
    let player = data.client.get_player(None).await?;

    match player {
        Some(player) => Ok(HttpResponse::Ok().json(player)),
        None => Ok(HttpResponse::NotFound().finish())
    }
}

#[post("/player/next")]
pub async fn default_control_next(data: web::Data<ApiState>) -> Result<impl Responder> {
    data.client.player_control_next(None).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/next")]
pub async fn control_next(
    data: web::Data<ApiState>,
    params: web::Path<PlayerQuery>,
) -> Result<impl Responder> {
    let player_id = from_cursor(&params.player)?;
    data.client.player_control_next(Some(&player_id)).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/player/prev")]
pub async fn default_control_prev(data: web::Data<ApiState>) -> Result<impl Responder> {
    data.client.player_control_prev(None).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/prev")]
pub async fn control_prev(
    data: web::Data<ApiState>,
    params: web::Path<PlayerQuery>,
) -> Result<impl Responder> {
    let player_id = from_cursor(&params.player)?;
    data.client.player_control_prev(Some(&player_id)).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/player/pause")]
pub async fn default_control_pause(data: web::Data<ApiState>) -> Result<impl Responder> {
    data.client.player_control_pause(None).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/pause")]
pub async fn control_pause(
    data: web::Data<ApiState>,
    params: web::Path<PlayerQuery>,
) -> Result<impl Responder> {
    let player_id = from_cursor(&params.player)?;
    data.client.player_control_pause(Some(&player_id)).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/player/play")]
pub async fn default_control_play(data: web::Data<ApiState>) -> Result<impl Responder> {
    data.client.player_control_play(None).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/play")]
pub async fn control_play(
    data: web::Data<ApiState>,
    params: web::Path<PlayerQuery>,
) -> Result<impl Responder> {
    let player_id = from_cursor(&params.player)?;
    data.client.player_control_play(Some(&player_id)).await?;

    Ok(HttpResponse::NoContent().finish())
}
