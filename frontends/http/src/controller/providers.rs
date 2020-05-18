use actix_web::{get, Responder, Result, web};
use serde::Deserialize;

use rustic_api::models::{ProviderTypeModel, ProviderAuthModel};

use crate::app::ApiClient;

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
    let providers = client.get_providers().await?;

    Ok(web::Json(providers))
}

#[get("/providers/{provider}/navigate")]
pub async fn navigate(
    client: web::Data<ApiClient>,
    params: web::Path<ProviderParams>,
    query: web::Query<NavigateQuery>,
) -> Result<impl Responder> {
    let folder = client.navigate_provider(params.provider, &query.path).await?;

    Ok(web::Json(folder))
}

#[get("/providers/available")]
pub async fn get_available_providers(client: web::Data<ApiClient>) -> Result<impl Responder> {
    let providers = client.get_available_providers().await?;

    Ok(web::Json(providers))
}

#[get("/providers/{provider}/auth/redirect")]
pub async fn provider_token_auth(
    query: web::Query<ProviderAuthModel>,
    params: web::Path<ProviderParams>,
    client: web::Data<ApiClient>,
) -> Result<impl Responder> {
    client.authenticate_provider(params.provider, query.into_inner()).await?;

    Ok(web::HttpResponse::Ok().body(
        "<html><body>You can close this window now<script>window.close()</script></body></html>",
    ))
}
