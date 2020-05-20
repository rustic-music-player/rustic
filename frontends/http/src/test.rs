use actix_web::dev::*;
use actix_web::{App, Error, FromRequest};

use rustic_api::TestApiClient;

use crate::app::ApiClient;
use crate::controller;
use crate::controller::search::SearchQuery;
use actix_service::ServiceFactory;
use serde_qs::actix::QsQuery;
use serde_qs::Config;
use std::sync::Arc;

pub fn build_app<T, B>(app: App<T, B>, client: TestApiClient) -> App<T, B>
where
    B: MessageBody,
    T: ServiceFactory<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error,
        InitError = (),
    >,
{
    let client: ApiClient = Arc::new(Box::new(client));
    app.data(client)
        .data(QsQuery::<SearchQuery>::configure(|cfg| {
            cfg.qs_config(Config::new(2, false))
        }))
        .service(controller::library::get_albums)
        .service(controller::library::get_album)
        .service(controller::library::get_artists)
        .service(controller::library::get_playlists)
        .service(controller::library::get_playlist)
        .service(controller::library::get_tracks)
        .service(controller::library::get_track)
        .service(controller::queue::fetch)
        .service(controller::queue::clear)
        .service(controller::queue::queue_playlist)
        .service(controller::queue::queue_album)
        .service(controller::queue::queue_track)
        .service(controller::queue::remove_item)
        .service(controller::search::search)
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
        .service(controller::extensions::get_extensions)
}
