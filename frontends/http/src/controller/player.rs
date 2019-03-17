use actix_web::{HttpRequest, Json, Result};
use handler::player as player_handler;
use rustic_core::Rustic;
use std::sync::Arc;
use viewmodels::PlayerModel;

pub fn player_state(req: &HttpRequest<Arc<Rustic>>) -> Result<Json<PlayerModel>> {
    let rustic = req.state();
    let state = player_handler::get_state(&rustic)?;

    Ok(Json(state))
}

pub fn control_next(req: &HttpRequest<Arc<Rustic>>) -> Result<Json<()>> {
    let rustic = req.state();
    player_handler::control_next(&rustic)?;

    Ok(Json(()))
}

pub fn control_prev(req: &HttpRequest<Arc<Rustic>>) -> Result<Json<()>> {
    let rustic = req.state();
    player_handler::control_prev(&rustic)?;

    Ok(Json(()))
}

pub fn control_pause(req: &HttpRequest<Arc<Rustic>>) -> Result<Json<()>> {
    let rustic = req.state();
    player_handler::control_pause(&rustic)?;

    Ok(Json(()))
}

pub fn control_play(req: &HttpRequest<Arc<Rustic>>) -> Result<Json<()>> {
    let rustic = req.state();
    player_handler::control_play(&rustic)?;

    Ok(Json(()))
}
