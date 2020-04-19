use actix_web::{get, post, web, HttpResponse, Responder, Result};
use app::ApiState;
use handler::player as player_handler;
use cursor::from_cursor;

#[derive(Deserialize)]
pub struct PlayerQuery {
    player: String,
}

#[get("/players")]
pub fn get_players(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    let players = player_handler::get_players(&rustic);

    Ok(web::Json(players))
}

#[get("/player")]
pub fn default_player_state(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    let state = player_handler::get_state(&rustic)?;

    Ok(web::Json(state))
}

#[post("/player/next")]
pub fn default_control_next(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    player_handler::control_next(&rustic, None)?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/next")]
pub fn control_next(data: web::Data<ApiState>,
                    params: web::Path<PlayerQuery>,
) -> Result<impl Responder> {
    let rustic = &data.app;
    let player_id = from_cursor(&params.player)?;
    player_handler::control_next(&rustic, Some(player_id))?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/player/prev")]
pub fn default_control_prev(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    player_handler::control_prev(&rustic, None)?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/prev")]
pub fn control_prev(data: web::Data<ApiState>,
                    params: web::Path<PlayerQuery>,
) -> Result<impl Responder> {
    let rustic = &data.app;
    let player_id = from_cursor(&params.player)?;
    player_handler::control_prev(&rustic, Some(player_id))?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/player/pause")]
pub fn default_control_pause(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    player_handler::control_pause(&rustic, None)?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/pause")]
pub fn control_pause(data: web::Data<ApiState>,
                    params: web::Path<PlayerQuery>,
) -> Result<impl Responder> {
    let rustic = &data.app;
    let player_id = from_cursor(&params.player)?;
    player_handler::control_pause(&rustic, Some(player_id))?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/player/play")]
pub fn default_control_play(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    player_handler::control_play(&rustic, None)?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/players/{player}/play")]
pub fn control_play(data: web::Data<ApiState>,
                    params: web::Path<PlayerQuery>,
) -> Result<impl Responder> {
    let rustic = &data.app;
    let player_id = from_cursor(&params.player)?;
    player_handler::control_play(&rustic, Some(player_id))?;

    Ok(HttpResponse::NoContent().finish())
}
