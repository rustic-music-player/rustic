use async_trait::async_trait;
use rustic_http_client::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub use rustic_http_client::RusticHttpClient;

#[derive(Debug, Clone)]
pub struct RusticNativeHttpClient {
    base_url: String,
    client: reqwest::Client
}

impl RusticNativeHttpClient {
    pub fn new<S: Into<String>>(url: S) -> RusticHttpClient<RusticNativeHttpClient> {
        let client = reqwest::Client::new();
        RusticHttpClient {
            client: RusticNativeHttpClient {
                base_url: url.into(),
                client
            }
        }
    }
}

#[async_trait]
impl HttpClient for RusticNativeHttpClient {
    async fn get<T>(&self, url: &str) -> Result<T, failure::Error>
        where T: DeserializeOwned {
        let url = format!("{}/{}", &self.base_url, url);
        let body = self.client.get(&url)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(body)
    }
    async fn post<TReq, TRes>(&self, url: &str, body: TReq) -> Result<TRes, failure::Error>
        where TRes: DeserializeOwned,
              TReq: Serialize + Send + Sync {
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
