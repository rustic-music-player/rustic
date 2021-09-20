use actix_web::{delete, error, get, post, web, HttpResponse, Responder, Result};
use futures::stream::StreamExt;
use serde::Deserialize;
use serde_qs::actix::QsQuery;

use rustic_api::models::{CoverArtModel, ProviderTypeModel};

use crate::app::ApiClient;
use rustic_api::cursor::Cursor;

use super::failure_to_response;

#[derive(Deserialize)]
pub struct EntityQuery {
    cursor: String,
}

#[derive(Deserialize)]
pub struct GetEntitiesQuery {
    providers: Option<Vec<ProviderTypeModel>>,
}

#[get("/library/albums/{cursor}")]
pub async fn get_album(
    client: web::Data<ApiClient>,
    params: web::Path<EntityQuery>,
) -> Result<impl Responder> {
    let album = client.get_album(&params.cursor).await.map_err(failure_to_response)?;

    match album {
        Some(album) => Ok(web::Json(album)),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[get("/library/albums")]
pub async fn get_albums(
    client: web::Data<ApiClient>,
    params: QsQuery<GetEntitiesQuery>,
) -> Result<impl Responder> {
    let params = params.into_inner();
    let albums = client.get_albums(params.providers).await.map_err(failure_to_response)?;

    Ok(web::Json(albums))
}

#[post("/library/albums/{cursor}")]
pub async fn add_album(
    client: web::Data<ApiClient>,
    params: web::Path<EntityQuery>,
) -> Result<impl Responder> {
    let cursor = params.into_inner().cursor;
    client.add_to_library(Cursor::Album(cursor)).await.map_err(failure_to_response)?;

    Ok(web::HttpResponse::NoContent())
}

#[delete("/library/albums/{cursor}")]
pub async fn remove_album(
    client: web::Data<ApiClient>,
    params: web::Path<EntityQuery>,
) -> Result<impl Responder> {
    let cursor = params.into_inner().cursor;
    client.remove_from_library(Cursor::Album(cursor)).await.map_err(failure_to_response)?;

    Ok(web::HttpResponse::NoContent())
}

#[get("/library/artists")]
pub async fn get_artists(client: web::Data<ApiClient>) -> Result<impl Responder> {
    let artists = client.get_artists().await.map_err(failure_to_response)?;

    Ok(web::Json(artists))
}

#[get("/library/artists/{cursor}")]
pub async fn get_artist(
    client: web::Data<ApiClient>,
    params: web::Path<EntityQuery>,
) -> Result<impl Responder> {
    let artist = client.get_artist(&params.cursor).await.map_err(failure_to_response)?;

    match artist {
        Some(artist) => Ok(web::Json(artist)),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/library/artists/{cursor}")]
pub async fn add_artist(
    client: web::Data<ApiClient>,
    params: web::Path<EntityQuery>,
) -> Result<impl Responder> {
    let cursor = params.into_inner().cursor;
    client.add_to_library(Cursor::Artist(cursor)).await.map_err(failure_to_response)?;

    Ok(web::HttpResponse::NoContent())
}

#[get("/library/playlists")]
pub async fn get_playlists(
    client: web::Data<ApiClient>,
    params: QsQuery<GetEntitiesQuery>,
) -> Result<impl Responder> {
    let params = params.into_inner();
    let playlists = client.get_playlists(params.providers).await.map_err(failure_to_response)?;

    Ok(web::Json(playlists))
}

#[get("/library/playlists/{cursor}")]
pub async fn get_playlist(
    client: web::Data<ApiClient>,
    params: web::Path<EntityQuery>,
) -> Result<impl Responder> {
    let playlist = client.get_playlist(&params.cursor).await.map_err(failure_to_response)?;

    match playlist {
        Some(playlist) => Ok(web::Json(playlist)),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/library/playlists/{cursor}")]
pub async fn add_playlist(
    client: web::Data<ApiClient>,
    params: web::Path<EntityQuery>,
) -> Result<impl Responder> {
    let cursor = params.into_inner().cursor;
    client.add_to_library(Cursor::Playlist(cursor)).await.map_err(failure_to_response)?;

    Ok(web::HttpResponse::NoContent())
}

#[get("/library/tracks")]
pub async fn get_tracks(
    client: web::Data<ApiClient>,
    params: QsQuery<GetEntitiesQuery>,
) -> Result<impl Responder> {
    let params = params.into_inner();
    let tracks = client.get_tracks(params.providers).await.map_err(failure_to_response)?;

    Ok(web::Json(tracks))
}

#[get("/tracks/{cursor}")]
pub async fn get_track(
    client: web::Data<ApiClient>,
    params: web::Path<EntityQuery>,
) -> Result<impl Responder> {
    let track = client.get_track(&params.cursor).await.map_err(failure_to_response)?;

    match track {
        Some(track) => Ok(web::Json(track)),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[post("/library/tracks/{cursor}")]
pub async fn add_track(
    client: web::Data<ApiClient>,
    params: web::Path<EntityQuery>,
) -> Result<impl Responder> {
    let cursor = params.into_inner().cursor;
    client.add_to_library(Cursor::Track(cursor)).await.map_err(failure_to_response)?;

    Ok(web::HttpResponse::NoContent())
}

fn get_cover_art(cover_art: Option<CoverArtModel>) -> Result<impl Responder> {
    match cover_art {
        Some(CoverArtModel::Data { data, mime_type }) => {
            let stream = data.map(|d| Ok(d.into()));
            let response = HttpResponse::Ok()
                .content_type(mime_type)
                .streaming::<_, actix_web::Error>(stream);
            Ok(response)
        }
        Some(CoverArtModel::Url(url)) => {
            let response = HttpResponse::Found().append_header(("Location", url)).finish();
            Ok(response)
        }
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[get("/albums/{cursor}/coverart")]
pub async fn get_album_cover_art(
    client: web::Data<ApiClient>,
    params: web::Path<EntityQuery>,
) -> Result<impl Responder> {
    let cover_art = client
        .get_thumbnail(Cursor::Album(params.cursor.clone()))
        .await.map_err(failure_to_response)?;
    get_cover_art(cover_art)
}

#[get("/artists/{cursor}/coverart")]
pub async fn get_artist_cover_art(
    client: web::Data<ApiClient>,
    params: web::Path<EntityQuery>,
) -> Result<impl Responder> {
    let cover_art = client
        .get_thumbnail(Cursor::Artist(params.cursor.clone()))
        .await.map_err(failure_to_response)?;
    get_cover_art(cover_art)
}

#[get("/tracks/{cursor}/coverart")]
pub async fn get_track_cover_art(
    client: web::Data<ApiClient>,
    params: web::Path<EntityQuery>,
) -> Result<impl Responder> {
    let cover_art = client
        .get_thumbnail(Cursor::Track(params.cursor.clone()))
        .await.map_err(failure_to_response)?;
    get_cover_art(cover_art)
}
