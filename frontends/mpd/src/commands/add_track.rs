use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::{Rustic};
use std::sync::Arc;
use futures::future::BoxFuture;
use rustic_api::ApiClient;
use futures::FutureExt;
use crate::client_ext::ClientExt;

pub struct AddTrackCommand {
    cursor: String,
}

impl AddTrackCommand {
    pub fn new(cursor: String) -> AddTrackCommand {
        AddTrackCommand { cursor }
    }
}

impl MpdCommand<()> for AddTrackCommand {
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<(), Error>> {
        async move {
            client.queue_track(None, &self.cursor).await?;

            Ok(())
        }.boxed()
    }
}
