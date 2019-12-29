use std::sync::Arc;

use failure::Error;

use cursor::to_cursor;
use rustic_core::player::PlayerState;
use rustic_core::Rustic;
use viewmodels::{PlayerModel, TrackModel};

pub fn get_state(rustic: &Arc<Rustic>) -> Result<PlayerModel, Error> {
    let player = rustic
        .get_default_player()
        .ok_or_else(|| format_err!("Missing default player"))?;
    let player_id = rustic.get_default_player_id().unwrap();
    let current = match player.current() {
        Some(track) => Some(TrackModel::new(track, &rustic)),
        None => None,
    };

    let state = PlayerModel {
        cursor: to_cursor(&player_id),
        playing: (player.state() == PlayerState::Play),
        current,
    };

    Ok(state)
}

pub fn control_next(rustic: &Arc<Rustic>) -> Result<(), Error> {
    let player = rustic
        .get_default_player()
        .ok_or_else(|| format_err!("Missing default player"))?;
    player.next()?;

    Ok(())
}

pub fn control_prev(rustic: &Arc<Rustic>) -> Result<(), Error> {
    let player = rustic
        .get_default_player()
        .ok_or_else(|| format_err!("Missing default player"))?;
    player.prev()?;

    Ok(())
}

pub fn control_pause(rustic: &Arc<Rustic>) -> Result<(), Error> {
    let player = rustic
        .get_default_player()
        .ok_or_else(|| format_err!("Missing default player"))?;
    player.set_state(PlayerState::Pause)?;

    Ok(())
}

pub fn control_play(rustic: &Arc<Rustic>) -> Result<(), Error> {
    let player = rustic
        .get_default_player()
        .ok_or_else(|| format_err!("Missing default player"))?;
    player.set_state(PlayerState::Play)?;

    Ok(())
}
