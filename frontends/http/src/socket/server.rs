use crate::socket::events::PlayerEvents;
use crate::socket::messages;
use actix::prelude::*;
use failure::Error;
use log::{debug, trace};
use rustic_core::Rustic;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct SocketServer {
    pub sessions: HashMap<String, Recipient<messages::Message>>,
    app: Arc<Rustic>,
}

impl SocketServer {
    pub fn new(app: Arc<Rustic>) -> SocketServer {
        trace!("Creating socket server");
        SocketServer {
            sessions: HashMap::default(),
            app,
        }
    }

    fn broadcast(&self, msg: messages::Message) -> Result<(), Error> {
        debug!(
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
        let players = self.app.get_players();
        for (id, player) in players {
            let events = PlayerEvents::new(id, player);

            ctx.add_message_stream(events);
        }
    }
}

impl Handler<messages::Connect> for SocketServer {
    type Result = String;

    fn handle(&mut self, msg: messages::Connect, _: &mut Context<Self>) -> Self::Result {
        debug!("Socket connected");

        let id = Uuid::new_v4().to_string();
        self.sessions.insert(id.clone(), msg.addr);

        id
    }
}

impl Handler<messages::Disconnect> for SocketServer {
    type Result = ();

    fn handle(&mut self, msg: messages::Disconnect, _: &mut Context<Self>) {
        debug!("Socket disconnected");

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
