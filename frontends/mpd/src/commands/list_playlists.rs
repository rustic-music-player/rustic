use serde::Serialize;
use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::{Playlist, Rustic};
use std::sync::Arc;
use futures::future::BoxFuture;
use rustic_api::ApiClient;
use rustic_api::models::PlaylistModel;
use crate::FutureExt;

#[derive(Debug, Serialize)]
pub struct PlaylistEntry {
    playlist: String,
    #[serde(rename = "Last-Modified")]
    last_modified: String,
}

impl From<Playlist> for PlaylistEntry {
    fn from(playlist: Playlist) -> PlaylistEntry {
        PlaylistEntry {
            playlist: playlist.title,
            last_modified: "2017-12-23T17:15:13Z".to_owned(),
        }
    }
}

impl From<PlaylistModel> for PlaylistEntry {
    fn from(playlist: PlaylistModel) -> Self {
        Self {
            playlist: playlist.title,
            last_modified: "2017-12-23T17:15:13Z".to_owned(),
        }
    }
}

pub struct ListPlaylistsCommand {}

impl ListPlaylistsCommand {
    pub fn new() -> ListPlaylistsCommand {
        ListPlaylistsCommand {}
    }
}

impl MpdCommand<Vec<PlaylistEntry>> for ListPlaylistsCommand {
    fn handle(&self, _: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<Vec<PlaylistEntry>, Error>> {
        async move {
            let playlists = client.get_playlists(None).await?;
            let playlists = playlists
                .into_iter()
                .map(PlaylistEntry::from)
                .collect();
            Ok(playlists)
        }.boxed()
    }
}
