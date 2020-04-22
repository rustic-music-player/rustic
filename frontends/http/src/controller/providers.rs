use actix_web::{get, web, Responder, Result};
use serde::{Deserialize};

use crate::app::ApiState;
use crate::handler::providers as providers_handler;
use rustic_core::Provider;

#[derive(Deserialize)]
pub struct NavigateQuery {
    path: String,
}

#[derive(Deserialize)]
pub struct ProviderParams {
    provider: Provider,
}

#[derive(Deserialize)]
pub struct AuthRedirectParams {
    code: String,
    state: String,
}

#[get("/providers")]
pub fn get_providers(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    let providers = providers_handler::get_providers(&rustic);

    Ok(web::Json(providers))
}

#[get("/providers/{provider}/navigate")]
pub fn navigate(
    data: web::Data<ApiState>,
    params: web::Path<ProviderParams>,
    query: web::Query<NavigateQuery>,
) -> Result<impl Responder> {
    let rustic = &data.app;
    let folder = providers_handler::navigate(&rustic, params.provider, &query.path)?;

    Ok(web::Json(folder))
}

#[get("/providers/available")]
pub fn get_available_providers(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    let providers = providers_handler::get_available_providers(&rustic);

    Ok(web::Json(providers))
}

#[get("/providers/{provider}/auth/redirect")]
pub fn provider_token_auth(
    query: web::Query<AuthRedirectParams>,
    params: web::Path<ProviderParams>,
    data: web::Data<ApiState>,
) -> Result<impl Responder> {
    let rustic = &data.app;
    providers_handler::authenticate(&rustic, params.provider, &query.code)?;

    Ok(web::HttpResponse::Ok().body(
        "<html><body>You can close this window now<script>window.close()</script></body></html>",
    ))
}
