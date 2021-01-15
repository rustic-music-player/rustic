use actix::Recipient;
use serde::Serialize;

use rustic_api::models::{QueuedTrackModel, TrackModel, AlbumModel, ArtistModel, PlaylistModel, LibraryEventModel};

#[derive(Message, Clone, Debug, Serialize)]
#[rtype(result = "()")]
#[serde(untagged)]
pub enum Message {
    PlayerMessage(PlayerMessage),
    LibraryMessage(LibraryMessage),
}

#[derive(Clone, Debug, Serialize)]
pub struct PlayerMessage {
    pub player_cursor: String,
    #[serde(flatten)]
    pub message: PlayerMessageData,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayerMessageData {
    PlayerStateChanged(bool),
    CurrentlyPlayingChanged(Option<TrackModel>),
    QueueUpdated(Vec<QueuedTrackModel>),
    VolumeChanged(f32),
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LibraryMessage {
    /// Emitted when a new track was added
    TrackAdded(TrackModel),
    /// Emitted when the track with the given cursor was removed
    TrackRemoved(String),
    /// Emitted when a new album was added
    AlbumAdded(AlbumModel),
    /// Emitted when the album with the given cursor was removed
    AlbumRemoved(String),
    /// Emitted when a new artist was added
    ArtistAdded(ArtistModel),
    /// Emitted when the artist with the given cursor was removed
    ArtistRemoved(String),
    /// Emitted when a new playlist was added
    PlaylistAdded(PlaylistModel),
    /// Emitted when the playlist with the given cursor was removed
    PlaylistRemoved(String),
}

impl From<LibraryEventModel> for LibraryMessage {
    fn from(event: LibraryEventModel) -> Self {
        match event {
            LibraryEventModel::TrackAdded(track) => LibraryMessage::TrackAdded(track),
            LibraryEventModel::TrackRemoved(cursor) => LibraryMessage::TrackRemoved(cursor),
            LibraryEventModel::AlbumAdded(album) => LibraryMessage::AlbumAdded(album),
            LibraryEventModel::AlbumRemoved(cursor) => LibraryMessage::AlbumRemoved(cursor),
            LibraryEventModel::ArtistAdded(artist) => LibraryMessage::ArtistAdded(artist),
            LibraryEventModel::ArtistRemoved(cursor) => LibraryMessage::ArtistRemoved(cursor),
            LibraryEventModel::PlaylistAdded(playlist) => LibraryMessage::PlaylistAdded(playlist),
            LibraryEventModel::PlaylistRemoved(cursor) => LibraryMessage::PlaylistRemoved(cursor),
        }
    }
}

#[derive(Message)]
#[rtype(String)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Ping;
