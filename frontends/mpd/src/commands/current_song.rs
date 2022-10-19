use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::Rustic;
use crate::song::MpdSong;
use std::sync::Arc;
use futures::future::BoxFuture;
use rustic_api::ApiClient;
use crate::FutureExt;

pub struct CurrentSongCommand {}

impl CurrentSongCommand {
    pub fn new() -> CurrentSongCommand {
        CurrentSongCommand {}
    }
}

impl MpdCommand<Option<MpdSong>> for CurrentSongCommand {
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<Option<MpdSong>, Error>> {
        async move {
            let player = client.get_player(None).await?
                .ok_or(format_err!("Missing default player"))?;

            Ok(player.current.map(MpdSong::from))
        }.boxed()
    }
}
