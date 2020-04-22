use std::sync::Arc;

use failure::{Error, format_err};

use rustic_core::player::PlayerState;
use rustic_core::Rustic;
use rustic_api::models::{PlayerModel, TrackModel};
use rustic_api::cursor::to_cursor;

pub fn get_players(rustic: &Arc<Rustic>) -> Vec<PlayerModel> {
    let players = rustic.get_players();
    players
        .into_iter()
        .map(|(id, player)| {
            let track = player.queue.current().map(TrackModel::from);

            PlayerModel {
                cursor: to_cursor(&id),
                name: player.display_name.clone(),
                playing: (player.backend.state() == PlayerState::Play),
                current: track,
            }
        })
        .collect()
}

pub fn get_state(rustic: &Arc<Rustic>) -> Result<PlayerModel, Error> {
    let player = rustic
        .get_default_player()
        .ok_or_else(|| format_err!("Missing default player"))?;
    let player_id = rustic.get_default_player_id().unwrap();
    let current = player.queue.current().map(TrackModel::from);

    let state = PlayerModel {
        cursor: to_cursor(&player_id),
        name: player.display_name.clone(),
        playing: (player.backend.state() == PlayerState::Play),
        current,
    };

    Ok(state)
}

pub fn control_next(rustic: &Arc<Rustic>, player_id: Option<String>) -> Result<(), Error> {
    let player = player_id
        .and_then(|id| rustic.get_player(id))
        .or_else(|| rustic.get_default_player())
        .ok_or_else(|| format_err!("Missing player"))?;
    player.queue.next()?;

    Ok(())
}

pub fn control_prev(rustic: &Arc<Rustic>, player_id: Option<String>) -> Result<(), Error> {
    let player = player_id
        .and_then(|id| rustic.get_player(id))
        .or_else(|| rustic.get_default_player())
        .ok_or_else(|| format_err!("Missing player"))?;
    player.queue.prev()?;

    Ok(())
}

pub fn control_pause(rustic: &Arc<Rustic>, player_id: Option<String>) -> Result<(), Error> {
    let player = player_id
        .and_then(|id| rustic.get_player(id))
        .or_else(|| rustic.get_default_player())
        .ok_or_else(|| format_err!("Missing player"))?;
    player.backend.set_state(PlayerState::Pause)?;

    Ok(())
}

pub fn control_play(rustic: &Arc<Rustic>, player_id: Option<String>) -> Result<(), Error> {
    let player = player_id
        .and_then(|id| rustic.get_player(id))
        .or_else(|| rustic.get_default_player())
        .ok_or_else(|| format_err!("Missing player"))?;
    player.backend.set_state(PlayerState::Play)?;

    Ok(())
}
