use std::fmt;
use std::sync::Arc;

use crossbeam_channel::Receiver;
use failure::Error;
use log::error;

use crate::library::Track;
pub use crate::player::backend::PlayerBackend;
pub use self::bus::PlayerBus;

pub use self::builder::PlayerBuilder;
pub use self::event::PlayerEvent;
pub use self::queue::PlayerQueue;
pub use self::state::PlayerState;
use crate::player::bus::PlayerBusCommand;
use crate::Rustic;
use futures::prelude::*;

pub mod backend;
pub mod builder;
pub mod bus;
pub mod event;
pub mod queue;
pub mod state;

pub struct Player {
    pub display_name: String,
    pub backend: Box<dyn PlayerBackend>,
    pub queue: Box<dyn PlayerQueue>,
    bus: PlayerBus,
    core: Arc<Rustic>,
}

impl Player {
    pub fn new(
        display_name: String,
        backend: Box<dyn PlayerBackend>,
        queue: Box<dyn PlayerQueue>,
        bus: PlayerBus,
        core: Arc<Rustic>,
    ) -> Arc<Self> {
        let player = Player {
            display_name,
            backend,
            queue,
            bus,
            core,
        };
        let player = Arc::new(player);

        let player_2 = Arc::clone(&player);
        tokio::spawn(async move {
            let player = player_2;
            let mut stream = Box::pin(player.bus.commands());
            loop {
                let result = {
                    match stream.try_next().await {
                        Ok(Some(PlayerBusCommand::Player(cmd))) => player.handle_player_msg(cmd).await,
                        Ok(Some(PlayerBusCommand::Queue(cmd))) => player.handle_queue_msg(cmd).await,
                        Ok(None) => Ok(()),
                        Err(e) => Err(e.into())
                    }
                };
                if let Err(e) = result {
                    error!("{:?}", e);
                }
            }
        });

        player
    }

    pub async fn clear_queue(&self) -> Result<(), Error> {
        self.queue.clear().await
    }

    pub async fn get_queue(&self) -> Result<Vec<Track>, Error> {
        self.queue.get_queue().await
    }

    async fn handle_player_msg(&self, msg: PlayerCommand) -> Result<(), Error> {
        match msg {
            PlayerCommand::Play(track) => {
                let stream_url = self.core.stream_url(&track).await?;
                self.backend.set_track(&track, stream_url)?
            },
            PlayerCommand::Stop => self.backend.set_state(PlayerState::Stop)?,
        };
        Ok(())
    }

    async fn handle_queue_msg(&self, msg: QueueCommand) -> Result<(), Error> {
        match msg {
            QueueCommand::Next => self.queue.next().await?,
        };
        Ok(())
    }

    pub fn observe(&self) -> Receiver<PlayerEvent> {
        self.bus.observe()
    }
}

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Player")
            .field("display_name", &self.display_name)
            .field("backend", &self.backend)
            .field("queue", &self.queue)
            .finish()
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
