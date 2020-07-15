use actix_web::{get, post, web, Responder, Result};
use serde::Deserialize;

use crate::app::ApiClient;

#[derive(Debug, Clone, Deserialize)]
pub struct ExtensionQuery {
    id: String
}

#[get("/extensions")]
pub async fn get_extensions(client: web::Data<ApiClient>) -> Result<impl Responder> {
    let extensions = client.get_extensions().await?;

    Ok(web::Json(extensions))
}

#[post("/extensions/{id}/enable")]
pub async fn enable_extension(client: web::Data<ApiClient>, params: web::Path<ExtensionQuery>) -> Result<impl Responder> {
    client.enable_extension(&params.id).await?;

    Ok(web::HttpResponse::NoContent())
}

#[post("/extensions/{id}/disable")]
pub async fn disable_extension(client: web::Data<ApiClient>, params: web::Path<ExtensionQuery>) -> Result<impl Responder> {
    client.disable_extension(&params.id).await?;

    Ok(web::HttpResponse::NoContent())
}

#[cfg(test)]
mod test {
    use actix_web::dev::*;
    use actix_web::{http, test, App};

    use rustic_api::models::ExtensionModel;
    use rustic_api::TestApiClient;

    use crate::test::build_app;

    #[actix_rt::test]
    async fn get_extensions_should_return_success() {
        let client = TestApiClient::new();
        let mut app = test::init_service(build_app(App::new(), client)).await;
        let req = test::TestRequest::get().uri("/extensions").to_request();

        let res = app.call(req).await.unwrap();
        let res = res.response();

        assert_eq!(res.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_extensions_should_return_extensions() {
        let extensions = vec![ExtensionModel {
            id: String::new(),
            name: String::new(),
            version: String::new(),
            enabled: true,
        }];
        let mut client = TestApiClient::new();
        client.extensions = extensions.clone();
        let mut app = test::init_service(build_app(App::new(), client)).await;
        let req = test::TestRequest::get().uri("/extensions").to_request();

        let res: Vec<ExtensionModel> = test::read_response_json(&mut app, req).await;

        assert_eq!(res, extensions);
    }
}
