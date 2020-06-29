use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use failure::Error;

use async_trait::async_trait;
pub use memory_queue::*;

use crate::Track;

mod memory_queue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedTrack {
    #[serde(flatten)]
    pub track: Track,
    pub playing: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RepeatMode {
    None,
    Single,
    All,
}

#[async_trait]
pub trait PlayerQueue: Send + Sync + Debug {
    /// Put a single track at the end of the current queue
    async fn queue_single(&self, track: &Track) -> Result<(), Error>;

    /// Put multiple tracks at the end of the current queue
    async fn queue_multiple(&self, tracks: &[Track]) -> Result<(), Error>;

    /// Queue single track behind the current
    async fn queue_next(&self, track: &Track) -> Result<(), Error>;

    /// Returns all tracks which are queued up right now
    async fn get_queue(&self) -> Result<Vec<QueuedTrack>, Error>;

    /// Jump to item at given index
    async fn select_item(&self, index: usize) -> Result<(), Error>;

    /// Remove item at the given index
    async fn remove_item(&self, index: usize) -> Result<(), Error>;

    /// Clear the current queue
    async fn clear(&self) -> Result<(), Error>;

    /// Returns the currently playing track or None when nothing is playing
    async fn current(&self) -> Result<Option<Track>, Error>;

    /// Play the previous track in the current queue
    async fn prev(&self) -> Result<Option<()>, Error>;

    /// Play the next track in the current queue
    async fn next(&self) -> Result<Option<()>, Error>;

    /// Move item at index_before to index_after
    /// Should fail when index_before or index_after are out of bounds
    async fn reorder_item(&self, index_before: usize, index_after: usize) -> Result<(), Error>;

    async fn set_shuffle(&self, shuffle: bool) -> Result<(), Error>;

    async fn shuffle(&self) -> Result<bool, Error>;

    async fn set_repeat(&self, repeat: RepeatMode) -> Result<(), Error>;

    async fn repeat(&self) -> Result<RepeatMode, Error>;
}
