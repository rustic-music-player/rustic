use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::{Rustic};
use std::sync::Arc;
use futures::future::{BoxFuture, FutureExt};
use rustic_api::ApiClient;

pub struct TogglePauseCommand {}

impl TogglePauseCommand {
    pub fn new() -> TogglePauseCommand {
        TogglePauseCommand {}
    }
}

impl MpdCommand<()> for TogglePauseCommand {
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<(), Error>> {
        async move {
            let player = client.get_player(None).await?
                .ok_or(failure::format_err!("Missing default player"))?;
            if player.playing {
                client.player_control_pause(None).await?;
            }else {
                client.player_control_play(None).await?;
            }

            Ok(())
        }.boxed()
    }
}
