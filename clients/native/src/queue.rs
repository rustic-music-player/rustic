use async_trait::async_trait;
use log::debug;
use failure::format_err;

use rustic_api::models::TrackModel;
use crate::RusticNativeClient;
use rustic_api::client::QueueApiClient;
use std::sync::Arc;
use rustic_core::player::Player;
use rustic_api::cursor::from_cursor;
use rustic_core::{SingleQuery, Track, PlayerState, Album, Playlist};

#[async_trait]
impl QueueApiClient for RusticNativeClient {
    async fn get_queue(&self, player_id: Option<&str>) -> Result<Vec<TrackModel>, failure::Error> {
        let player = self.get_player(player_id)?;
        let tracks = player.get_queue()
            .into_iter()
            .map(TrackModel::from)
            .collect();

        Ok(tracks)
    }

    async fn queue_track(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>, failure::Error> {
        let player = self.get_player(player_id)?;
        let uri = from_cursor(cursor)?;
        debug!("adding track to queue {}", uri);
        let track: Option<Track> = self.app.query_track(SingleQuery::uri(uri))?;
        match track {
            Some(track) => {
                let play = player.get_queue().is_empty() && player.backend.state() == PlayerState::Stop;
                player.queue.queue_single(&track);
                if play {
                    player.backend.set_state(PlayerState::Play)?;
                }

                Ok(Some(()))
            }
            None => Ok(None),
        }
    }

    async fn queue_album(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>, failure::Error> {
        let player = self.get_player(player_id)?;
        let uri = from_cursor(cursor)?;
        debug!("adding album to queue {}", uri);
        let album: Option<Album> = self.app.query_album(SingleQuery::uri(uri))?;
        match album {
            Some(album) => {
                self.queue_multiple(player, &album.tracks)?;

                Ok(Some(()))
            }
            None => Ok(None),
        }
    }

    async fn queue_playlist(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>, failure::Error> {
        let player = self.get_player(player_id)?;
        let uri = from_cursor(cursor)?;
        debug!("adding playlist to queue {}", uri);
        let playlist: Option<Playlist> = self.app.query_playlist(SingleQuery::uri(uri))?;
        match playlist {
            Some(playlist) => {
                self.queue_multiple(player, &playlist.tracks)?;

                Ok(Some(()))
            }
            None => Ok(None),
        }
    }

    async fn clear_queue(&self, player_id: Option<&str>) -> Result<(), failure::Error> {
        let player = self.get_player(player_id)?;
        player.queue.clear();

        Ok(())
    }

    async fn remove_queue_item(&self, player_id: Option<&str>, item: usize) -> Result<(), failure::Error> {
        let player = self.get_player(player_id)?;
        player.queue.remove_item(item)?;

        Ok(())
    }
}

impl RusticNativeClient {
    fn get_player(&self, player_id: Option<&str>) -> Result<Arc<Player>, failure::Error> {
        let player = match player_id {
            Some(id) => self.app.get_player(id.into()),
            None => self.app.get_default_player()
        };
        player.ok_or_else(|| format_err!("Missing default player"))
    }

    fn queue_multiple(&self, player: Arc<Player>, tracks: &Vec<Track>) -> Result<(), failure::Error> {
        let play = player.get_queue().is_empty() && player.backend.state() == PlayerState::Stop;
        player.queue.queue_multiple(&tracks);
        if play {
            player.backend.set_state(PlayerState::Play)?;
        }

        Ok(())
    }
}