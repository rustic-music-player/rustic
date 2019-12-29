use actix_web::{error, get, web, Responder};
use app::ApiState;
use handler::search as search_handler;

#[derive(Deserialize)]
pub struct SearchQuery {
    query: String,
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
