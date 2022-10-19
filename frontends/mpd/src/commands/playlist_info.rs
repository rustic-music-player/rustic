use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::{Rustic};
use crate::song::MpdSong;
use std::sync::Arc;
use futures::future::BoxFuture;
use rustic_api::ApiClient;
use crate::FutureExt;

pub struct PlaylistInfoCommand;

impl PlaylistInfoCommand {
    pub fn new() -> PlaylistInfoCommand {
        PlaylistInfoCommand
    }
}

impl MpdCommand<Vec<MpdSong>> for PlaylistInfoCommand {
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<Vec<MpdSong>, Error>> {
        async move {
            let queue = client.get_queue(None).await?;

            let tracks = queue
                .into_iter()
                .map(|track| track.track)
                .map(MpdSong::from)
                .collect();

            Ok(tracks)
        }.boxed()
    }
}
