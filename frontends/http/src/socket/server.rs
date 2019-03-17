use actix::prelude::*;
use failure::Error;
use socket::messages;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default)]
pub struct SocketServer {
    pub sessions: HashMap<String, Recipient<messages::Message>>,
}

impl SocketServer {
    fn broadcast(&self, msg: messages::Message) -> Result<(), Error> {
        debug!("broadcast msg {:?}", &msg);
        for (_, addr) in self.sessions.iter() {
            let _ = addr.do_send(msg.clone());
        }
        Ok(())
    }
}

impl Handler<messages::Message> for SocketServer {
    type Result = ();

    fn handle(&mut self, msg: messages::Message, _: &mut Context<Self>) {
        self.broadcast(msg).unwrap();
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

impl Actor for SocketServer {
    type Context = Context<Self>;
}
