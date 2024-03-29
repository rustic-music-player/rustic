use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::{Rustic};
use std::sync::Arc;
use futures::future::BoxFuture;
use rustic_api::ApiClient;
use futures::FutureExt;
use crate::client_ext::ClientExt;

pub struct LoadPlaylistCommand {
    name: String,
}

impl LoadPlaylistCommand {
    pub fn new(name: String) -> LoadPlaylistCommand {
        LoadPlaylistCommand { name }
    }
}

impl MpdCommand<()> for LoadPlaylistCommand {
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<(), Error>> {
        async move {
            let playlist = client.get_playlist_by_name(&self.name).await?.unwrap();

            client.queue_playlist(None, &playlist.cursor).await?;

            Ok(())
        }.boxed()
    }
}
