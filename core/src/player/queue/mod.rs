use crate::Track;
use failure::Error;
use std::fmt::Debug;

mod memory_queue;

pub use memory_queue::*;

pub trait PlayerQueue: Send + Sync + Debug {
    /// Put a single track at the end of the current queue
    fn queue_single(&self, track: &Track);

    /// Put multiple tracks at the end of the current queue
    fn queue_multiple(&self, tracks: &[Track]);

    /// Queue single track behind the current
    fn queue_next(&self, track: &Track);

    /// Returns all tracks which are queued up right now
    fn get_queue(&self) -> Vec<Track>;

    fn remove_item(&self, index: usize) -> Result<(), Error>;

    /// Clear the current queue
    fn clear(&self);

    /// Returns the currently playing track or None when nothing is playing
    fn current(&self) -> Option<Track>;

    /// Play the previous track in the current queue
    fn prev(&self) -> Result<Option<()>, Error>;

    /// Play the next track in the current queue
    fn next(&self) -> Result<Option<()>, Error>;
}
