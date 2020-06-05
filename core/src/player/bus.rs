use crate::player::{PlayerCommand, QueueCommand};
use crate::PlayerEvent;
use failure::{format_err, Error};
use std::fmt;
use std::fmt::Debug;
use tokio::stream::StreamExt;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct PlayerBus {
    event_tx: crossbeam_channel::Sender<PlayerEvent>,
    event_rx: crossbeam_channel::Receiver<PlayerEvent>,
    player_tx: broadcast::Sender<PlayerCommand>,
    queue_tx: broadcast::Sender<QueueCommand>,
}

#[derive(Debug, Clone)]
pub enum PlayerBusCommand {
    Player(PlayerCommand),
    Queue(QueueCommand),
}

impl PlayerBus {
    pub fn new() -> Self {
        let (event_tx, event_rx) = crossbeam_channel::unbounded();
        let (player_tx, _) = broadcast::channel(1);
        let (queue_tx, _) = broadcast::channel(1);

        PlayerBus {
            event_rx,
            event_tx,
            player_tx,
            queue_tx,
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

    pub fn commands(&self) -> impl futures::Stream<Item = Result<PlayerBusCommand, Error>> {
        let player_rx = self
            .player_tx
            .subscribe()
            .into_stream()
            .map(|r| Ok(r.map(PlayerBusCommand::Player)?));
        let queue_rx = self
            .queue_tx
            .subscribe()
            .into_stream()
            .map(|r| Ok(r.map(PlayerBusCommand::Queue)?));

        player_rx.merge(queue_rx)
    }

    pub fn observe(&self) -> crossbeam_channel::Receiver<PlayerEvent> {
        self.event_rx.clone()
    }
}

impl Debug for PlayerBus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlayerBus").finish()
    }
}
