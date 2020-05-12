use actix_web::{error, get, web, Responder};
use serde::{Deserialize};
use log::trace;

use crate::app::{ApiState, ApiClient};
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
    client: web::Data<ApiClient>,
    params: QsQuery<SearchQuery>,
) -> Result<impl Responder, error::Error> {
    trace!("search {}", &params.query);
    let result = client.search(&params.query, params.providers.as_ref()).await?;
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

#[cfg(test)]
mod test {
    use actix_web::{App, http, test};
    use actix_web::dev::*;

    use rustic_api::TestApiClient;

    use crate::test::build_app;
    use rustic_api::models::{SearchResults, ProviderType};

    #[actix_rt::test]
    async fn search_should_return_success() {
        let mut client = TestApiClient::new();
        client.expect_search()
            .called_once()
            .returning(|_| Ok(SearchResults::default()));
        let mut app = test::init_service(build_app(App::new(), client)).await;
        let req = test::TestRequest::get().uri("/search?query=test").to_request();

        let res = app.call(req).await.unwrap();
        let res = res.response();

        assert_eq!(res.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn search_should_perform_search() {
        let mut client = TestApiClient::new();
        client.expect_search()
            .called_once()
            .with((String::from("test"), None))
            .returning(move|_| Ok(SearchResults::default()));
        let mut app = test::init_service(build_app(App::new(), client)).await;
        let req = test::TestRequest::get().uri("/search?query=test").to_request();

        let res: SearchResults = test::read_response_json(&mut app, req).await;

        assert_eq!(res, SearchResults::default());
    }

    #[actix_rt::test]
    async fn search_should_perform_search_with_providers() {
        let mut client = TestApiClient::new();
        let providers = vec![ProviderType::Soundcloud, ProviderType::Spotify];
        client.expect_search()
            .called_once()
            .with((String::from("test"), Some(providers)))
            .returning(move|_| Ok(SearchResults::default()));
        let mut app = test::init_service(build_app(App::new(), client)).await;
        let req = test::TestRequest::get().uri("/search?query=test&providers[]=soundcloud&providers[]=spotify").to_request();

        let res: SearchResults = test::read_response_json(&mut app, req).await;

        assert_eq!(res, SearchResults::default());
    }
}
