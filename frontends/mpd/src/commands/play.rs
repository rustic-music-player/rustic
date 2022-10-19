use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::{Rustic};
use std::sync::Arc;
use futures::future::{BoxFuture, FutureExt};
use rustic_api::ApiClient;

pub struct PlayCommand {}

impl PlayCommand {
    pub fn new() -> PlayCommand {
        PlayCommand {}
    }
}

impl MpdCommand<()> for PlayCommand {
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<(), Error>> {
        async move {
            client.player_control_play(None).await?;

            Ok(())
        }.boxed()
    }
}
