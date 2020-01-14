use std::sync::Arc;
use std::time::Duration;

use actix::prelude::*;
use crossbeam_channel::Receiver;
use futures::{Async, Poll, Stream};

use rustic_core::{PlayerEvent, PlayerState, Rustic};
use socket::{messages, SocketServer};
use viewmodels::TrackModel;

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
    receiver: Receiver<PlayerEvent>,
}

impl PlayerEvents {
    pub fn new(app: Arc<Rustic>) -> PlayerEvents {
        let player = app
            .get_default_player()
            .ok_or_else(|| format_err!("Missing default player"))
            .unwrap();
        let receiver = player.observe();

        PlayerEvents { receiver }
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
                    Ok(messages::Message::PlayerStateChanged(
                        state == PlayerState::Play,
                    ))
                }
                PlayerEvent::TrackChanged(track) => {
                    debug!("received currently playing track");
                    let model = TrackModel::new(track);
                    Ok(messages::Message::CurrentlyPlayingChanged(Some(model)))
                }
                PlayerEvent::QueueUpdated(queue) => {
                    debug!("received new queue");
                    let models = queue
                        .into_iter()
                        .map(|track| TrackModel::new(track))
                        .collect();
                    Ok(messages::Message::QueueUpdated(models))
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
