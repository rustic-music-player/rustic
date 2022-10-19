use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::Rustic;
use std::sync::Arc;
use futures::future::{BoxFuture, FutureExt};
use rustic_api::ApiClient;

pub struct SetVolumeCommand {
    pub volume: u32,
}

impl SetVolumeCommand {
    pub fn new(volume: u32) -> SetVolumeCommand {
        SetVolumeCommand { volume }
    }
}

impl MpdCommand<()> for SetVolumeCommand {
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<(), Error>> {
        async move {
            let volume = (self.volume as f32) / 100f32;

            client.player_set_volume(None, volume).await?;

            Ok(())
        }.boxed()
    }
}
