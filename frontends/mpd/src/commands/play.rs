use commands::MpdCommand;
use failure::Error;
use rustic_core::{player::PlayerState, Rustic};
use std::sync::Arc;

pub struct PlayCommand {}

impl PlayCommand {
    pub fn new() -> PlayCommand {
        PlayCommand {}
    }
}

impl MpdCommand<()> for PlayCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<(), Error> {
        let player = app
            .get_default_player()
            .ok_or(format_err!("Missing default player"))?;
        player.backend.set_state(PlayerState::Play)
    }
}
