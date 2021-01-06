use crate::commands;
use crate::delegate::PartialDelegate;
use crate::state::{AsyncData, State};
use druid::{Command, Env, ExtEventSink, Target};
use futures::{future, future::BoxFuture, FutureExt};
use rustic_api::ApiClient;
use std::sync::Arc;

pub struct PlaylistDelegate {
    client: ApiClient,
}

impl PlaylistDelegate {
    pub fn new(client: ApiClient) -> Self {
        PlaylistDelegate { client }
    }
}

impl PartialDelegate for PlaylistDelegate {
    fn command(
        &mut self,
        sink: ExtEventSink,
        _target: Target,
        cmd: &Command,
        data: &mut State,
        _env: &Env,
    ) -> Option<BoxFuture<'static, ()>> {
        if let Some(link) = cmd.get(commands::LOAD_PLAYLIST) {
            if let Some(playlist) = data
                .sidebar
                .playlists
                .iter()
                .find(|playlist| playlist.cursor == link.cursor)
            {
                sink.submit_command(
                    commands::events::PLAYLIST_LOADED,
                    (link.clone(), Arc::clone(playlist)),
                    Target::Auto,
                )
                .unwrap();
                Some(future::ready(()).boxed())
            } else {
                let client = self.client.clone();
                let link = link.clone();
                Some(
                    async move {
                        let playlist = client.get_playlist(&link.cursor).await.unwrap().unwrap();
                        sink.submit_command(
                            commands::events::PLAYLIST_LOADED,
                            (link, Arc::new(playlist)),
                            Target::Auto,
                        )
                        .unwrap()
                    }
                    .boxed(),
                )
            }
        } else if let Some((_, playlist)) = cmd.get(commands::events::PLAYLIST_LOADED) {
            data.playlist.playlist = AsyncData::Resolved(Arc::clone(playlist));
            data.playlist.tracks = AsyncData::Resolved(playlist.into());
            Some(future::ready(()).boxed())
        } else if cmd.is(commands::LOAD_PLAYLISTS) {
            let client = self.client.clone();
            Some(
                async move {
                    let playlists = client.get_playlists(None).await.unwrap();
                    sink.submit_command(
                        commands::events::PLAYLISTS_UPDATED,
                        playlists,
                        Target::Auto,
                    )
                    .unwrap()
                }
                .boxed(),
            )
        } else if let Some(playlists) = cmd.get(commands::events::PLAYLISTS_UPDATED) {
            let playlists = playlists
                .iter()
                .map(|playlist| Arc::new(playlist.clone()))
                .collect();
            data.sidebar.playlists = playlists;
            Some(future::ready(()).boxed())
        } else {
            None
        }
    }
}
