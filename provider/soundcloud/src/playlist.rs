use rustic_core::library::{Playlist, Track};
use rustic_core::{provider, Album};

use crate::track::SoundcloudTrack;
use crate::user::SoundcloudUser;
use rustic_core::provider::ThumbnailState;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SoundcloudPlaylist {
    pub id: u64,
    pub title: String,
    pub tracks: Vec<Track>,
    playlist_type: soundcloud::PlaylistType,
    user: soundcloud::User,
    artwork_url: Option<String>,
}

impl SoundcloudPlaylist {
    pub fn is_album(&self) -> bool {
        self.playlist_type != soundcloud::PlaylistType::Playlist
    }
}

impl From<SoundcloudPlaylist> for Playlist {
    fn from(playlist: SoundcloudPlaylist) -> Self {
        Playlist {
            id: None,
            title: playlist.title,
            tracks: playlist.tracks,
            provider: provider::ProviderType::Soundcloud,
            uri: format!("soundcloud://playlist/{}", playlist.id),
        }
    }
}

impl From<SoundcloudPlaylist> for Album {
    fn from(playlist: SoundcloudPlaylist) -> Self {
        Album {
            id: None,
            title: playlist.title,
            tracks: playlist.tracks,
            provider: provider::ProviderType::Soundcloud,
            uri: format!("soundcloud://playlist/{}", playlist.id),
            thumbnail: playlist
                .artwork_url
                .map(|url| url.replace("large", "t500x500"))
                .map(ThumbnailState::Url)
                .unwrap_or_default(),
            artist: Some(SoundcloudUser::from(playlist.user).into()),
            meta: HashMap::new(),
            artist_id: None,
            explicit: None,
            description: None,
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
                .unwrap_or_default()
                .iter()
                .cloned()
                .filter(|track| track.stream_url.is_some())
                .map(SoundcloudTrack::from)
                .map(Track::from)
                .collect(),
            playlist_type: playlist
                .playlist_type
                .unwrap_or(soundcloud::PlaylistType::Playlist),
            user: playlist.user,
            artwork_url: playlist.artwork_url,
        }
    }
}

impl From<SoundcloudPlaylist> for provider::ProviderItem {
    fn from(playlist: SoundcloudPlaylist) -> Self {
        if playlist.is_album() {
            Album::from(playlist).into()
        } else {
            Playlist::from(playlist).into()
        }
    }
}
