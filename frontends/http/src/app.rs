use std::fs::create_dir_all;
use std::sync::Arc;

use actix::{Addr, System};
use actix_files::{Files, NamedFile};
use actix_web::{middleware, web, App, FromRequest, HttpServer, Responder, Result, Scope};

use crate::controller;
use crate::controller::search::SearchQuery;
use crate::socket::{create_socket_server, socket_service, SocketServer};
use crate::HttpConfig;
pub use rustic_api::ApiClient;
use rustic_api::RusticApiClient;
use rustic_core::Rustic;
use serde_qs::actix::QsQuery;
use serde_qs::Config;

pub struct ApiState {
    pub app: Arc<Rustic>,
    pub client: ApiClient,
}

fn build_api(app: Arc<Rustic>, client: ApiClient, ws_server: Addr<SocketServer>) -> Scope {
    web::scope("/api")
        .data(Arc::clone(&client))
        .data(ApiState { app, client })
        .data(QsQuery::<SearchQuery>::configure(|cfg| {
            cfg.qs_config(Config::new(2, false))
        }))
        .service(controller::library::get_albums)
        .service(controller::library::get_album)
        .service(controller::library::get_artists)
        .service(controller::library::get_artist)
        .service(controller::library::get_playlists)
        .service(controller::library::get_playlist)
        .service(controller::library::get_tracks)
        .service(controller::library::get_track)
        .service(controller::library::get_track_cover_art)
        .service(controller::queue::fetch)
        .service(controller::queue::clear)
        .service(controller::queue::queue_playlist)
        .service(controller::queue::queue_album)
        .service(controller::queue::queue_track)
        .service(controller::queue::remove_item)
        .service(controller::queue::reorder_item)
        .service(controller::search::search)
        .service(controller::search::open)
        .service(controller::player::get_players)
        .service(controller::player::default_player_state)
        .service(controller::player::default_control_next)
        .service(controller::player::control_next)
        .service(controller::player::default_control_prev)
        .service(controller::player::control_prev)
        .service(controller::player::default_control_play)
        .service(controller::player::control_play)
        .service(controller::player::default_control_pause)
        .service(controller::player::control_pause)
        .service(controller::player::default_set_volume)
        .service(controller::player::set_volume)
        .service(controller::extensions::get_extensions)
        .service(controller::providers::get_providers)
        .service(controller::providers::navigate)
        .service(controller::providers::get_available_providers)
        .service(controller::providers::provider_token_auth)
        .service(controller::providers::provider_basic_auth)
        .service(socket_service(ws_server))
}

async fn index() -> Result<impl Responder> {
    let file = NamedFile::open("static/index.html")?;

    Ok(file)
}

pub fn start(
    config: &HttpConfig,
    app: Arc<Rustic>,
    client: Arc<Box<dyn RusticApiClient>>,
) -> Result<()> {
    create_dir_all(&config.static_files)?;
    let sys = System::new("rustic-http-frontend");

    let ws_server = create_socket_server(Arc::clone(&app));

    let static_file_dir = config.static_files.clone();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(build_api(
                Arc::clone(&app),
                client.clone(),
                ws_server.clone(),
            ))
            .service(Files::new("/cache", ".cache"))
            .service(
                Files::new("", &static_file_dir)
                    .index_file("index.html")
                    .default_handler(web::get().to(index)),
            )
    })
    .bind(format!("{}:{}", config.ip, config.port))?
    .run();

    sys.run()?;
    Ok(())
}
