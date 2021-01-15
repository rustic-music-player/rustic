use crate::player::{PlayerCommand, QueueCommand};
use crate::PlayerEvent;
use failure::{format_err, Error};
use futures::stream::{select, StreamExt};
use rustic_queue::{broadcast, Receiver, Sender};
use std::fmt;
use std::fmt::Debug;

#[derive(Clone)]
pub struct PlayerBus {
    event_tx: Sender<PlayerEvent>,
    event_rx: Receiver<PlayerEvent>,
    player_tx: Sender<PlayerCommand>,
    player_rx: Receiver<PlayerCommand>,
    queue_tx: Sender<QueueCommand>,
    queue_rx: Receiver<QueueCommand>,
}

#[derive(Debug, Clone)]
pub enum PlayerBusCommand {
    Player(PlayerCommand),
    Queue(QueueCommand),
}

impl PlayerBus {
    pub fn new() -> Self {
        let (event_tx, event_rx) = broadcast();
        let (player_tx, player_rx) = broadcast();
        let (queue_tx, queue_rx) = broadcast();

        PlayerBus {
            event_rx,
            event_tx,
            player_tx,
            player_rx,
            queue_tx,
            queue_rx,
        }
    }

    pub fn send_player_msg(&self, cmd: PlayerCommand) -> Result<(), Error> {
        self.player_tx
            .send(cmd)
            .map_err(|e| format_err!("Error sending player command {:?}", e))?;

        Ok(())
    }

    pub fn send_queue_msg(&self, cmd: QueueCommand) -> Result<(), Error> {
        self.queue_tx
            .send(cmd)
            .map_err(|e| format_err!("Error sending queue command {:?}", e))?;

        Ok(())
    }

    pub fn emit_event(&self, event: PlayerEvent) -> Result<(), Error> {
        self.event_tx.send(event)?;

        Ok(())
    }

    pub fn commands(&self) -> impl futures::Stream<Item = PlayerBusCommand> {
        let player_rx = self.player_rx.stream().map(PlayerBusCommand::Player);
        let queue_rx = self.queue_rx.stream().map(PlayerBusCommand::Queue);

        select(player_rx, queue_rx)
    }

    pub fn observe(&self) -> Receiver<PlayerEvent> {
        self.event_rx.clone()
    }
}

impl Debug for PlayerBus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlayerBus").finish()
    }
}
