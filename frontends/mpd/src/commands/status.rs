use serde::Serialize;
use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::player::PlayerState;
use rustic_core::Rustic;
use std::sync::Arc;
use futures::future::{BoxFuture, FutureExt};
use rustic_api::ApiClient;
use rustic_api::models::RepeatModeModel;

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
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<StatusResponse, Error>> {
        async move {
            let status = client.get_player(None).await?
                .ok_or(failure::format_err!("Missing default player"))?;
            let queue = client.get_queue(None).await?;

            Ok(StatusResponse {
                volume: (status.volume * 100f32) as u32,
                repeat: status.repeat == RepeatModeModel::All,
                single: status.repeat == RepeatModeModel::Single,
                random: false,
                consume: false,
                playlist: 0,
                playlistlength: queue.len(),
                state: if status.playing {
                    PlayerState::Play
                }else {
                    PlayerState::Pause
                },
                xfade: 0,
            })
        }.boxed()
    }
}
