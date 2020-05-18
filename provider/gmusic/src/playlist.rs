use rustic_core::{Playlist, ProviderType};

#[derive(Debug, Clone)]
pub struct GmusicPlaylist(gmusic::Playlist);

impl From<gmusic::Playlist> for GmusicPlaylist {
    fn from(playlist: gmusic::Playlist) -> Self {
        GmusicPlaylist(playlist)
    }
}

impl From<GmusicPlaylist> for Playlist {
    fn from(playlist: GmusicPlaylist) -> Self {
        let playlist = playlist.0;
        Playlist {
            id: None,
            title: playlist.name,
            provider: ProviderType::GooglePlayMusic,
            tracks: Vec::new(),
            uri: format!("gmusic:playlist:{}", playlist.id),
        }
    }
}
