use actix_web::{error, get, web, Responder};
use log::trace;
use serde::Deserialize;
use serde_qs::actix::QsQuery;

use rustic_api::cursor::from_cursor;
use rustic_api::models::ProviderTypeModel;

use crate::app::ApiClient;

#[derive(Deserialize)]
pub struct SearchQuery {
    query: String,
    providers: Option<Vec<ProviderTypeModel>>,
}

#[derive(Deserialize)]
pub struct OpenParams {
    url: String,
}

#[get("/search")]
pub async fn search(
    client: web::Data<ApiClient>,
    params: QsQuery<SearchQuery>,
) -> Result<impl Responder, error::Error> {
    trace!("search {}", &params.query);
    let query = params.into_inner();
    let result = client.search(&query.query, query.providers).await?;
    Ok(web::Json(result))
}

#[get("/search/aggregated")]
pub async fn search_aggregated(
    client: web::Data<ApiClient>,
    params: QsQuery<SearchQuery>,
) -> Result<impl Responder, error::Error> {
    trace!("search {}", &params.query);
    let query = params.into_inner();
    let result = client
        .aggregated_search(&query.query, query.providers)
        .await?;

    Ok(web::Json(result))
}

#[get("/library/search")]
pub async fn search_library(
    client: web::Data<ApiClient>,
    params: QsQuery<SearchQuery>,
) -> Result<impl Responder, error::Error> {
    trace!("search {}", &params.query);
    let query = params.into_inner();
    let result = client.search_library(&query.query).await?;
    Ok(web::Json(result))
}

#[get("/open/{url}")]
pub async fn open(
    client: web::Data<ApiClient>,
    params: web::Path<OpenParams>,
) -> Result<impl Responder, error::Error> {
    let url = from_cursor(&params.url)?;

    match client.open_share_url(&url).await? {
        Some(result) => Ok(web::Json(result)),
        None => Err(error::ErrorNotFound("no results")),
    }
}

#[cfg(test)]
mod test {
    use actix_web::dev::*;
    use actix_web::{http, test, App};

    use rustic_api::models::{ProviderTypeModel, SearchResults};
    use rustic_api::TestApiClient;

    use crate::test::build_app;

    #[actix_rt::test]
    async fn search_should_return_success() {
        let mut client = TestApiClient::new();
        client
            .expect_search()
            .called_once()
            .returning(|_| Ok(SearchResults::default()));
        let mut app = test::init_service(build_app(App::new(), client)).await;
        let req = test::TestRequest::get()
            .uri("/search?query=test")
            .to_request();

        let res = app.call(req).await.unwrap();
        let res = res.response();

        assert_eq!(res.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn search_should_perform_search() {
        let mut client = TestApiClient::new();
        client
            .expect_search()
            .called_once()
            .with((String::from("test"), None))
            .returning(move |_| Ok(SearchResults::default()));
        let mut app = test::init_service(build_app(App::new(), client)).await;
        let req = test::TestRequest::get()
            .uri("/search?query=test")
            .to_request();

        let res: SearchResults = test::read_response_json(&mut app, req).await;

        assert_eq!(res, SearchResults::default());
    }

    #[actix_rt::test]
    async fn search_should_perform_search_with_providers() {
        let mut client = TestApiClient::new();
        let providers = vec![ProviderTypeModel::Soundcloud, ProviderTypeModel::Spotify];
        client
            .expect_search()
            .called_once()
            .with((String::from("test"), Some(providers)))
            .returning(move |_| Ok(SearchResults::default()));
        let mut app = test::init_service(build_app(App::new(), client)).await;
        let req = test::TestRequest::get()
            .uri("/search?query=test&providers[]=soundcloud&providers[]=spotify")
            .to_request();

        let res: SearchResults = test::read_response_json(&mut app, req).await;

        assert_eq!(res, SearchResults::default());
    }
}
