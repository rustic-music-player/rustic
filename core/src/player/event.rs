use super::state::PlayerState;
use crate::library::Track;
use std::time::Duration;
use crate::player::QueuedTrack;

#[derive(Debug)]
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
    VolumeChanged(f32)
}
