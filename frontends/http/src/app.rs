use actix_web::http::Method;
use actix_web::{fs, middleware, server, App, Result};
use controller;
use rustic_core::Rustic;
use socket::build_socket_app;
use std::sync::Arc;
use HttpConfig;

fn build_api_app(app: Arc<Rustic>) -> App<Arc<Rustic>> {
    App::with_state(app)
        .prefix("/api")
        .middleware(middleware::Logger::default())
        .resource("/library/albums", |r| {
            r.method(Method::GET).f(controller::library::get_albums)
        }).resource("/library/albums/{album_id}", |r| {
            r.method(Method::GET).with(controller::library::get_album)
        }).resource("/library/artists", |r| {
            r.method(Method::GET).f(controller::library::get_artists)
        }).resource("/library/playlists", |r| {
            r.method(Method::GET).f(controller::library::get_playlists)
        }).resource("/library/tracks", |r| {
            r.method(Method::GET).f(controller::library::get_tracks)
        }).resource("/queue", |r| {
            r.method(Method::GET).f(controller::queue::fetch)
        }).resource("/queue/clear", |r| {
            r.method(Method::POST).f(controller::queue::clear)
        }).resource("/queue/playlist/{playlist_id}", |r| {
            r.method(Method::POST)
                .with(controller::queue::queue_playlist)
        }).resource("/queue/track/{track_id}", |r| {
            r.method(Method::POST).with(controller::queue::queue_track)
        }).resource("/queue/{track_id}", |r| {
            r.method(Method::POST).with(controller::queue::queue_track)
        }).resource("/player", |r| {
            r.method(Method::GET).f(controller::player::player_state)
        }).resource("/player/play", |r| {
            r.method(Method::POST).f(controller::player::control_play)
        }).resource("/player/pause", |r| {
            r.method(Method::POST).f(controller::player::control_pause)
        }).resource("/player/next", |r| {
            r.method(Method::POST).f(controller::player::control_next)
        }).resource("/player/prev", |r| {
            r.method(Method::POST).f(controller::player::control_prev)
        }).resource("/search", |r| {
            r.method(Method::GET).with(controller::search::search)
        })
}

fn build_static_app() -> Result<App<()>> {
    Ok(App::new()
        .middleware(middleware::Logger::default())
        .handler("/cache", fs::StaticFiles::new(".cache")?))
}

pub fn start(config: &HttpConfig, app: Arc<Rustic>) -> Result<()> {
    server::new(move || {
        vec![
            build_socket_app(Arc::clone(&app)).boxed(),
            build_api_app(Arc::clone(&app)).boxed(),
            build_static_app().unwrap().boxed(),
        ]
    }).bind(format!("{}:{}", config.ip, config.port))?
    .run();
    Ok(())
}
