use std::sync::Arc;

use log::debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;

use async_trait::async_trait;
pub use rustic_http_client::RusticHttpClient;
use rustic_http_client::*;

#[derive(Debug, Clone)]
pub struct RusticNativeHttpClient {
    base_url: String,
    client: reqwest::Client,
}

impl RusticNativeHttpClient {
    pub fn new<S: Into<String>>(
        url: S,
    ) -> RusticHttpClient<RusticNativeHttpClient, ReqwestResponse> {
        let client = reqwest::Client::new();
        RusticHttpClient::new(RusticNativeHttpClient {
            base_url: url.into(),
            client,
        })
    }
}

#[derive(Clone)]
pub struct ReqwestResponse(Arc<reqwest::Response>);

#[async_trait]
impl HttpResponse for ReqwestResponse {
    fn no_content(self) -> Result<(), failure::Error> {
        Ok(())
    }

    async fn json<TRes>(self) -> Result<TRes, failure::Error>
    where
        TRes: DeserializeOwned,
    {
        let res =
            Arc::try_unwrap(self.0).unwrap_or_else(|_| unreachable!("This shouldn't be possible"));
        let body = res.json::<TRes>().await?;

        Ok(body)
    }
}

impl From<reqwest::Response> for ReqwestResponse {
    fn from(res: reqwest::Response) -> Self {
        ReqwestResponse(Arc::new(res))
    }
}

#[async_trait]
impl HttpClient<ReqwestResponse> for RusticNativeHttpClient {
    async fn get<T>(&self, api_url: &str) -> Result<T, failure::Error>
    where
        T: DeserializeOwned,
    {
        let mut url = Url::parse(&self.base_url)?;
        url.set_path(api_url);
        debug!("GET {}", url);
        let body = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await?;

        Ok(body)
    }

    async fn post<TReq>(&self, api_url: &str, body: TReq) -> Result<ReqwestResponse, failure::Error>
    where
        TReq: Serialize + Send + Sync,
    {
        let mut url = Url::parse(&self.base_url)?;
        url.set_path(api_url);
        debug!("POST {}", url);
        let res = self
            .client
            .post(url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.into())
    }

    async fn put<TReq>(&self, api_url: &str, body: TReq) -> Result<ReqwestResponse, failure::Error>
    where
        TReq: Serialize + Send + Sync,
    {
        let mut url = Url::parse(&self.base_url)?;
        url.set_path(api_url);
        debug!("PUT {}", url);
        let res = self
            .client
            .put(url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.into())
    }

    async fn delete(&self, api_url: &str) -> Result<(), failure::Error> {
        let mut url = Url::parse(&self.base_url)?;
        url.set_path(api_url);
        debug!("DELETE {}", url);
        self.client.delete(url).send().await?.error_for_status()?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use mockito::mock;
    use serde::{Deserialize, Serialize};

    use rustic_http_client::HttpClient;

    use crate::RusticNativeHttpClient;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    struct TestResponse {
        pub cursor: String,
    }

    fn create_json_mock<T>(
        method: &str,
        url: &str,
        response: &T,
    ) -> Result<mockito::Mock, failure::Error>
    where
        T: Serialize,
    {
        let m = mock(method, url)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(response)?)
            .create();

        Ok(m)
    }

    #[tokio::test]
    async fn get_should_do_a_get_request_to_the_api_root() -> Result<(), failure::Error> {
        let expected = TestResponse {
            cursor: String::from("abc"),
        };
        let _m = create_json_mock("GET", "/", &expected)?;
        let client = RusticNativeHttpClient::new(mockito::server_url());

        let res = client.get::<TestResponse>("/").await?;

        assert_eq!(res, expected);
        Ok(())
    }

    #[tokio::test]
    async fn get_should_do_a_get_request_1() -> Result<(), failure::Error> {
        let expected = TestResponse {
            cursor: String::from("abc"),
        };
        let _m = create_json_mock("GET", "/api/test", &expected)?;
        let client = RusticNativeHttpClient::new(mockito::server_url());

        let res = client.get::<TestResponse>("/api/test").await?;

        assert_eq!(res, expected);
        Ok(())
    }
}
