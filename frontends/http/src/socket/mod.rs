use actix::{Addr, Arbiter};
use actix_web::{http::Method, middleware, ws, App, Error, HttpRequest, HttpResponse};
use rustic_core::{
    player::{PlayerEvent, PlayerState},
    Rustic,
};
use std::sync::Arc;
use std::thread;
use viewmodels::TrackModel;
use failure;

mod messages;
mod server;
mod session;

pub struct SocketState {
    pub rustic: Arc<Rustic>,
    pub addr: Addr<server::SocketServer>,
}

pub fn build_socket_app(rustic: Arc<Rustic>) -> App<SocketState> {
    let addr = Arbiter::start(|_| server::SocketServer::default());
    let state = SocketState {
        rustic: rustic.clone(),
        addr: addr.clone(),
    };
    thread::spawn(move || {
        let player = rustic.get_default_player().ok_or(format_err!("Missing default player")).unwrap();

        loop {
            let event = player.observe().recv();

            match event {
                Some(PlayerEvent::StateChanged(state)) => {
                    debug!("received new playing state");
                    addr.do_send(messages::Message::PlayerStateChanged(
                        state == PlayerState::Play,
                    ));
                }
                Some(PlayerEvent::TrackChanged(track)) => {
                    debug!("received currently playing track");
                    let model = TrackModel::new_with_joins(track, &rustic).ok();
                    addr.do_send(messages::Message::CurrentlyPlayingChanged(model));
                }
                Some(PlayerEvent::QueueUpdated(queue)) => {
                    debug!("received new queue");
                    let models = queue.into_iter()
                        .map(|track| TrackModel::new_with_joins(track, &rustic))
                        .collect::<Result<Vec<TrackModel>, failure::Error>>()
                        .unwrap();
                    addr.do_send(messages::Message::QueueUpdated(models));
                }
                msg => debug!("unexpected msg {:?}", msg),
            }
        }
    });
    App::with_state(state)
        .middleware(middleware::Logger::default())
        .prefix("/api/socket")
        .resource("", |r| r.method(Method::GET).f(open))
}

pub fn open(req: &HttpRequest<SocketState>) -> Result<HttpResponse, Error> {
    debug!("connection");
    ws::start(req, session::SocketSession::default())
}
