use async_trait::async_trait;
use rustic_api::models::*;

pub use rustic_api::models;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[async_trait(?Send)]
pub trait HttpClient {
    type Error;

    async fn get<T>(&self, url: &str) -> Result<T, Self::Error>
        where T: DeserializeOwned;

    async fn post<TReq, TRes>(&self, url: &str, req: TReq) -> Result<TRes, Self::Error>
        where TRes: DeserializeOwned,
              TReq: Serialize;

    async fn get_players(&self) -> Result<Vec<PlayerModel>, Self::Error> {
        let res = self.get("/api/players").await?;

        Ok(res)
    }

    async fn search<S: Into<String> + Send>(&self, query: S) -> Result<SearchResults, Self::Error> {
        let url = format!("/api/search?query={}", query.into());
        let res = self.get(&url).await?;

        Ok(res)
    }

    async fn get_extensions(&self) -> Result<Vec<ExtensionModel>, Self::Error> {
        let res = self.get("/api/extensions").await?;

        Ok(res)
    }

    async fn get_albums(&self) -> Result<Vec<AlbumModel>, Self::Error> {
        let res = self.get("/api/library/albums").await?;

        Ok(res)
    }

    async fn get_album<S: Into<String> + Send>(&self, cursor: S) -> Result<Option<AlbumModel>, Self::Error> {
        let res = self.get(&format!("/api/library/albums/{}", cursor.into())).await?;

        Ok(res)
    }

    async fn get_artists(&self) -> Result<Vec<ArtistModel>, Self::Error> {
        let res = self.get("/api/library/artists").await?;

        Ok(res)
    }

    async fn get_playlists(&self) -> Result<Vec<PlaylistModel>, Self::Error> {
        let res = self.get("/api/library/playlists").await?;

        Ok(res)
    }

    async fn get_playlist<S: Into<String> + Send>(&self, cursor: S) -> Result<Option<PlaylistModel>, Self::Error> {
        let res = self.get(&format!("/api/library/playlists/{}", cursor.into())).await?;

        Ok(res)
    }

    async fn get_tracks(&self) -> Result<Vec<TrackModel>, Self::Error> {
        let res = self.get("/api/library/tracks").await?;

        Ok(res)
    }

    async fn get_track<S: Into<String> + Send>(&self, cursor: S) -> Result<Option<TrackModel>, Self::Error> {
        let res = self.get(&format!("/api/library/tracks/{}", cursor.into())).await?;

        Ok(res)
    }

    async fn get_providers(&self) -> Result<Vec<ProviderModel>, Self::Error> {
        let res = self.get("/api/providers").await?;

        Ok(res)
    }

    async fn get_available_providers(&self) -> Result<Vec<AvailableProviderModel>, Self::Error> {
        let res = self.get("/api/providers/available").await?;

        Ok(res)
    }

    async fn get_queue(&self) -> Result<Vec<TrackModel>, Self::Error> {
        let res = self.get("/api/queue").await?;

        Ok(res)
    }

    async fn queue_track<S: Into<String> + Send>(&self, cursor: S) -> Result<(), Self::Error> {
        self.post(&format!("/api/queue/track/{}", cursor.into()), ()).await?;

        Ok(())
    }

    async fn queue_album<S: Into<String> + Send>(&self, cursor: S) -> Result<(), Self::Error> {
        self.post(&format!("/api/queue/album/{}", cursor.into()), ()).await?;

        Ok(())
    }

    async fn queue_playlist<S: Into<String> + Send>(&self, cursor: S) -> Result<(), Self::Error> {
        self.post(&format!("/api/queue/playlist/{}", cursor.into()), ()).await?;

        Ok(())
    }

    async fn clear_queue(&self) -> Result<(), Self::Error> {
        self.post("/api/queue/clear", ()).await?;

        Ok(())
    }
}