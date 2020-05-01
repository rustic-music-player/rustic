use actix_web::{get, post, web, HttpResponse, Responder, Result};
use serde::{Deserialize};
use crate::app::ApiState;
use crate::cursor::from_cursor;
use crate::handler::player as player_handler;

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
    let rustic = &data.app;
    let state = player_handler::get_state(&rustic)?;

    Ok(web::Json(state))
}

#[post("/player/next")]
pub async fn default_control_next(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    player_handler::control_next(&rustic, None)?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/next")]
pub async fn control_next(
    data: web::Data<ApiState>,
    params: web::Path<PlayerQuery>,
) -> Result<impl Responder> {
    let rustic = &data.app;
    let player_id = from_cursor(&params.player)?;
    player_handler::control_next(&rustic, Some(player_id))?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/player/prev")]
pub async fn default_control_prev(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    player_handler::control_prev(&rustic, None)?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/prev")]
pub async fn control_prev(
    data: web::Data<ApiState>,
    params: web::Path<PlayerQuery>,
) -> Result<impl Responder> {
    let rustic = &data.app;
    let player_id = from_cursor(&params.player)?;
    player_handler::control_prev(&rustic, Some(player_id))?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/player/pause")]
pub async fn default_control_pause(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    player_handler::control_pause(&rustic, None)?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/pause")]
pub async fn control_pause(
    data: web::Data<ApiState>,
    params: web::Path<PlayerQuery>,
) -> Result<impl Responder> {
    let rustic = &data.app;
    let player_id = from_cursor(&params.player)?;
    player_handler::control_pause(&rustic, Some(player_id))?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/player/play")]
pub async fn default_control_play(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    player_handler::control_play(&rustic, None)?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/play")]
pub async fn control_play(
    data: web::Data<ApiState>,
    params: web::Path<PlayerQuery>,
) -> Result<impl Responder> {
    let rustic = &data.app;
    let player_id = from_cursor(&params.player)?;
    player_handler::control_play(&rustic, Some(player_id))?;

    Ok(HttpResponse::NoContent().finish())
}
