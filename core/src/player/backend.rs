use std::any::Any;
use std::fmt::Debug;
use std::time::Duration;

use failure::Error;

use crate::{PlayerState, Track};

pub trait PlayerBackend: Send + Sync + Debug {
    fn set_track(&self, track: &Track, stream_url: String) -> Result<(), Error>;

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

    fn as_any(&self) -> &dyn Any;

    fn close(&self) -> Result<(), Error> {
        Ok(())
    }
}
