use crate::library::{Album, Artist, Playlist, Track};

#[derive(Debug, Clone)]
pub enum LibraryEvent {
    /// Emitted when a new track was added
    TrackAdded(Track),
    /// Emitted when the track with the given uri was removed
    TrackRemoved(String),
    /// Emitted when a new album was added
    AlbumAdded(Album),
    /// Emitted when the album with the given uri was removed
    AlbumRemoved(String),
    /// Emitted when a new artist was added
    ArtistAdded(Artist),
    /// Emitted when the artist with the given uri was removed
    ArtistRemoved(String),
    /// Emitted when a new playlist was added
    PlaylistAdded(Playlist),
    /// Emitted when the playlist with the given uri was removed
    PlaylistRemoved(String),
}
