use actix_web::{error, get, web, HttpResponse, Responder, Result};
use serde::{Deserialize};

use crate::app::ApiState;
use crate::handler::library as library_handler;
use rustic_core::provider::CoverArt;

#[derive(Deserialize)]
pub struct GetEntityQuery {
    cursor: String,
}

//pub fn library_controller() -> Scope {
//    web::scope("/library")
//        .service(get_album)
//        .service(get_albums)
//        .service(get_artists)
//        .service(get_playlists)
//        .service(get_tracks)
//}

#[get("/library/albums/{cursor}")]
pub async fn get_album(
    data: web::Data<ApiState>,
    params: web::Path<GetEntityQuery>,
) -> Result<impl Responder> {
    let album = data.client.get_album(&params.cursor).await?;

    match album {
        Some(album) => Ok(web::Json(album)),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[get("/library/albums")]
pub async fn get_albums(data: web::Data<ApiState>) -> Result<impl Responder> {
    let albums = data.client.get_albums().await?;

    Ok(web::Json(albums))
}

#[get("/library/artists")]
pub async fn get_artists(data: web::Data<ApiState>) -> Result<impl Responder> {
    let artists = data.client.get_artists().await?;

    Ok(web::Json(artists))
}

#[get("/library/playlists")]
pub async fn get_playlists(data: web::Data<ApiState>) -> Result<impl Responder> {
    let playlists = data.client.get_playlists().await?;

    Ok(web::Json(playlists))
}

#[get("/library/playlists/{cursor}")]
pub async fn get_playlist(
    data: web::Data<ApiState>,
    params: web::Path<GetEntityQuery>,
) -> Result<impl Responder> {
    let playlist = data.client.get_playlist(&params.cursor).await?;

    match playlist {
        Some(playlist) => Ok(web::Json(playlist)),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[get("/library/tracks")]
pub async fn get_tracks(data: web::Data<ApiState>) -> Result<impl Responder> {
    let tracks = data.client.get_tracks().await?;

    Ok(web::Json(tracks))
}

#[get("/tracks/{cursor}")]
pub async fn get_track(
    data: web::Data<ApiState>,
    params: web::Path<GetEntityQuery>,
) -> Result<impl Responder> {
    let track = data.client.get_track(&params.cursor).await?;

    match track {
        Some(track) => Ok(web::Json(track)),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[get("/tracks/{cursor}/coverart")]
pub async fn get_track_cover_art(
    data: web::Data<ApiState>,
    params: web::Path<GetEntityQuery>,
) -> Result<impl Responder> {
    let rustic = &data.app;
    let cover_art = library_handler::get_coverart_for_track(&params.cursor, &rustic)?;
    match cover_art {
        Some(CoverArt::Data { data, mime_type }) => {
            let response = HttpResponse::Ok().content_type(mime_type).body(data);
            Ok(response)
        }
        Some(CoverArt::Url(url)) => {
            let response = HttpResponse::Found().header("Location", url).finish();
            Ok(response)
        }
        None => Err(error::ErrorNotFound("Not Found")),
    }
}
