use commands::MpdCommand;
use failure::Error;
use rustic_core::{player::PlayerState, Rustic};
use std::sync::Arc;

pub struct StopCommand {}

impl StopCommand {
    pub fn new() -> StopCommand {
        StopCommand {}
    }
}

impl MpdCommand<()> for StopCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<(), Error> {
        let player = app
            .get_default_player()
            .ok_or(format_err!("Missing default player"))?;
        player.backend.set_state(PlayerState::Stop)
    }
}
