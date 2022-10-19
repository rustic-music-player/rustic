use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::{Rustic};
use crate::song::MpdSong;
use std::sync::Arc;
use futures::future::BoxFuture;
use rustic_api::ApiClient;
use futures::FutureExt;
use crate::client_ext::ClientExt;

pub struct ListPlaylistInfoCommand {
    name: String,
}

impl ListPlaylistInfoCommand {
    pub fn new(name: String) -> ListPlaylistInfoCommand {
        ListPlaylistInfoCommand { name }
    }
}

impl MpdCommand<Vec<MpdSong>> for ListPlaylistInfoCommand {
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<Vec<MpdSong>, Error>> {
        async move {
            let playlist = client.get_playlist_by_name(&self.name).await?;
            match playlist {
                Some(playlist) => {
                    let tracks = playlist.tracks.into_iter().map(MpdSong::from).collect();
                    Ok(tracks)
                }
                None => Ok(vec![]),
            }
        }.boxed()
    }
}
