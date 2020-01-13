use actix_web::{error, get, Responder, web};

use app::ApiState;
use cursor::from_cursor;
use handler::search as search_handler;

#[derive(Deserialize)]
pub struct SearchQuery {
    query: String,
}

#[derive(Deserialize)]
pub struct OpenParams {
    url: String
}

#[get("/search")]
pub fn search(
    data: web::Data<ApiState>,
    params: web::Query<SearchQuery>,
) -> Result<impl Responder, error::Error> {
    let rustic = &data.app;
    trace!("search {}", &params.query);
    let result = search_handler::search(&params.query, &rustic)?;
    Ok(web::Json(result))
}

#[get("/open/{url}")]
pub fn open(
    data: web::Data<ApiState>,
    params: web::Path<OpenParams>
) -> Result<impl Responder, error::Error> {
    let rustic = &data.app;
    let url = from_cursor(&params.url)?;

    match search_handler::open(url, rustic)? {
        Some(result) => Ok(web::Json(result)),
        None => Err(error::ErrorNotFound("no results"))
    }
}