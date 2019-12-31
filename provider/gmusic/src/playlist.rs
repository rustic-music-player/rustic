use gmusic::Playlist;

#[derive(Debug, Clone)]
pub struct GmusicPlaylist(Playlist);

impl From<Playlist> for GmusicPlaylist {
    fn from(playlist: Playlist) -> Self {
        GmusicPlaylist(playlist)
    }
}