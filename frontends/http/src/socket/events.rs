use std::sync::Arc;
use std::task::Poll;
use std::time::Duration;

use actix::prelude::*;
use crossbeam_channel::{Receiver, TryRecvError};
use log::{debug, warn};

use crate::socket::{messages, SocketServer};
use rustic_api::cursor::to_cursor;
use rustic_api::models::{QueuedTrackModel, TrackModel};
use rustic_core::player::Player;
use rustic_core::{PlayerEvent, PlayerState};
use std::pin::Pin;

pub struct PlayerEventActor {
    addr: Addr<SocketServer>,
}

impl PlayerEventActor {
    pub fn new(addr: Addr<SocketServer>) -> Self {
        PlayerEventActor { addr }
    }
}

impl Actor for PlayerEventActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // TODO: this feels like a hack
        ctx.run_interval(Duration::new(1, 0), |act, _| {
            act.addr.do_send(messages::Ping)
        });
    }
}

pub struct PlayerEvents {
    id: String,
    receiver: Receiver<PlayerEvent>,
}

impl PlayerEvents {
    pub fn new(id: String, player: Arc<Player>) -> PlayerEvents {
        let receiver = player.observe();

        PlayerEvents { id, receiver }
    }
}

impl Stream for PlayerEvents {
    type Item = messages::Message;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match self.receiver.try_recv() {
            Ok(PlayerEvent::StateChanged(state)) => {
                debug!("received new playing state");
                let msg =
                    messages::PlayerMessageData::PlayerStateChanged(state == PlayerState::Play);
                let msg = messages::Message::PlayerMessage(messages::PlayerMessage {
                    message: msg,
                    player_cursor: to_cursor(&self.id),
                });
                Poll::Ready(Some(msg))
            }
            Ok(PlayerEvent::TrackChanged(track)) => {
                debug!("received currently playing track");
                let model = TrackModel::from(track);
                let msg = messages::PlayerMessageData::CurrentlyPlayingChanged(Some(model));
                let msg = messages::Message::PlayerMessage(messages::PlayerMessage {
                    message: msg,
                    player_cursor: to_cursor(&self.id),
                });
                Poll::Ready(Some(msg))
            }
            Ok(PlayerEvent::QueueUpdated(queue)) => {
                debug!("received new queue");
                let models = queue.into_iter().map(QueuedTrackModel::from).collect();
                let msg = messages::PlayerMessageData::QueueUpdated(models);
                let msg = messages::Message::PlayerMessage(messages::PlayerMessage {
                    message: msg,
                    player_cursor: to_cursor(&self.id),
                });
                Poll::Ready(Some(msg))
            }
            Ok(PlayerEvent::VolumeChanged(volume)) => {
                let msg = messages::PlayerMessageData::VolumeChanged(volume);
                let msg = messages::Message::PlayerMessage(messages::PlayerMessage {
                    message: msg,
                    player_cursor: to_cursor(&self.id),
                });
                Poll::Ready(Some(msg))
            }
            Ok(msg) => {
                warn!("unexpected msg {:?}", msg);
                Poll::Pending
            }
            Err(TryRecvError::Empty) => Poll::Pending,
            Err(TryRecvError::Disconnected) => Poll::Ready(None),
        }
    }
}
