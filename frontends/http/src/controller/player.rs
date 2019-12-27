use actix_web::{get, post, web, HttpResponse, Responder, Result};
use app::ApiState;
use handler::player as player_handler;

#[get("/player")]
pub fn player_state(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    let state = player_handler::get_state(&rustic)?;

    Ok(web::Json(state))
}

#[post("/player/next")]
pub fn control_next(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    player_handler::control_next(&rustic)?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/player/prev")]
pub fn control_prev(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    player_handler::control_prev(&rustic)?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/player/pause")]
pub fn control_pause(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    player_handler::control_pause(&rustic)?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/player/play")]
pub fn control_play(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    player_handler::control_play(&rustic)?;

    Ok(HttpResponse::NoContent().finish())
}
