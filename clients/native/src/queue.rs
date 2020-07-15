use std::sync::Arc;

use failure::format_err;
use futures::future;
use futures::stream::BoxStream;
use futures::StreamExt;
use log::debug;

use async_trait::async_trait;
use rustic_api::client::{QueueApiClient, Result};
use rustic_api::cursor::from_cursor;
use rustic_api::models::{QueueEventModel, QueuedTrackModel};
use rustic_core::player::Player;
use rustic_core::{PlayerEvent, PlayerState, Playlist, QueryJoins, SingleQuery, Track};

use crate::stream_util::from_channel;
use crate::RusticNativeClient;
use rustic_extension_api::ExtensionApi;

#[async_trait]
impl QueueApiClient for RusticNativeClient {
    async fn get_queue(&self, player_id: Option<&str>) -> Result<Vec<QueuedTrackModel>> {
        let player = self.get_player_or_default(player_id)?;
        let tracks = player
            .get_queue()
            .await?
            .into_iter()
            .map(QueuedTrackModel::from)
            .collect();

        Ok(tracks)
    }

    async fn queue_track(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>> {
        let player = self.get_player_or_default(player_id)?;
        let uri = from_cursor(cursor)?;
        debug!("adding track to queue {}", uri);
        let track = self.query_track(SingleQuery::uri(uri)).await?;
        match track {
            Some(track) => {
                self.queue_multiple(player, &[track]).await?;

                Ok(Some(()))
            }
            None => Ok(None),
        }
    }

    async fn queue_album(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>> {
        let player = self.get_player_or_default(player_id)?;
        let uri = from_cursor(cursor)?;
        debug!("adding album to queue {}", uri);
        let mut query = SingleQuery::uri(uri);
        query.join_tracks();
        let album = self.query_album(query).await?;
        match album {
            Some(album) => {
                self.queue_multiple(player, &album.tracks).await?;

                Ok(Some(()))
            }
            None => Ok(None),
        }
    }

    async fn queue_playlist(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>> {
        let player = self.get_player_or_default(player_id)?;
        let uri = from_cursor(cursor)?;
        debug!("adding playlist to queue {}", uri);
        let playlist: Option<Playlist> = self.query_playlist(SingleQuery::uri(uri)).await?;
        match playlist {
            Some(playlist) => {
                self.queue_multiple(player, &playlist.tracks).await?;

                Ok(Some(()))
            }
            None => Ok(None),
        }
    }

    async fn clear_queue(&self, player_id: Option<&str>) -> Result<()> {
        let player = self.get_player_or_default(player_id)?;
        player.queue.clear().await?;

        Ok(())
    }

    async fn select_queue_item(&self, player_id: Option<&str>, item: usize) -> Result<()> {
        let player = self.get_player_or_default(player_id)?;
        player.queue.select_item(item).await?;

        Ok(())
    }

    async fn remove_queue_item(&self, player_id: Option<&str>, item: usize) -> Result<()> {
        let player = self.get_player_or_default(player_id)?;
        player.queue.remove_item(item).await?;

        Ok(())
    }

    async fn reorder_queue_item(
        &self,
        player_id: Option<&str>,
        before: usize,
        after: usize,
    ) -> Result<()> {
        let player = self.get_player_or_default(player_id)?;
        player.queue.reorder_item(before, after).await?;

        Ok(())
    }

    fn observe_queue(&self, player_id: Option<&str>) -> BoxStream<'static, QueueEventModel> {
        let player = self.get_player_or_default(player_id).unwrap();

        from_channel(player.observe())
            .filter(|e| match *e {
                PlayerEvent::QueueUpdated(_) => future::ready(true),
                _ => future::ready(false),
            })
            .map(QueueEventModel::from)
            .boxed()
    }
}

impl RusticNativeClient {
    pub(crate) fn get_player_or_default(&self, player_id: Option<&str>) -> Result<Arc<Player>> {
        let player = match player_id {
            Some(id) => self.app.get_player(id.into()),
            None => self.app.get_default_player(),
        };
        player.ok_or_else(|| format_err!("Missing default player"))
    }

    async fn queue_multiple(&self, player: Arc<Player>, tracks: &[Track]) -> Result<()> {
        let tracks = self.extensions.on_add_to_queue(tracks.to_vec()).await?;
        let play =
            player.get_queue().await?.is_empty() && player.backend.state() == PlayerState::Stop;
        player.queue.queue_multiple(&tracks).await?;
        if play {
            player.backend.set_state(PlayerState::Play)?;
        }

        Ok(())
    }
}
