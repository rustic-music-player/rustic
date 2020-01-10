use actix_web::{get, web, Responder, Result};
use app::ApiState;

use handler::providers as providers_handler;
use rustic_core::Provider;

#[derive(Deserialize)]
pub struct NavigateQuery {
    path: String,
}

#[derive(Deserialize)]
pub struct ProviderParams {
    provider: Provider
}

#[get("/providers")]
pub fn get_providers(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    let providers = providers_handler::get_providers(&rustic);

    Ok(web::Json(providers))
}

#[get("/providers/{provider}/navigate")]
pub fn navigate(data: web::Data<ApiState>,
                params: web::Path<ProviderParams>,
                query: web::Query<NavigateQuery>) -> Result<impl Responder> {
    let rustic = &data.app;
    let folder = providers_handler::navigate(&rustic, params.provider, &query.path)?;

    Ok(web::Json(folder))
}