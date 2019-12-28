use commands::MpdCommand;
use failure::Error;
use rustic_core::Rustic;
use song::MpdSong;
use std::sync::Arc;

pub struct CurrentSongCommand {}

impl CurrentSongCommand {
    pub fn new() -> CurrentSongCommand {
        CurrentSongCommand {}
    }
}

impl MpdCommand<Option<MpdSong>> for CurrentSongCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<Option<MpdSong>, Error> {
        let player = app
            .get_default_player()
            .ok_or(format_err!("Missing default player"))?;
        let track = player.current().map(MpdSong::from);
        Ok(track)
    }
}
