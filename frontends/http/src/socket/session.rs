use actix::*;
use actix_web_actors::ws;
use serde_json;
use log::{debug, trace, warn};

use crate::socket::messages;
use crate::socket::server::SocketServer;

pub struct SocketSession {
    pub id: String,
    addr: Addr<SocketServer>,
}

impl SocketSession {
    pub fn new(addr: Addr<SocketServer>) -> SocketSession {
        SocketSession {
            id: String::new(),
            addr,
        }
    }
}

impl Actor for SocketSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(messages::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(messages::Disconnect {
            id: self.id.clone(),
        });
        Running::Stop
    }
}

impl Handler<messages::Message> for SocketSession {
    type Result = ();

    fn handle(&mut self, msg: messages::Message, ctx: &mut Self::Context) {
        let json = serde_json::to_string(&msg).unwrap();
        debug!("Sending Socket Message {}", json);
        ctx.text(json);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        trace!("WEBSOCKET MESSAGE: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Binary(_)) => warn!("Unexpected binary"),
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => {}
        }
    }
}
