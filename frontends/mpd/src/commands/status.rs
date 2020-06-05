use commands::MpdCommand;
use failure::Error;
use rustic_core::player::PlayerState;
use rustic_core::Rustic;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct AudioFormat {
    samplerate: i32,
    bits: i32,
    channels: i32,
}

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    volume: u32,
    repeat: bool,
    random: bool,
    single: bool,
    consume: bool,
    playlist: u32,
    playlistlength: usize,
    state: PlayerState,
    //    song: i32,
    //    songid: i32,
    //    nextsong: i32,
    //    nextsongid: i32,
    //    time: i32,
    //    elapsed: i32,
    //    duration: i32,
    //    bitrate: i32,
    xfade: i32,
    //    mixrampdb: i32,
    //    mixrampdelay: i32,
    //    audio: AudioFormat,
    //    updating_db: i32,
    //    error: String
}

pub struct StatusCommand {}

impl StatusCommand {
    pub fn new() -> StatusCommand {
        StatusCommand {}
    }
}

impl MpdCommand<StatusResponse> for StatusCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<StatusResponse, Error> {
        let player = app
            .get_default_player()
            .ok_or(format_err!("Missing default player"))?;
        unimplemented!()
        // Ok(StatusResponse {
        //     volume: (player.backend.volume() * 100f32) as u32,
        //     repeat: false,
        //     random: false,
        //     single: false,
        //     consume: false,
        //     playlist: 0,
        //     playlistlength: player.get_queue().len(),
        //     state: player.backend.state(),
        //     xfade: 0,
        // })
    }
}
