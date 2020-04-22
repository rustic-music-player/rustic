use std::sync::Arc;
use std::time::Duration;

use log::debug;
use actix::prelude::*;
use crossbeam_channel::Receiver;
use futures::{Async, Poll, Stream};

use crate::cursor::to_cursor;
use rustic_core::player::Player;
use rustic_core::{PlayerEvent, PlayerState};
use crate::socket::{messages, SocketServer};
use rustic_api::models::TrackModel;

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
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.receiver
            .try_recv()
            .map_err(|_| ())
            .and_then(|event| match event {
                PlayerEvent::StateChanged(state) => {
                    debug!("received new playing state");
                    let msg =
                        messages::PlayerMessageData::PlayerStateChanged(state == PlayerState::Play);
                    Ok(messages::Message::PlayerMessage(messages::PlayerMessage {
                        message: msg,
                        player_cursor: to_cursor(&self.id),
                    }))
                }
                PlayerEvent::TrackChanged(track) => {
                    debug!("received currently playing track");
                    let model = TrackModel::from(track);
                    let msg = messages::PlayerMessageData::CurrentlyPlayingChanged(Some(model));
                    Ok(messages::Message::PlayerMessage(messages::PlayerMessage {
                        message: msg,
                        player_cursor: to_cursor(&self.id),
                    }))
                }
                PlayerEvent::QueueUpdated(queue) => {
                    debug!("received new queue");
                    let models = queue
                        .into_iter()
                        .map(TrackModel::from)
                        .collect();
                    let msg = messages::PlayerMessageData::QueueUpdated(models);
                    Ok(messages::Message::PlayerMessage(messages::PlayerMessage {
                        message: msg,
                        player_cursor: to_cursor(&self.id),
                    }))
                }
                msg => {
                    debug!("unexpected msg {:?}", msg);
                    Err(())
                }
            })
            .map(|msg| Async::Ready(Some(msg)))
            .or_else(|_| Ok(Async::NotReady))
    }
}
