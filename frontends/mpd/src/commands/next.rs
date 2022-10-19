use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::Rustic;
use std::sync::Arc;
use futures::future::BoxFuture;
use rustic_api::ApiClient;
use crate::FutureExt;

pub struct NextCommand {}

impl NextCommand {
    pub fn new() -> NextCommand {
        NextCommand {}
    }
}

impl MpdCommand<()> for NextCommand {
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<(), Error>> {
        async move {
            client.player_control_next(None).await?;

            Ok(())
        }.boxed()
    }
}
