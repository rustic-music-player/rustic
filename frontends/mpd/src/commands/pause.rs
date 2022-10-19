use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::{Rustic};
use std::sync::Arc;
use futures::future::{BoxFuture, FutureExt};
use rustic_api::ApiClient;

pub struct PauseCommand {}

impl PauseCommand {
    pub fn new() -> PauseCommand {
        PauseCommand {}
    }
}

impl MpdCommand<()> for PauseCommand {
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<(), Error>> {
        async move {
            client.player_control_pause(None).await?;

            Ok(())
        }.boxed()
    }
}
