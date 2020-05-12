use actix_web::{error, get, web, HttpResponse, Responder, Result};
use serde::{Deserialize};

use crate::app::{ApiState, ApiClient};
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
    client: web::Data<ApiClient>,
    params: web::Path<GetEntityQuery>,
) -> Result<impl Responder> {
    let album = client.get_album(&params.cursor).await?;

    match album {
        Some(album) => Ok(web::Json(album)),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[get("/library/albums")]
pub async fn get_albums(client: web::Data<ApiClient>) -> Result<impl Responder> {
    let albums = client.get_albums().await?;

    Ok(web::Json(albums))
}

#[get("/library/artists")]
pub async fn get_artists(client: web::Data<ApiClient>) -> Result<impl Responder> {
    let artists = client.get_artists().await?;

    Ok(web::Json(artists))
}

#[get("/library/playlists")]
pub async fn get_playlists(client: web::Data<ApiClient>) -> Result<impl Responder> {
    let playlists = client.get_playlists().await?;

    Ok(web::Json(playlists))
}

#[get("/library/playlists/{cursor}")]
pub async fn get_playlist(
    client: web::Data<ApiClient>,
    params: web::Path<GetEntityQuery>,
) -> Result<impl Responder> {
    let playlist = client.get_playlist(&params.cursor).await?;

    match playlist {
        Some(playlist) => Ok(web::Json(playlist)),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[get("/library/tracks")]
pub async fn get_tracks(client: web::Data<ApiClient>) -> Result<impl Responder> {
    let tracks = client.get_tracks().await?;

    Ok(web::Json(tracks))
}

#[get("/tracks/{cursor}")]
pub async fn get_track(
    client: web::Data<ApiClient>,
    params: web::Path<GetEntityQuery>,
) -> Result<impl Responder> {
    let track = client.get_track(&params.cursor).await?;

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
