use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::Rustic;
use std::sync::Arc;
use futures::future::{BoxFuture, FutureExt};
use rustic_api::ApiClient;

pub struct PreviousCommand {}

impl PreviousCommand {
    pub fn new() -> PreviousCommand {
        PreviousCommand {}
    }
}

impl MpdCommand<()> for PreviousCommand {
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<(), Error>> {
        async move {
            client.player_control_prev(None).await?;

            Ok(())
        }.boxed()
    }
}
