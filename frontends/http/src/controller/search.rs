use actix_web::{error, get, web, Responder};
use serde::{Deserialize};
use log::trace;

use crate::app::ApiState;
use crate::cursor::from_cursor;
use crate::handler::search as search_handler;
use serde_qs::actix::QsQuery;
use rustic_api::models::ProviderType;

#[derive(Deserialize)]
pub struct SearchQuery {
    query: String,
    providers: Option<Vec<ProviderType>>,
}

#[derive(Deserialize)]
pub struct OpenParams {
    url: String,
}

#[get("/search")]
pub async fn search(
    data: web::Data<ApiState>,
    params: QsQuery<SearchQuery>,
) -> Result<impl Responder, error::Error> {
    trace!("search {}", &params.query);
    let result = data.client.search(&params.query, params.providers.as_ref()).await?;
    Ok(web::Json(result))
}

#[get("/open/{url}")]
pub async fn open(
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
