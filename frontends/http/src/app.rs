use std::fs::create_dir_all;
use std::sync::Arc;

use actix::Addr;
use actix_files::{Files, NamedFile};
use actix_web::{middleware, web, App, HttpServer, Responder, Result, Scope};

use crate::controller;
use crate::socket::{create_socket_server, socket_service, SocketServer};
use crate::HttpConfig;
pub use rustic_api::ApiClient;
use rustic_core::Rustic;
use serde_qs::actix::QsQueryConfig;
use serde_qs::Config;

fn build_api(client: ApiClient, ws_server: Addr<SocketServer>) -> Scope {
    web::scope("/api")
        .app_data(web::Data::new(Arc::clone(&client)))
        .app_data(QsQueryConfig::default().qs_config(Config::new(2, false)))
        .service(controller::library::get_albums)
        .service(controller::library::get_album)
        .service(controller::library::add_album)
        .service(controller::library::remove_album)
        .service(controller::library::get_artists)
        .service(controller::library::get_artist)
        .service(controller::library::add_artist)
        .service(controller::library::get_playlists)
        .service(controller::library::get_playlist)
        .service(controller::library::add_playlist)
        .service(controller::library::get_tracks)
        .service(controller::library::get_track)
        .service(controller::library::add_track)
        .service(controller::library::get_artist_cover_art)
        .service(controller::library::get_album_cover_art)
        .service(controller::library::get_track_cover_art)
        .service(controller::playlists::add_playlist)
        .service(controller::playlists::remove_playlist)
        .service(controller::playlists::add_track_to_playlist)
        .service(controller::playlists::remove_track_from_playlist)
        .service(controller::queue::fetch_default)
        .service(controller::queue::fetch)
        .service(controller::queue::clear_default)
        .service(controller::queue::clear)
        .service(controller::queue::queue_playlist_default)
        .service(controller::queue::queue_playlist)
        .service(controller::queue::queue_album_default)
        .service(controller::queue::queue_album)
        .service(controller::queue::queue_track_default)
        .service(controller::queue::queue_track)
        .service(controller::queue::select_item_default)
        .service(controller::queue::select_item)
        .service(controller::queue::remove_item_default)
        .service(controller::queue::remove_item)
        .service(controller::queue::reorder_item_default)
        .service(controller::queue::reorder_item)
        .service(controller::search::search)
        .service(controller::search::search_aggregated)
        .service(controller::search::search_library)
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
        .service(controller::player::default_set_repeat)
        .service(controller::player::set_repeat)
        .service(controller::extensions::get_extensions)
        .service(controller::extensions::enable_extension)
        .service(controller::extensions::disable_extension)
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

pub async fn start(config: &HttpConfig, app: Arc<Rustic>, client: ApiClient) -> Result<()> {
    create_dir_all(&config.static_files)?;

    let ws_server = create_socket_server(Arc::clone(&app), Arc::clone(&client));

    let static_file_dir = config.static_files.clone();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(build_api(client.clone(), ws_server.clone()))
            .service(Files::new("/cache", ".cache"))
            .service(
                Files::new("", &static_file_dir)
                    .index_file("index.html")
                    .default_handler(web::get().to(index)),
            )
    })
    .bind(format!("{}:{}", config.ip, config.port))?
    .run()
    .await?;

    Ok(())
}
