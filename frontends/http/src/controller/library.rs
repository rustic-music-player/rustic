use actix_web::{error, get, HttpResponse, Responder, Result, web};
use futures::stream::StreamExt;
use serde::Deserialize;
use serde_qs::actix::QsQuery;

use rustic_api::models::{CoverArtModel, ProviderTypeModel};

use crate::app::ApiClient;

#[derive(Deserialize)]
pub struct GetEntityQuery {
    cursor: String,
}

#[derive(Deserialize)]
pub struct GetEntitiesQuery {
    providers: Option<Vec<ProviderTypeModel>>,
}

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
pub async fn get_albums(client: web::Data<ApiClient>,
                        params: QsQuery<GetEntitiesQuery>) -> Result<impl Responder> {
    let params = params.into_inner();
    let albums = client.get_albums(params.providers).await?;

    Ok(web::Json(albums))
}

#[get("/library/artists")]
pub async fn get_artists(client: web::Data<ApiClient>) -> Result<impl Responder> {
    let artists = client.get_artists().await?;

    Ok(web::Json(artists))
}

#[get("/library/playlists")]
pub async fn get_playlists(client: web::Data<ApiClient>,
                           params: QsQuery<GetEntitiesQuery>) -> Result<impl Responder> {
    let params = params.into_inner();
    let playlists = client.get_playlists(params.providers).await?;

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
pub async fn get_tracks(client: web::Data<ApiClient>,
                        params: QsQuery<GetEntitiesQuery>) -> Result<impl Responder> {
    let params = params.into_inner();
    let tracks = client.get_tracks(params.providers).await?;

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
    client: web::Data<ApiClient>,
    params: web::Path<GetEntityQuery>,
) -> Result<impl Responder> {
    let cover_art = client.get_track_cover_art(&params.cursor).await?;
    match cover_art {
        Some(CoverArtModel::Data { data, mime_type }) => {
            let stream = data.map(|d| Ok(d.into()));
            let response = HttpResponse::Ok().content_type(mime_type).streaming::<_, failure::Error>(stream);
            Ok(response)
        }
        Some(CoverArtModel::Url(url)) => {
            let response = HttpResponse::Found().header("Location", url).finish();
            Ok(response)
        }
        None => Err(error::ErrorNotFound("Not Found")),
    }
}
