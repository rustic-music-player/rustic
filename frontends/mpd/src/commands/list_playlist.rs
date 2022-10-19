use serde::Serialize;
use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::{Rustic};
use std::sync::Arc;
use futures::future::BoxFuture;
use futures::FutureExt;
use rustic_api::ApiClient;
use rustic_api::models::TrackModel;
use crate::client_ext::ClientExt;

#[derive(Debug, Serialize)]
pub struct PlaylistItem {
    file: String,
}

impl From<TrackModel> for PlaylistItem {
    fn from(track: TrackModel) -> Self {
        Self { file: track.cursor }
    }
}

pub struct ListPlaylistCommand {
    name: String,
}

impl ListPlaylistCommand {
    pub fn new(name: String) -> ListPlaylistCommand {
        ListPlaylistCommand { name }
    }
}

impl MpdCommand<Vec<PlaylistItem>> for ListPlaylistCommand {
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<Vec<PlaylistItem>, Error>> {
        async move {
            let playlist = client.get_playlist_by_name(&self.name).await?;

            match playlist {
                Some(playlist) => {
                    let tracks = playlist
                        .tracks
                        .into_iter()
                        .map(PlaylistItem::from)
                        .collect();
                    Ok(tracks)
                }
                None => Ok(vec![]),
            }
        }.boxed()
    }
}
