use actix_web::{get, HttpResponse, post, Responder, Result, web};
use serde::Deserialize;

use rustic_api::cursor::from_cursor;
use rustic_api::models::RepeatModeModel;

use crate::app::ApiClient;

#[derive(Deserialize)]
pub struct PlayerQuery {
    player: String,
}

#[get("/players")]
pub async fn get_players(client: web::Data<ApiClient>) -> Result<impl Responder> {
    let players = client.get_players().await?;

    Ok(web::Json(players))
}

#[get("/player")]
pub async fn default_player_state(client: web::Data<ApiClient>) -> Result<impl Responder> {
    let player = client.get_player(None).await?;

    match player {
        Some(player) => Ok(HttpResponse::Ok().json(player)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[post("/player/next")]
pub async fn default_control_next(client: web::Data<ApiClient>) -> Result<impl Responder> {
    client.player_control_next(None).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/next")]
pub async fn control_next(
    client: web::Data<ApiClient>,
    params: web::Path<PlayerQuery>,
) -> Result<impl Responder> {
    let player_id = from_cursor(&params.player)?;
    client.player_control_next(Some(&player_id)).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/player/prev")]
pub async fn default_control_prev(client: web::Data<ApiClient>) -> Result<impl Responder> {
    client.player_control_prev(None).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/prev")]
pub async fn control_prev(
    client: web::Data<ApiClient>,
    params: web::Path<PlayerQuery>,
) -> Result<impl Responder> {
    let player_id = from_cursor(&params.player)?;
    client.player_control_prev(Some(&player_id)).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/player/pause")]
pub async fn default_control_pause(client: web::Data<ApiClient>) -> Result<impl Responder> {
    client.player_control_pause(None).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/pause")]
pub async fn control_pause(
    client: web::Data<ApiClient>,
    params: web::Path<PlayerQuery>,
) -> Result<impl Responder> {
    let player_id = from_cursor(&params.player)?;
    client.player_control_pause(Some(&player_id)).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/player/play")]
pub async fn default_control_play(client: web::Data<ApiClient>) -> Result<impl Responder> {
    client.player_control_play(None).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/play")]
pub async fn control_play(
    client: web::Data<ApiClient>,
    params: web::Path<PlayerQuery>,
) -> Result<impl Responder> {
    let player_id = from_cursor(&params.player)?;
    client.player_control_play(Some(&player_id)).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/player/volume")]
pub async fn default_set_volume(
    client: web::Data<ApiClient>,
    volume: web::Json<f32>,
) -> Result<impl Responder> {
    client.player_set_volume(None, volume.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/volume")]
pub async fn set_volume(
    client: web::Data<ApiClient>,
    params: web::Path<PlayerQuery>,
    volume: web::Json<f32>,
) -> Result<impl Responder> {
    let player_id = from_cursor(&params.player)?;
    client
        .player_set_volume(Some(&player_id), volume.into_inner())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/player/repeat")]
pub async fn default_set_repeat(client: web::Data<ApiClient>, repeat: web::Json<RepeatModeModel>) -> Result<impl Responder> {
    client.player_set_repeat(None, repeat.into_inner())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/repeat")]
pub async fn set_repeat(client: web::Data<ApiClient>, params: web::Path<PlayerQuery>, repeat: web::Json<RepeatModeModel>) -> Result<impl Responder> {
    let player_id = from_cursor(&params.player)?;
    client.player_set_repeat(Some(&player_id), repeat.into_inner())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}
