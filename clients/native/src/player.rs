use std::sync::Arc;

use futures::future;
use futures::stream::BoxStream;
use futures::StreamExt;

use async_trait::async_trait;
use rustic_api::client::{PlayerApiClient, Result};
use rustic_api::cursor::to_cursor;
use rustic_api::models::*;
use rustic_core::{PlayerEvent, PlayerState};
use rustic_core::player::Player;

use crate::RusticNativeClient;
use crate::stream_util::from_channel;

#[async_trait]
impl PlayerApiClient for RusticNativeClient {
    async fn get_players(&self) -> Result<Vec<PlayerModel>> {
        let mut players = Vec::new();
        for (id, player) in self.app.get_players() {
            players.push(player_to_model(id, player).await?);
        }

        Ok(players)
    }

    async fn get_player(&self, player_id: Option<&str>) -> Result<Option<PlayerModel>> {
        let player = self.get_player_or_default(player_id)?;
        let player_id = player_id
            .map(|id| id.to_owned())
            .or_else(|| self.app.get_default_player_id())
            .unwrap();

        let state = player_to_model(player_id, player).await?;

        Ok(Some(state))
    }

    async fn player_control_next(&self, player_id: Option<&str>) -> Result<Option<()>> {
        let player = self.get_player_or_default(player_id)?;

        player.queue.next().await
    }

    async fn player_control_prev(&self, player_id: Option<&str>) -> Result<Option<()>> {
        let player = self.get_player_or_default(player_id)?;

        player.queue.prev().await
    }

    async fn player_control_play(&self, player_id: Option<&str>) -> Result<()> {
        let player = self.get_player_or_default(player_id)?;

        player.backend.set_state(PlayerState::Play)
    }

    async fn player_control_pause(&self, player_id: Option<&str>) -> Result<()> {
        let player = self.get_player_or_default(player_id)?;

        player.backend.set_state(PlayerState::Pause)
    }

    async fn player_set_volume(&self, player_id: Option<&str>, volume: f32) -> Result<()> {
        let player = self.get_player_or_default(player_id)?;
        player.backend.set_volume(volume)?;

        Ok(())
    }

    async fn player_set_repeat(&self, player_id: Option<&str>, repeat: RepeatModeModel) -> Result<()> {
        let player = self.get_player_or_default(player_id)?;
        player.queue.set_repeat(repeat.into()).await?;

        Ok(())
    }

    fn observe_player(&self, player_id: Option<&str>) -> BoxStream<'static, PlayerEventModel> {
        let player = self.get_player_or_default(player_id).unwrap();

        from_channel(player.observe())
            .filter(|e| match *e {
                PlayerEvent::QueueUpdated(_) => future::ready(false),
                _ => future::ready(true),
            })
            .map(PlayerEventModel::from)
            .boxed()
    }
}

async fn player_to_model(player_id: String, player: Arc<Player>) -> Result<PlayerModel> {
    let player_state = player.backend.state();
    let current = if player_state == PlayerState::Stop {
        None
    } else {
        player.queue.current().await?.map(TrackModel::from)
    };
    let volume = player.backend.volume();
    let repeat_mode = player.queue.repeat().await?;

    Ok(PlayerModel {
        cursor: to_cursor(&player_id),
        name: player.display_name.clone(),
        playing: (player_state == PlayerState::Play),
        volume,
        current,
        repeat: repeat_mode.into()
    })
}
