use maplit::hashmap;

use rustic_core::{Album, Artist, provider};
use rustic_core::library::{Playlist, Track};

use crate::meta::META_SOUNDCLOUD_USER_ID;
use crate::track::SoundcloudTrack;
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
            image_url: playlist.artwork_url,
            artist: Some(Artist {
                id: None,
                name: playlist.user.username,
                image_url: Some(playlist.user.avatar_url),
                uri: format!("soundcloud://user/{}", playlist.user.id),
                meta: hashmap!(
                    META_SOUNDCLOUD_USER_ID.into() => playlist.user.id.into()
                ),
                provider: provider::ProviderType::Soundcloud,
                albums: Vec::new(),
                playlists: Vec::new(),
            }),
            meta: HashMap::new(),
            artist_id: None
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
            playlist_type: playlist.playlist_type.unwrap_or(soundcloud::PlaylistType::Playlist),
            user: playlist.user,
            artwork_url: playlist.artwork_url,
        }
    }
}
