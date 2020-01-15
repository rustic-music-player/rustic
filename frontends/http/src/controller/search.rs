use actix_web::{error, get, web, Responder};

use app::ApiState;
use cursor::from_cursor;
use handler::search as search_handler;
use rustic_core::Provider;
use serde_qs::actix::QsQuery;

#[derive(Deserialize)]
pub struct SearchQuery {
    query: String,
    providers: Option<Vec<Provider>>
}

#[derive(Deserialize)]
pub struct OpenParams {
    url: String,
}

#[get("/search")]
pub fn search(
    data: web::Data<ApiState>,
    params: QsQuery<SearchQuery>,
) -> Result<impl Responder, error::Error> {
    let rustic = &data.app;
    trace!("search {}", &params.query);
    let result = search_handler::search(&params.query, params.providers.as_ref(), &rustic)?;
    Ok(web::Json(result))
}

#[get("/open/{url}")]
pub fn open(
    data: web::Data<ApiState>,
    params: web::Path<OpenParams>,
) -> Result<impl Responder, error::Error> {
    let rustic = &data.app;
    let url = from_cursor(&params.url)?;

    match search_handler::open(url, rustic)? {
        Some(result) => Ok(web::Json(result)),
        None => Err(error::ErrorNotFound("no results")),
    }
}
