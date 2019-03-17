use actix::{Addr, fut};
use actix::prelude::*;
use actix_web::ws;
use serde_json;

use socket::{messages, SocketState};

#[derive(Default, Debug)]
pub struct SocketSession {
    pub id: String,
}

impl Actor for SocketSession {
    type Context = ws::WebsocketContext<Self, SocketState>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr: Addr<_> = ctx.address();
        ctx.state()
            .addr
            .send(messages::Connect {
                addr: addr.recipient(),
            }).into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ok(())
            }).wait(ctx);
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        // notify chat server
        ctx.state().addr.do_send(messages::Disconnect {
            id: self.id.clone(),
        });
        Running::Stop
    }
}

impl Handler<messages::Message> for SocketSession {
    type Result = ();

    fn handle(&mut self, msg: messages::Message, ctx: &mut Self::Context) {
        let json = serde_json::to_string(&msg).unwrap();
        ctx.text(json);
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for SocketSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        trace!("WEBSOCKET MESSAGE: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Pong(_) => {}
            ws::Message::Text(_text) => {}
            ws::Message::Binary(_) => warn!("Unexpected binary"),
            ws::Message::Close(_) => {
                ctx.stop();
            }
        }
    }
}
