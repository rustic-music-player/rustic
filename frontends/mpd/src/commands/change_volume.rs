use commands::MpdCommand;
use failure::Error;
use rustic_core::Rustic;
use std::sync::Arc;

pub struct ChangeVolumeCommand {
    pub volume: i32,
}

impl ChangeVolumeCommand {
    pub fn new(volume: i32) -> ChangeVolumeCommand {
        ChangeVolumeCommand { volume }
    }
}

impl MpdCommand<()> for ChangeVolumeCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<(), Error> {
        let player = app
            .get_default_player()
            .ok_or(format_err!("Missing default player"))?;
        let volume = player.backend.volume();
        let volume_percent = volume * 100f32;
        let volume_percent = (volume_percent + self.volume as f32).min(100f32).max(0f32);
        let volume = volume_percent / 100f32;

        player.backend.set_volume(volume)
    }
}
