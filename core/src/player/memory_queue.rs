use crate::Track;
use pinboard::NonEmptyPinboard;
use std::sync::atomic;

#[derive(Debug)]
pub struct MemoryQueue {
    queue: NonEmptyPinboard<Vec<Track>>,
    current_index: atomic::AtomicUsize,
}

impl MemoryQueue {
    pub fn new() -> Self {
        MemoryQueue {
            queue: NonEmptyPinboard::new(vec![]),
            current_index: atomic::AtomicUsize::new(0),
        }
    }

    pub fn queue_single(&self, track: &Track) {
        let mut queue = self.queue.read();
        queue.push(track.clone());
        self.queue.set(queue);
    }

    pub fn queue_multiple(&self, tracks: &[Track]) {
        let mut queue = self.queue.read();
        queue.append(&mut tracks.to_vec());
        self.queue.set(queue);
    }

    pub fn queue_next(&self, track: &Track) {
        let mut queue = self.queue.read();
        let current_index = self.current_index.load(atomic::Ordering::Relaxed);
        queue.insert(current_index + 1, track.clone());
        self.queue.set(queue);
    }

    pub fn get_queue(&self) -> Vec<Track> {
        self.queue.read()
    }

    pub fn clear(&self) {
        self.queue.set(vec![]);
        self.current_index.store(0, atomic::Ordering::Relaxed);
    }

    pub fn prev(&self) -> Option<Track> {
        let mut current_index = self.current_index.load(atomic::Ordering::Relaxed);
        if current_index == 0 {
            return None;
        }

        let queue = self.get_queue();

        current_index -= 1;
        self.current_index
            .store(current_index, atomic::Ordering::Relaxed);

        queue.get(current_index).cloned()
    }

    pub fn next(&self) -> Option<Track> {
        let mut current_index = self.current_index.load(atomic::Ordering::Relaxed);
        let queue = self.get_queue();

        if current_index >= queue.len() {
            return None;
        }
        current_index += 1;
        self.current_index
            .store(current_index, atomic::Ordering::Relaxed);
        queue.get(current_index).cloned()
    }

    pub fn get_current_track(&self) -> Option<Track> {
        let queue = self.get_queue();
        let current_index = self.current_index.load(atomic::Ordering::Relaxed);
        queue.get(current_index).cloned()
    }
}
