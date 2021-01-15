use super::stream::StreamDelegate;
use crate::commands::events;
use druid::{ExtEventSink, Target};
use futures::future::BoxFuture;
use futures::FutureExt;
use futures::StreamExt;
use rustic_api::{models::LibraryEventModel, ApiClient};

pub struct LibraryDelegate {
    client: ApiClient,
}

impl LibraryDelegate {
    pub fn new(client: ApiClient) -> Self {
        LibraryDelegate { client }
    }
}

impl StreamDelegate for LibraryDelegate {
    fn observe(&self, sink: ExtEventSink) -> BoxFuture<'static, ()> {
        self.client
            .observe_library()
            .for_each(move |event| {
                log::trace!("received library message {:?}", event);
                let sink = sink.clone();
                async move {
                    match event {
                        LibraryEventModel::AlbumAdded(model) => {
                            sink.submit_command(events::ALBUM_ADDED, model, Target::Auto)
                        }
                        LibraryEventModel::ArtistAdded(model) => {
                            sink.submit_command(events::ARTIST_ADDED, model, Target::Auto)
                        }
                        LibraryEventModel::PlaylistAdded(model) => {
                            sink.submit_command(events::PLAYLIST_ADDED, model, Target::Auto)
                        }
                        LibraryEventModel::TrackAdded(model) => {
                            sink.submit_command(events::TRACK_ADDED, model, Target::Auto)
                        }
                        LibraryEventModel::AlbumRemoved(cursor) => {
                            sink.submit_command(events::ALBUM_REMOVED, cursor, Target::Auto)
                        }
                        LibraryEventModel::ArtistRemoved(cursor) => {
                            sink.submit_command(events::ARTIST_REMOVED, cursor, Target::Auto)
                        }
                        LibraryEventModel::PlaylistRemoved(cursor) => {
                            sink.submit_command(events::PLAYLIST_REMOVED, cursor, Target::Auto)
                        }
                        LibraryEventModel::TrackRemoved(cursor) => {
                            sink.submit_command(events::TRACK_REMOVED, cursor, Target::Auto)
                        }
                    }
                    .unwrap()
                }
            })
            .boxed()
    }
}
