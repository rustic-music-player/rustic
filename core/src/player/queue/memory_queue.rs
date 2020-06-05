use std::sync::atomic;

use failure::{format_err, Error};
use pinboard::NonEmptyPinboard;

use async_trait::async_trait;

use crate::player::{PlayerBuilder, PlayerBus, PlayerCommand};
use crate::{PlayerEvent, Track};

use super::PlayerQueue;

#[derive(Debug)]
pub struct MemoryQueue {
    queue: NonEmptyPinboard<Vec<Track>>,
    current_index: atomic::AtomicUsize,
    current_track: NonEmptyPinboard<Option<Track>>,
    bus: PlayerBus,
}

impl MemoryQueue {
    pub fn new(bus: PlayerBus) -> Self {
        MemoryQueue {
            queue: NonEmptyPinboard::new(vec![]),
            current_index: atomic::AtomicUsize::new(0),
            current_track: NonEmptyPinboard::new(None),
            bus,
        }
    }

    fn select_track(&self, queue: &[Track], index: usize) -> Result<Option<()>, Error> {
        let result = if let Some(track) = queue.get(index).cloned() {
            self.bus
                .send_player_msg(PlayerCommand::Play(track.clone()))?;
            self.current_track.set(Some(track));
            Some(())
        } else {
            self.current_track.set(None);
            None
        };
        Ok(result)
    }

    async fn queue_changed(&self) -> Result<(), Error> {
        self.bus
            .emit_event(PlayerEvent::QueueUpdated(self.get_queue().await?))?;
        Ok(())
    }

    async fn emit_current_track(&self) -> Result<(), Error> {
        let next = self.current().await?;
        let current = self.current_track.read();
        if current == next {
            return Ok(());
        }
        if let Some(track) = next {
            self.current_track.set(Some(track.clone()));
            self.bus.send_player_msg(PlayerCommand::Play(track))?;
        } else {
            self.current_track.set(None);
            self.bus.send_player_msg(PlayerCommand::Stop)?;
        }
        Ok(())
    }
}

#[async_trait]
impl PlayerQueue for MemoryQueue {
    async fn queue_single(&self, track: &Track) -> Result<(), Error> {
        let mut queue = self.queue.read();
        queue.push(track.clone());
        self.queue.set(queue);
        self.queue_changed().await?;
        self.emit_current_track().await?;

        Ok(())
    }

    async fn queue_multiple(&self, tracks: &[Track]) -> Result<(), Error> {
        let mut queue = self.queue.read();
        queue.append(&mut tracks.to_vec());
        self.queue.set(queue);
        self.queue_changed().await?;
        self.emit_current_track().await?;

        Ok(())
    }

    async fn queue_next(&self, track: &Track) -> Result<(), Error> {
        let mut queue = self.queue.read();
        let current_index = self.current_index.load(atomic::Ordering::Relaxed);
        queue.insert(current_index + 1, track.clone());
        self.queue.set(queue);
        self.queue_changed().await?;
        self.emit_current_track().await?;

        Ok(())
    }

    async fn get_queue(&self) -> Result<Vec<Track>, Error> {
        Ok(self.queue.read())
    }

    async fn remove_item(&self, index: usize) -> Result<(), Error> {
        let current_index = self.current_index.load(atomic::Ordering::Relaxed);
        let mut queue = self.queue.read();

        queue.remove(index);

        if current_index == index {
            self.select_track(&queue, current_index)?;
        }
        self.queue.set(queue);
        self.queue_changed().await?;

        Ok(())
    }

    async fn clear(&self) -> Result<(), Error> {
        self.queue.set(vec![]);
        self.current_index.store(0, atomic::Ordering::Relaxed);
        self.queue_changed().await?;
        self.emit_current_track().await?;

        Ok(())
    }

    async fn current(&self) -> Result<Option<Track>, Error> {
        let queue = self.get_queue().await?;
        let current_index = self.current_index.load(atomic::Ordering::Relaxed);

        Ok(queue.get(current_index).cloned())
    }

    async fn prev(&self) -> Result<Option<()>, Error> {
        let mut current_index = self.current_index.load(atomic::Ordering::Relaxed);
        if current_index == 0 {
            return Ok(None);
        }

        let queue = self.get_queue().await?;

        current_index -= 1;
        self.current_index
            .store(current_index, atomic::Ordering::Relaxed);

        Ok(self.select_track(&queue, current_index)?)
    }

    async fn next(&self) -> Result<Option<()>, Error> {
        let mut current_index = self.current_index.load(atomic::Ordering::Relaxed);
        let queue = self.get_queue().await?;

        if current_index >= queue.len() {
            return Ok(None);
        }
        current_index += 1;
        self.current_index
            .store(current_index, atomic::Ordering::Relaxed);

        Ok(self.select_track(&queue, current_index)?)
    }

    async fn reorder_item(&self, index_before: usize, index_after: usize) -> Result<(), Error> {
        let mut queue = self.get_queue().await?;
        if index_before >= queue.len() || index_after >= queue.len() {
            return Err(format_err!(
                "index out of bounds\nreorder_item got index outside of the queue size"
            ));
        }
        let item = queue.remove(index_before);
        queue.insert(index_after, item);
        self.queue.set(queue);
        self.queue_changed().await?;

        Ok(())
    }
}

pub trait MemoryQueueBuilder {
    fn with_memory_queue(&mut self) -> &mut Self;
}

impl MemoryQueueBuilder for PlayerBuilder {
    fn with_memory_queue(&mut self) -> &mut Self {
        self.with_queue(|_, bus| Ok(Box::new(MemoryQueue::new(bus))))
            .unwrap()
    }
}
