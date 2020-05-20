use crate::track::SoundcloudTrack;
use rustic_core::library::{Playlist, Track};
use rustic_core::provider;

#[derive(Debug, Clone)]
pub struct SoundcloudPlaylist {
    pub id: u64,
    pub title: String,
    pub tracks: Vec<Track>,
}

impl From<SoundcloudPlaylist> for Playlist {
    fn from(playlist: SoundcloudPlaylist) -> Playlist {
        Playlist {
            id: None,
            title: playlist.title,
            tracks: playlist.tracks,
            provider: provider::ProviderType::Soundcloud,
            uri: format!("soundcloud://playlist/{}", playlist.id),
        }
    }
}

impl From<soundcloud::Playlist> for SoundcloudPlaylist {
    fn from(playlist: soundcloud::Playlist) -> SoundcloudPlaylist {
        SoundcloudPlaylist {
            id: playlist.id,
            title: playlist.title,
            tracks: playlist
                .tracks
                .unwrap_or_else(|| Vec::new())
                .iter()
                .cloned()
                .filter(|track| track.stream_url.is_some())
                .map(SoundcloudTrack::from)
                .map(Track::from)
                .collect(),
        }
    }
}
