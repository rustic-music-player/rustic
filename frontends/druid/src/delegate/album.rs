use crate::commands;
use crate::delegate::PartialDelegate;
use crate::state::{AsyncData, State};
use druid::{Command, Env, ExtEventSink, Target};
use futures::future::BoxFuture;
use futures::{future, FutureExt};
use rustic_api::ApiClient;
use std::sync::Arc;

pub struct AlbumDelegate {
    client: ApiClient,
}

impl AlbumDelegate {
    pub fn new(client: ApiClient) -> Self {
        AlbumDelegate { client }
    }
}

impl PartialDelegate for AlbumDelegate {
    fn command(
        &mut self,
        sink: ExtEventSink,
        _target: Target,
        cmd: &Command,
        data: &mut State,
        _env: &Env,
    ) -> Option<BoxFuture<'static, ()>> {
        if cmd.is(commands::LOAD_ALBUMS) {
            data.albums = AsyncData::Pending(());
            let client = self.client.clone();
            Some(
                async move {
                    let albums = client.get_albums(None).await.unwrap();
                    sink.submit_command(commands::events::ALBUMS_UPDATED, albums, Target::Auto)
                        .unwrap();
                }
                .boxed(),
            )
        } else if let Some(albums) = cmd.get(commands::events::ALBUMS_UPDATED) {
            let albums = albums.iter().map(|album| Arc::new(album.clone())).collect();
            data.albums = AsyncData::Resolved(albums);
            Some(future::ready(()).boxed())
        } else if let Some(album) = cmd.get(commands::events::ALBUM_ADDED) {
            match &mut data.albums {
                AsyncData::Resolved(ref mut albums) => albums.push_back(Arc::new(album.clone())),
                _ => {}
            }
            Some(future::ready(()).boxed())
        } else if let Some(cursor) = cmd.get(commands::events::ALBUM_REMOVED) {
            match &mut data.albums {
                AsyncData::Resolved(ref mut albums) => {
                    if let Some(index) = albums.iter().position(|album| &album.cursor == cursor) {
                        albums.remove(index);
                    }
                }
                _ => {}
            }
            Some(future::ready(()).boxed())
        } else {
            None
        }
    }
}
