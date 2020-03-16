use std::sync::atomic;

use crossbeam_channel::Sender;
use failure::Error;
use pinboard::NonEmptyPinboard;

use crate::player::{PlayerBuilder, PlayerCommand};
use crate::{PlayerEvent, Track};

use super::PlayerQueue;

#[derive(Debug)]
pub struct MemoryQueue {
    queue: NonEmptyPinboard<Vec<Track>>,
    current_index: atomic::AtomicUsize,
    current_track: NonEmptyPinboard<Option<Track>>,
    player_tx: Sender<PlayerCommand>,
    event_tx: Sender<PlayerEvent>,
}

impl MemoryQueue {
    pub fn new(player_tx: Sender<PlayerCommand>, event_tx: Sender<PlayerEvent>) -> Self {
        MemoryQueue {
            queue: NonEmptyPinboard::new(vec![]),
            current_index: atomic::AtomicUsize::new(0),
            current_track: NonEmptyPinboard::new(None),
            player_tx,
            event_tx,
        }
    }

    fn select_track(&self, queue: &Vec<Track>, index: usize) -> Option<()> {
        if let Some(track) = queue.get(index).cloned() {
            self.player_tx.send(PlayerCommand::Play(track));
            Some(())
        } else {
            None
        }
    }

    fn queue_changed(&self) {
        self.event_tx
            .send(PlayerEvent::QueueUpdated(self.get_queue()));
    }

    fn emit_current_track(&self) {
        let current = self.current();
        if self.current_track.read() == current {
            return;
        }
        if let Some(track) = self.current() {
            self.current_track.set(Some(track.clone()));
            self.player_tx.send(PlayerCommand::Play(track));
        } else {
            self.current_track.set(None);
            self.player_tx.send(PlayerCommand::Stop);
        }
    }
}

impl PlayerQueue for MemoryQueue {
    fn queue_single(&self, track: &Track) {
        let mut queue = self.queue.read();
        queue.push(track.clone());
        self.queue.set(queue);
        self.queue_changed();
        self.emit_current_track();
    }

    fn queue_multiple(&self, tracks: &[Track]) {
        let mut queue = self.queue.read();
        queue.append(&mut tracks.to_vec());
        self.queue.set(queue);
        self.queue_changed();
        self.emit_current_track();
    }

    fn queue_next(&self, track: &Track) {
        let mut queue = self.queue.read();
        let current_index = self.current_index.load(atomic::Ordering::Relaxed);
        queue.insert(current_index + 1, track.clone());
        self.queue.set(queue);
        self.queue_changed();
        self.emit_current_track();
    }

    fn get_queue(&self) -> Vec<Track> {
        self.queue.read()
    }

    fn remove_item(&self, index: usize) -> Result<(), Error> {
        let current_index = self.current_index.load(atomic::Ordering::Relaxed);
        let mut queue = self.queue.read();

        queue.remove(index);

        if current_index == index {
            self.select_track(&queue, current_index);
        }
        self.queue.set(queue);
        self.queue_changed();

        Ok(())
    }

    fn clear(&self) {
        self.queue.set(vec![]);
        self.current_index.store(0, atomic::Ordering::Relaxed);
        self.queue_changed();
        self.emit_current_track();
    }

    fn prev(&self) -> Result<Option<()>, Error> {
        let mut current_index = self.current_index.load(atomic::Ordering::Relaxed);
        if current_index == 0 {
            return Ok(None);
        }

        let queue = self.get_queue();

        current_index -= 1;
        self.current_index
            .store(current_index, atomic::Ordering::Relaxed);

        Ok(self.select_track(&queue, current_index))
    }

    fn next(&self) -> Result<Option<()>, Error> {
        let mut current_index = self.current_index.load(atomic::Ordering::Relaxed);
        let queue = self.get_queue();

        if current_index >= queue.len() {
            return Ok(None);
        }
        current_index += 1;
        self.current_index
            .store(current_index, atomic::Ordering::Relaxed);

        Ok(self.select_track(&queue, current_index))
    }

    fn current(&self) -> Option<Track> {
        let queue = self.get_queue();
        let current_index = self.current_index.load(atomic::Ordering::Relaxed);
        queue.get(current_index).cloned()
    }
}

pub trait MemoryQueueBuilder {
    fn with_memory_queue(&mut self) -> &mut Self;
}

impl MemoryQueueBuilder for PlayerBuilder {
    fn with_memory_queue(&mut self) -> &mut Self {
        self.with_queue(|_, player_tx, _, event_tx| {
            Ok(Box::new(MemoryQueue::new(player_tx, event_tx)))
        })
        .unwrap()
    }
}
