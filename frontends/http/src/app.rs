use std::fs::create_dir_all;
use std::sync::Arc;

use actix::{Addr, System};
use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer, Result, Scope};

use controller;
use rustic_core::Rustic;
use socket::{create_socket_server, socket_service, SocketServer};
use HttpConfig;

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
        .service(controller::library::get_track)
        .service(controller::library::get_track_cover_art)
        .service(controller::queue::fetch)
        .service(controller::queue::clear)
        .service(controller::queue::queue_playlist)
        .service(controller::queue::queue_track)
        .service(controller::search::search)
        .service(controller::search::open)
        .service(controller::player::get_players)
        .service(controller::player::player_state)
        .service(controller::player::control_next)
        .service(controller::player::control_prev)
        .service(controller::player::control_play)
        .service(controller::player::control_pause)
        .service(controller::extensions::get_extensions)
        .service(controller::providers::get_providers)
        .service(controller::providers::navigate)
        .service(socket_service(ws_server))
}

pub fn start(config: &HttpConfig, app: Arc<Rustic>) -> Result<()> {
    create_dir_all(&config.static_files)?;
    let sys = System::new("rustic-http-frontend");

    let ws_server = create_socket_server(Arc::clone(&app));

    let static_file_dir = config.static_files.clone();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(build_api(Arc::clone(&app), ws_server.clone()))
            .service(Files::new("/cache", ".cache"))
            .service(Files::new("", &static_file_dir).index_file("index.html"))
    })
    .bind(format!("{}:{}", config.ip, config.port))?
    .start();

    sys.run()?;
    Ok(())
}
