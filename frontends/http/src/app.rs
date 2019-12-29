use std::sync::Arc;

use actix::{Addr, System};
use actix_files::Files;
use actix_web::{App, HttpServer, middleware, Result, Scope, web};

use controller;
use HttpConfig;
use rustic_core::Rustic;
use socket::{create_socket_server, socket_service, SocketServer};

pub struct ApiState {
    pub app: Arc<Rustic>,
}

fn build_api(app: Arc<Rustic>, ws_server: Addr<SocketServer>) -> Scope {
    web::scope("/api")
        .data(ApiState { app })
        .service(controller::library::get_albums)
        .service(controller::library::get_album)
        .service(controller::library::get_artists)
        .service(controller::library::get_playlists)
        .service(controller::library::get_playlist)
        .service(controller::library::get_tracks)
        .service(controller::queue::fetch)
        .service(controller::queue::clear)
        .service(controller::queue::queue_playlist)
        .service(controller::queue::queue_track)
        .service(controller::search::search)
        .service(controller::player::player_state)
        .service(controller::player::control_next)
        .service(controller::player::control_prev)
        .service(controller::player::control_play)
        .service(controller::player::control_pause)
        .service(socket_service(ws_server))
}

pub fn start(config: &HttpConfig, app: Arc<Rustic>) -> Result<()> {
    let sys = System::new("rustic-http-frontend");

    let ws_server = create_socket_server(Arc::clone(&app));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(build_api(Arc::clone(&app), ws_server.clone()))
            .service(Files::new("/cache", ".cache"))
    })
    .bind(format!("{}:{}", config.ip, config.port))?
    .start();

    sys.run()?;
    Ok(())
}
