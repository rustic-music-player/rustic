use actix_web::{delete, post, put, web, Responder, Result};
use serde::Deserialize;

use rustic_api::ApiClient;
use super::failure_to_response;

#[derive(Deserialize)]
pub struct EntityQuery {
    #[serde(rename = "playlist")]
    cursor: String,
}

#[derive(Deserialize)]
pub struct TrackQuery {
    #[serde(rename = "track")]
    cursor: String,
}

#[derive(Deserialize)]
pub struct AddPlaylistRequest {
    name: String,
}

#[post("/library/playlists")]
pub async fn add_playlist(
    client: web::Data<ApiClient>,
    req: web::Json<AddPlaylistRequest>,
) -> Result<impl Responder> {
    let playlist = client.add_playlist(&req.name).await.map_err(failure_to_response)?;

    Ok(web::Json(playlist))
}

#[delete("/library/playlists/{playlist}")]
pub async fn remove_playlist(
    client: web::Data<ApiClient>,
    params: web::Path<EntityQuery>,
) -> Result<impl Responder> {
    client.remove_playlist(&params.cursor).await.map_err(failure_to_response)?;

    Ok(web::HttpResponse::NoContent())
}

#[put("/library/playlists/{playlist}/{track}")]
pub async fn add_track_to_playlist(
    client: web::Data<ApiClient>,
    playlist: web::Path<EntityQuery>,
    track: web::Path<TrackQuery>,
) -> Result<impl Responder> {
    client
        .add_track_to_playlist(&playlist.cursor, &track.cursor)
        .await.map_err(failure_to_response)?;

    Ok(web::HttpResponse::NoContent())
}

#[delete("/library/playlists/{playlist}/{track}")]
pub async fn remove_track_from_playlist(
    client: web::Data<ApiClient>,
    playlist: web::Path<EntityQuery>,
    track: web::Path<TrackQuery>,
) -> Result<impl Responder> {
    client
        .remove_track_from_playlist(&playlist.cursor, &track.cursor)
        .await.map_err(failure_to_response)?;

    Ok(web::HttpResponse::NoContent())
}
