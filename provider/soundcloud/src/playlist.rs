use rustic::library::{Playlist, Track};
use rustic::provider;
use soundcloud;
use track::SoundcloudTrack;

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
            provider: provider::Provider::Soundcloud,
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
                .iter()
                .cloned()
                .filter(|track| track.stream_url.is_some())
                .map(SoundcloudTrack::from)
                .map(Track::from)
                .collect(),
        }
    }
}
