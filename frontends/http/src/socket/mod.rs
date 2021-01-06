use crate::socket::events::PlayerEventActor;
pub use crate::socket::server::SocketServer;
use actix::{Actor, Addr};
use actix_web::{web, Error, HttpRequest, HttpResponse, Resource};
use actix_web_actors::ws;
use log::debug;
use rustic_core::Rustic;
use std::sync::Arc;
use rustic_api::ApiClient;

mod events;
mod messages;
mod server;
mod session;

pub fn create_socket_server(app: Arc<Rustic>, client: ApiClient) -> Addr<SocketServer> {
    let server = SocketServer::new(app, client).start();
    let _ = PlayerEventActor::new(server.clone()).start();
    server
}

pub fn socket_service(server: Addr<SocketServer>) -> Resource {
    web::resource("/socket").data(server).to(open)
}

pub async fn open(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::SocketServer>>,
) -> Result<HttpResponse, Error> {
    debug!("connection");
    ws::start(
        session::SocketSession::new(srv.get_ref().clone()),
        &req,
        stream,
    )
}
