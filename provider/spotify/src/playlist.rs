use rspotify::spotify::model::playlist::*;
use serde_derive::{Deserialize, Serialize};

use rustic_core::{Playlist, Provider, Track};

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
            provider: Provider::Spotify,
            uri: format!("spotify://playlists/{}", playlist.id),
            tracks: playlist
                .tracks
                .items
                .into_iter()
                .map(|track| SpotifyFullTrack::from(track.track))
                .map(Track::from)
                .collect(),
        }
    }
}