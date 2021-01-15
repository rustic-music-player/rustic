use super::state::PlayerState;
use crate::library::Track;
use crate::player::{QueuedTrack, RepeatMode};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    /// Emitted when the player state changes
    StateChanged(PlayerState),
    /// Emitted when the player seeks to a different position
    Seek(Duration),
    /// The currently playing track has changed
    TrackChanged(Track),
    /// The queue has been changed
    QueueUpdated(Vec<QueuedTrack>),
    /// The player is waiting for I/O
    Buffering,
    /// The Volume has changed
    VolumeChanged(f32),
    /// The repeat mode has changed
    RepeatChanged(RepeatMode),
}
