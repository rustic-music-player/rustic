use actix_web::{get, post, web, Responder, Result, HttpResponse};
use serde::Deserialize;

use rustic_api::models::{ProviderAuthModel, ProviderTypeModel};

use crate::app::ApiClient;
use super::failure_to_response;

#[derive(Deserialize)]
pub struct NavigateQuery {
    path: String,
}

#[derive(Deserialize)]
pub struct ProviderParams {
    provider: ProviderTypeModel,
}

#[get("/providers")]
pub async fn get_providers(client: web::Data<ApiClient>) -> Result<impl Responder> {
    let providers = client.get_providers().await.map_err(failure_to_response)?;

    Ok(web::Json(providers))
}

#[get("/providers/{provider}/navigate")]
pub async fn navigate(
    client: web::Data<ApiClient>,
    params: web::Path<ProviderParams>,
    query: web::Query<NavigateQuery>,
) -> Result<impl Responder> {
    let folder = client
        .navigate_provider(params.provider, &query.path)
        .await.map_err(failure_to_response)?;

    Ok(web::Json(folder))
}

#[get("/providers/available")]
pub async fn get_available_providers(client: web::Data<ApiClient>) -> Result<impl Responder> {
    let providers = client.get_available_providers().await.map_err(failure_to_response)?;

    Ok(web::Json(providers))
}

#[post("/providers/{provider}/auth")]
pub async fn provider_basic_auth(
    params: web::Path<ProviderParams>,
    body: web::Json<ProviderAuthModel>,
    client: web::Data<ApiClient>,
) -> Result<impl Responder> {
    client
        .authenticate_provider(params.provider, body.into_inner())
        .await.map_err(failure_to_response)?;

    Ok(HttpResponse::NoContent().finish())
}

#[get("/providers/{provider}/auth/redirect")]
pub async fn provider_token_auth(
    query: web::Query<ProviderAuthModel>,
    params: web::Path<ProviderParams>,
    client: web::Data<ApiClient>,
) -> Result<impl Responder> {
    client
        .authenticate_provider(params.provider, query.into_inner())
        .await.map_err(failure_to_response)?;

    Ok(HttpResponse::Ok().body(
        "<html><body>You can close this window now<script>window.close()</script></body></html>",
    ))
}
