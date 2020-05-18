use rspotify::model::playlist::*;
use serde_derive::{Deserialize, Serialize};

use rustic_core::{Playlist, ProviderType, Track};

use crate::track::SpotifyFullTrack;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotifyPlaylist(FullPlaylist);

impl From<FullPlaylist> for SpotifyPlaylist {
    fn from(playlist: FullPlaylist) -> Self {
        SpotifyPlaylist(playlist)
    }
}

impl From<SpotifyPlaylist> for Playlist {
    fn from(playlist: SpotifyPlaylist) -> Self {
        let playlist = playlist.0;

        Playlist {
            id: None,
            title: playlist.name,
            provider: ProviderType::Spotify,
            uri: format!("spotify://playlists/{}", playlist.id),
            tracks: playlist
                .tracks
                .items
                .into_iter()
                .map(|track| track.track.map(SpotifyFullTrack::from))
                .filter_map(|t| t)
                .map(Track::from)
                .collect(),
        }
    }
}
