use actix_web::{error, get, web, Responder, Result};
use app::ApiState;
use handler::library as library_handler;

#[derive(Deserialize)]
pub struct GetAlbumQuery {
    album_id: usize,
}

//pub fn library_controller() -> Scope {
//    web::scope("/library")
//        .service(get_album)
//        .service(get_albums)
//        .service(get_artists)
//        .service(get_playlists)
//        .service(get_tracks)
//}

#[get("/library/albums/{album_id}")]
pub fn get_album(
    data: web::Data<ApiState>,
    params: web::Path<GetAlbumQuery>,
) -> Result<impl Responder> {
    let rustic = &data.app;
    let album = library_handler::fetch_album(params.album_id, rustic)?;
    match album {
        Some(album) => Ok(web::Json(album)),
        None => Err(error::ErrorNotFound("Not Found")),
    }
}

#[get("/library/albums")]
pub fn get_albums(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    let albums = library_handler::fetch_albums(rustic)?;

    Ok(web::Json(albums))
}

#[get("/library/artists")]
pub fn get_artists(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    let artists = library_handler::fetch_artists(&rustic)?;

    Ok(web::Json(artists))
}

#[get("/library/playlists")]
pub fn get_playlists(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    let playlists = library_handler::fetch_playlists(&rustic)?;

    Ok(web::Json(playlists))
}

#[get("/library/tracks")]
pub fn get_tracks(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    let tracks = library_handler::fetch_tracks(&rustic)?;

    Ok(web::Json(tracks))
}
