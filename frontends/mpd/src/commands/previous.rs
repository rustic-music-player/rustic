use commands::MpdCommand;
use failure::Error;
use rustic_core::Rustic;
use std::sync::Arc;

pub struct PreviousCommand {}

impl PreviousCommand {
    pub fn new() -> PreviousCommand {
        PreviousCommand {}
    }
}

impl MpdCommand<()> for PreviousCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<(), Error> {
        let player = app
            .get_default_player()
            .ok_or(format_err!("Missing default player"))?;
        player.prev().map(|_| ())
    }
}
