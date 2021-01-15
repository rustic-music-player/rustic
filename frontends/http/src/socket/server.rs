use crate::socket::messages;
use actix::prelude::*;
use failure::Error;
use futures::StreamExt;
use rustic_api::cursor::to_cursor;
use rustic_api::models::{PlayerEventModel, QueueEventModel};
use rustic_api::ApiClient;
use rustic_core::Rustic;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct SocketServer {
    pub sessions: HashMap<String, Recipient<messages::Message>>,
    app: Arc<Rustic>,
    client: ApiClient,
}

impl SocketServer {
    pub fn new(app: Arc<Rustic>, client: ApiClient) -> SocketServer {
        log::trace!("Creating socket server");
        SocketServer {
            sessions: HashMap::default(),
            app,
            client,
        }
    }

    fn broadcast(&self, msg: messages::Message) -> Result<(), Error> {
        log::debug!(
            "broadcast msg {:?} to {} sockets",
            &msg,
            self.sessions.len()
        );
        for (_, addr) in self.sessions.iter() {
            let _ = addr.do_send(msg.clone());
        }
        Ok(())
    }
}

impl Actor for SocketServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let stream = self
            .client
            .observe_library()
            .map(|event| messages::Message::LibraryMessage(event.into()));

        ctx.add_message_stream(stream);

        let players = self.app.get_players();
        for (id, _) in players {
            let id2 = id.clone();
            let player_stream = self
                .client
                .observe_player(Some(&id))
                .filter_map(move |event| SocketServer::map_player_messages(id2.clone(), event));

            let queue_stream = self
                .client
                .observe_queue(Some(&id))
                .filter_map(move |event| SocketServer::map_queue_message(id.clone(), event));

            ctx.add_message_stream(player_stream);
            ctx.add_message_stream(queue_stream);
        }
    }
}

impl Handler<messages::Connect> for SocketServer {
    type Result = String;

    fn handle(&mut self, msg: messages::Connect, _: &mut Context<Self>) -> Self::Result {
        log::debug!("Socket connected");

        let id = Uuid::new_v4().to_string();
        self.sessions.insert(id.clone(), msg.addr);

        id
    }
}

impl Handler<messages::Disconnect> for SocketServer {
    type Result = ();

    fn handle(&mut self, msg: messages::Disconnect, _: &mut Context<Self>) {
        log::debug!("Socket disconnected");

        self.sessions.remove(&msg.id);
    }
}

impl Handler<messages::Message> for SocketServer {
    type Result = ();

    fn handle(&mut self, msg: messages::Message, _: &mut Context<Self>) {
        self.broadcast(msg).unwrap();
    }
}

impl Handler<messages::Ping> for SocketServer {
    type Result = ();

    fn handle(&mut self, _: messages::Ping, _: &mut Context<Self>) {}
}

impl SocketServer {
    async fn map_player_messages(id: String, event: PlayerEventModel) -> Option<messages::Message> {
        match event {
            PlayerEventModel::StateChanged(state) => {
                log::debug!("received new playing state");
                let msg = messages::PlayerMessageData::PlayerStateChanged(state);
                Some(messages::Message::PlayerMessage(messages::PlayerMessage {
                    message: msg,
                    player_cursor: to_cursor(&id),
                }))
            }
            PlayerEventModel::TrackChanged(track) => {
                log::debug!("received currently playing track");
                let msg = messages::PlayerMessageData::CurrentlyPlayingChanged(Some(track));
                Some(messages::Message::PlayerMessage(messages::PlayerMessage {
                    message: msg,
                    player_cursor: to_cursor(&id),
                }))
            }
            PlayerEventModel::VolumeChanged(volume) => {
                let msg = messages::PlayerMessageData::VolumeChanged(volume);
                Some(messages::Message::PlayerMessage(messages::PlayerMessage {
                    message: msg,
                    player_cursor: to_cursor(&id),
                }))
            }
            msg => {
                log::warn!("unexpected msg {:?}", msg);
                None
            }
        }
    }

    async fn map_queue_message(id: String, event: QueueEventModel) -> Option<messages::Message> {
        match event {
            QueueEventModel::QueueUpdated(queue) => {
                log::debug!("received new queue");
                let msg = messages::PlayerMessageData::QueueUpdated(queue);
                let msg = messages::Message::PlayerMessage(messages::PlayerMessage {
                    message: msg,
                    player_cursor: to_cursor(&id),
                });
                Some(msg)
            }
        }
    }
}
