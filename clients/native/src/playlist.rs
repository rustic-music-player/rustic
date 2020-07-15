use async_trait::async_trait;
use failure::format_err;

use rustic_api::client::*;
use rustic_api::cursor::from_cursor;
use rustic_api::models::PlaylistModel;
use rustic_core::{Playlist, ProviderType};

use crate::RusticNativeClient;

#[async_trait]
impl PlaylistApiClient for RusticNativeClient {
    async fn add_playlist(&self, name: &str) -> Result<PlaylistModel> {
        let mut playlist = Playlist {
            id: None,
            title: name.into(),
            tracks: vec![],
            provider: ProviderType::Internal,
            uri: format!("internal://playlist/{}", name),
        };
        self.app.library.add_playlist(&mut playlist)?;
        self.app.library.flush()?;
        Ok(playlist.into())
    }

    async fn remove_playlist(&self, cursor: &str) -> Result<()> {
        let playlist = self
            .app
            .library
            .query_playlist(from_cursor(cursor)?.into())?
            .ok_or_else(|| format_err!("unknown playlist"))?;
        self.app.library.remove_playlist(&playlist)?;
        self.app.library.flush()?;

        Ok(())
    }

    async fn add_track_to_playlist(&self, cursor: &str, track: &str) -> Result<()> {
        let mut playlist = self
            .app
            .library
            .query_playlist(from_cursor(cursor)?.into())?
            .ok_or_else(|| format_err!("unknown playlist"))?;
        let track = self
            .query_track(from_cursor(track)?.into())
            .await?
            .ok_or_else(|| format_err!("unknown track"))?;
        playlist.tracks.push(track);
        self.app.library.sync_playlist(&mut playlist)?;
        self.app.library.flush()?;

        Ok(())
    }

    async fn remove_track_from_playlist(&self, cursor: &str, track: &str) -> Result<()> {
        let mut playlist = self
            .app
            .library
            .query_playlist(from_cursor(cursor)?.into())?
            .ok_or_else(|| format_err!("unknown playlist"))?;
        let track_uri = from_cursor(track)?;
        if let Some(track_index) = playlist.tracks.iter().position(|t| t.uri == track_uri) {
            playlist.tracks.remove(track_index);
            self.app.library.sync_playlist(&mut playlist)?;
            self.app.library.flush()?;
            Ok(())
        } else {
            Err(format_err!("Track is not in playlist"))
        }
    }
}
