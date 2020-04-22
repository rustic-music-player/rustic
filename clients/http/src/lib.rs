use async_trait::async_trait;
use rustic_api::models::*;

pub use rustic_api::models;
use serde::de::DeserializeOwned;

#[async_trait(?Send)]
pub trait HttpClient {
    type Error;

    async fn get<T>(&self, url: &str) -> Result<T, Self::Error>
        where T: DeserializeOwned;

    async fn get_players(&self) -> Result<Vec<PlayerModel>, Self::Error> {
        let res = self.get("/api/players").await?;

        Ok(res)
    }

    async fn search<S: Into<String> + Send>(&self, query: S) -> Result<SearchResults, Self::Error> {
        let url = format!("/api/search?query={}", query.into());
        let res = self.get(&url).await?;

        Ok(res)
    }
}