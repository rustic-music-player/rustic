pub mod event;
pub mod state;

use crate::channel::Receiver;
use failure::Error;
use crate::library::Track;
use std::any::Any;
use std::time::Duration;

pub use self::event::PlayerEvent;
pub use self::state::PlayerState;

pub trait PlayerBackend: Send + Sync {
    /// Put a single track at the end of the current queue
    fn queue_single(&self, track: &Track);

    /// Put multiple tracks at the end of the current queue
    fn queue_multiple(&self, tracks: &[Track]);

    /// Queue single track behind the current
    fn queue_next(&self, track: &Track);

    /// Returns all tracks which are queued up right now
    fn get_queue(&self) -> Vec<Track>;

    /// Clear the current queue
    /// Does not stop playback
    fn clear_queue(&self);

    /// Returns the currently playing track or None when nothing is playing
    fn current(&self) -> Option<Track>;

    /// Play the previous track in the current queue
    fn prev(&self) -> Result<Option<()>, Error>;

    /// Play the next track in the current queue
    fn next(&self) -> Result<Option<()>, Error>;

    /// Set the player state
    fn set_state(&self, state: PlayerState) -> Result<(), Error>;

    /// Get the player state
    fn state(&self) -> PlayerState;

    /// Set the volume of this player (from 0 to 1)
    fn set_volume(&self, volume: f32) -> Result<(), Error>;

    /// Get the volume of this player (from 0 to 1)
    fn volume(&self) -> f32;

    /// Set time from the end of the current track when the next track should start playing
    fn set_blend_time(&self, duration: Duration) -> Result<(), Error>;

    /// Get time from the end of the current track when the next track should start playing
    fn blend_time(&self) -> Duration;

    /// Seek to a point in the current track
    fn seek(&self, duration: Duration) -> Result<(), Error>;

    fn observe(&self) -> Receiver<PlayerEvent>;

    fn as_any(&self) -> &dyn Any;
}