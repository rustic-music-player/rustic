use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::{Rustic};
use std::sync::Arc;
use futures::future::{BoxFuture, FutureExt};
use rustic_api::ApiClient;

pub struct ClearQueueCommand;

impl ClearQueueCommand {
    pub fn new() -> Self {
        Self
    }
}

impl MpdCommand<()> for ClearQueueCommand {
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<(), Error>> {
        async move {
            client.clear_queue(None).await?;

            Ok(())
        }.boxed()
    }
}
