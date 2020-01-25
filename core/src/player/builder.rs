use crate::player::{Player, PlayerBackend, PlayerCommand, PlayerQueue, QueueCommand};
use crate::{PlayerEvent, Rustic};
use crossbeam_channel::{unbounded, Receiver, Sender};
use failure::Error;
use std::sync::Arc;

pub struct PlayerBuilder {
    core: Arc<Rustic>,
    backend: Option<Box<dyn PlayerBackend>>,
    queue: Option<Box<dyn PlayerQueue>>,
    player_rx: Receiver<PlayerCommand>,
    player_tx: Sender<PlayerCommand>,
    queue_rx: Receiver<QueueCommand>,
    queue_tx: Sender<QueueCommand>,
    event_rx: Receiver<PlayerEvent>,
    event_tx: Sender<PlayerEvent>,
}

impl PlayerBuilder {
    pub fn new(core: Arc<Rustic>) -> Self {
        let (player_tx, player_rx) = unbounded();
        let (queue_tx, queue_rx) = unbounded();
        let (event_tx, event_rx) = unbounded();

        PlayerBuilder {
            core,
            backend: None,
            queue: None,
            player_rx,
            player_tx,
            queue_rx,
            queue_tx,
            event_rx,
            event_tx,
        }
    }

    pub fn with_player<P>(&mut self, builder: P) -> Result<&mut Self, Error>
    where
        P: Fn(
            Arc<Rustic>,
            Sender<QueueCommand>,
            Receiver<PlayerCommand>,
            Sender<PlayerEvent>,
        ) -> Result<Box<dyn PlayerBackend>, Error>,
    {
        let backend = builder(
            Arc::clone(&self.core),
            self.queue_tx.clone(),
            self.player_rx.clone(),
            self.event_tx.clone(),
        )?;
        self.backend = Some(backend);

        Ok(self)
    }

    pub fn with_queue<Q>(&mut self, builder: Q) -> Result<&mut Self, Error>
    where
        Q: Fn(
            Arc<Rustic>,
            Sender<PlayerCommand>,
            Receiver<QueueCommand>,
            Sender<PlayerEvent>,
        ) -> Result<Box<dyn PlayerQueue>, Error>,
    {
        let queue = builder(
            Arc::clone(&self.core),
            self.player_tx.clone(),
            self.queue_rx.clone(),
            self.event_tx.clone(),
        )?;
        self.queue = Some(queue);

        Ok(self)
    }

    pub fn build(&mut self) -> Arc<Player> {
        assert!(self.backend.is_some());
        assert!(self.queue.is_some());
        let backend = self.backend.take().unwrap();
        let queue = self.queue.take().unwrap();

        Player::new(
            backend,
            queue,
            self.player_rx.clone(),
            self.queue_rx.clone(),
            self.event_rx.clone(),
        )
    }
}
