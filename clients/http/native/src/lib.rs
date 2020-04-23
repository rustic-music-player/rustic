use async_trait::async_trait;
use rustic_http_client::HttpClient;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct RusticHttpClient {
    base_url: String,
    client: reqwest::Client
}

impl RusticHttpClient {
    pub fn new<S: Into<String>>(url: S) -> RusticHttpClient {
        let client = reqwest::Client::new();
        RusticHttpClient {
            base_url: url.into(),
            client
        }
    }
}

#[async_trait(?Send)]
impl HttpClient for RusticHttpClient {
    type Error = reqwest::Error;

    async fn get<T>(&self, url: &str) -> Result<T, Self::Error>
        where T: DeserializeOwned {
        let url = format!("{}/{}", &self.base_url, url);
        let body = self.client.get(&url)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(body)
    }
    async fn post<TReq, TRes>(&self, url: &str, body: TReq) -> Result<TRes, Self::Error>
        where TRes: DeserializeOwned,
              TReq: Serialize {
        let url = format!("{}/{}", &self.base_url, url);
        let body = self.client.post(&url)
            .json(&body)
            .send()
            .await?
            .json::<TRes>()
            .await?;

        Ok(body)
    }
}