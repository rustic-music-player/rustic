use futures::future;
use futures::stream::BoxStream;
use futures::StreamExt;

use async_trait::async_trait;
use rustic_api::client::{PlayerApiClient, Result};
use rustic_api::cursor::to_cursor;
use rustic_api::models::*;
use rustic_core::{PlayerEvent, PlayerState};

use crate::RusticNativeClient;
use crate::stream_util::from_channel;

#[async_trait]
impl PlayerApiClient for RusticNativeClient {
    async fn get_players(&self) -> Result<Vec<PlayerModel>> {
        let players = self.app.get_players()
            .into_iter()
            .map(|(id, player)| {
                let track = player.queue.current().map(TrackModel::from);

                PlayerModel {
                    cursor: to_cursor(&id),
                    name: player.display_name.clone(),
                    playing: (player.backend.state() == PlayerState::Play),
                    current: track,
                }
            })
            .collect();

        Ok(players)
    }

    async fn get_player(&self, player_id: Option<&str>) -> Result<Option<PlayerModel>> {
        let player = self.get_player_or_default(player_id)?;
        let player_id = player_id.map(|id| id.to_owned())
            .or_else(|| self.app.get_default_player_id())
            .unwrap();
        let current = player.queue.current().map(TrackModel::from);

        let state = PlayerModel {
            cursor: to_cursor(&player_id),
            name: player.display_name.clone(),
            playing: (player.backend.state() == PlayerState::Play),
            current,
        };

        Ok(Some(state))
    }

    async fn player_control_next(&self, player_id: Option<&str>) -> Result<Option<()>> {
        let player = self.get_player_or_default(player_id)?;

        player.queue.next()
    }

    async fn player_control_prev(&self, player_id: Option<&str>) -> Result<Option<()>> {
        let player = self.get_player_or_default(player_id)?;

        player.queue.prev()
    }

    async fn player_control_play(&self, player_id: Option<&str>) -> Result<()> {
        let player = self.get_player_or_default(player_id)?;

        player.backend.set_state(PlayerState::Play)
    }

    async fn player_control_pause(&self, player_id: Option<&str>) -> Result<()> {
        let player = self.get_player_or_default(player_id)?;

        player.backend.set_state(PlayerState::Pause)
    }

    fn observe_player(&self, player_id: Option<&str>) -> BoxStream<'static, PlayerEventModel> {
        let player = self.get_player_or_default(player_id).unwrap();

        from_channel(player.observe()).filter(|e| {
            match e {
                &PlayerEvent::QueueUpdated(_) => future::ready(false),
                _ => future::ready(true)
            }
        }).map(PlayerEventModel::from).boxed()
    }
}
