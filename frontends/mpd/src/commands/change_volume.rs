use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::Rustic;
use rustic_api::ApiClient;
use std::sync::Arc;
use futures::future::{BoxFuture, FutureExt};

pub struct ChangeVolumeCommand {
    pub volume: i32,
}

impl ChangeVolumeCommand {
    pub fn new(volume: i32) -> ChangeVolumeCommand {
        ChangeVolumeCommand { volume }
    }
}

impl MpdCommand<()> for ChangeVolumeCommand {
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<(), Error>> {
        async move {
            let player = client.get_player(None).await?;
            let player = player.ok_or(format_err!("Missing default player"))?;
            let volume = player.volume;
            let volume_percent = volume * 100f32;
            let volume_percent = (volume_percent + self.volume as f32).min(100f32).max(0f32);
            let volume = volume_percent / 100f32;

            client.player_set_volume(None, volume).await?;

            Ok(())
        }.boxed()
    }
}
