use commands::MpdCommand;
use failure::Error;
use rustic_core::Rustic;
use std::sync::Arc;

pub struct SetVolumeCommand {
    pub volume: u32,
}

impl SetVolumeCommand {
    pub fn new(volume: u32) -> SetVolumeCommand {
        SetVolumeCommand { volume }
    }
}

impl MpdCommand<()> for SetVolumeCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<(), Error> {
        let player = app
            .get_default_player()
            .ok_or(format_err!("Missing default player"))?;
        let volume = (self.volume as f32) / 100f32;

        player.backend.set_volume(volume)
    }
}
