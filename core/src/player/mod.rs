use std::fmt::Debug;
use std::sync::Arc;
use std::thread;

use crossbeam_channel::{select, Receiver};
use failure::Error;

use crate::library::Track;
pub use crate::player::backend::PlayerBackend;

pub use self::builder::PlayerBuilder;
pub use self::event::PlayerEvent;
pub use self::queue::PlayerQueue;
pub use self::state::PlayerState;

pub mod backend;
pub mod builder;
pub mod event;
pub mod queue;
pub mod state;

#[derive(Debug)]
pub struct Player {
    pub display_name: String,
    pub backend: Box<dyn PlayerBackend>,
    pub queue: Box<dyn PlayerQueue>,
    event_rx: Receiver<PlayerEvent>,
}

impl Player {
    pub fn new(
        display_name: String,
        backend: Box<dyn PlayerBackend>,
        queue: Box<dyn PlayerQueue>,
        player_rx: Receiver<PlayerCommand>,
        queue_rx: Receiver<QueueCommand>,
        event_rx: Receiver<PlayerEvent>,
    ) -> Arc<Self> {
        let player = Player {
            display_name,
            backend,
            queue,
            event_rx,
        };
        let player = Arc::new(player);

        let player_2 = Arc::clone(&player);
        thread::spawn(move || {
            let player = player_2;
            loop {
                select! {
                    recv(player_rx) -> msg => {
                        msg.map_err(|err| err.into()).and_then(|msg| player.handle_player_msg(msg));
                    },
                    recv(queue_rx) -> msg => {
                        msg.map_err(|err| err.into()).and_then(|msg| player.handle_queue_msg(msg));
                    }
                }
            }
        });

        player
    }

    pub fn clear_queue(&self) {
        self.queue.clear();
    }

    pub fn get_queue(&self) -> Vec<Track> {
        self.queue.get_queue()
    }

    fn handle_player_msg(&self, msg: PlayerCommand) -> Result<(), Error> {
        match msg {
            PlayerCommand::Play(track) => self.backend.set_track(&track)?,
            PlayerCommand::Stop => self.backend.set_state(PlayerState::Stop)?,
        };
        Ok(())
    }

    fn handle_queue_msg(&self, msg: QueueCommand) -> Result<(), Error> {
        match msg {
            QueueCommand::Next => self.queue.next()?,
        };
        Ok(())
    }

    pub fn observe(&self) -> Receiver<PlayerEvent> {
        self.event_rx.clone()
    }
}

#[derive(Debug, Clone)]
pub enum PlayerCommand {
    Play(Track),
    Stop,
}

#[derive(Debug, Clone)]
pub enum QueueCommand {
    Next,
}
