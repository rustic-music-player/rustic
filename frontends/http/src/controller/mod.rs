use actix_web::http::StatusCode;

pub mod extensions;
pub mod library;
pub mod player;
pub mod playlists;
pub mod providers;
pub mod queue;
pub mod search;

pub(crate) fn failure_to_response(err: failure::Error) -> impl actix_web::ResponseError {
    actix_web::error::InternalError::new(err, StatusCode::INTERNAL_SERVER_ERROR)
}
