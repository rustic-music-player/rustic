use std::time::Duration;

use actix::prelude::*;

use crate::socket::{messages, SocketServer};

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
