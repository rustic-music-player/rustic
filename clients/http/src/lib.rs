use async_trait::async_trait;
use rustic_api::models::*;
use rustic_api::client::*;

pub use rustic_api::models;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Clone)]
pub struct RusticHttpClient<T> where T: HttpClient {
    pub client: T
}

#[async_trait]
pub trait HttpClient: Clone + Sync + Send {
    async fn get<T>(&self, url: &str) -> Result<T, failure::Error>
        where T: DeserializeOwned;

    async fn post<TReq, TRes>(&self, url: &str, req: TReq) -> Result<TRes, failure::Error>
        where TRes: DeserializeOwned,
              TReq: Serialize + Send + Sync;
}

#[async_trait]
impl<T> HttpClient for RusticHttpClient<T> where T: HttpClient {
    async fn get<TReq>(&self, url: &str) -> Result<TReq, failure::Error>
        where TReq: DeserializeOwned {
        self.client.get(url).await
    }

    async fn post<TReq, TRes>(&self, url: &str, req: TReq) -> Result<TRes, failure::Error>
        where TRes: DeserializeOwned,
              TReq: Serialize + Send + Sync {
        self.client.post(url, req).await
    }
}

#[async_trait]
impl<T> RusticApiClient for RusticHttpClient<T> where T: HttpClient {
    async fn get_players(&self) -> Result<Vec<PlayerModel>, failure::Error> {
        let res = self.get("/api/players").await?;

        Ok(res)
    }

    async fn search(&self, query: &str, provider: Option<&Vec<ProviderType>>) -> Result<SearchResults, failure::Error> {
        let url = format!("/api/search?query={}", query);
        let res = self.get(&url).await?;

        Ok(res)
    }

    async fn get_extensions(&self) -> Result<Vec<ExtensionModel>, failure::Error> {
        let res = self.get("/api/extensions").await?;

        Ok(res)
    }

    async fn get_providers(&self) -> Result<Vec<ProviderModel>, failure::Error> {
        let res = self.get("/api/providers").await?;

        Ok(res)
    }

    async fn get_available_providers(&self) -> Result<Vec<AvailableProviderModel>, failure::Error> {
        let res = self.get("/api/providers/available").await?;

        Ok(res)
    }
}

#[async_trait]
impl<T> LibraryApiClient for RusticHttpClient<T> where T: HttpClient {
    async fn get_albums(&self) -> Result<Vec<AlbumModel>, failure::Error> {
        let res = self.get("/api/library/albums").await?;

        Ok(res)
    }

    async fn get_album(&self, cursor: &str) -> Result<Option<AlbumModel>, failure::Error> {
        let res = self.get(&format!("/api/library/albums/{}", cursor)).await?;

        Ok(res)
    }

    async fn get_artists(&self) -> Result<Vec<ArtistModel>, failure::Error> {
        let res = self.get("/api/library/artists").await?;

        Ok(res)
    }

    async fn get_playlists(&self) -> Result<Vec<PlaylistModel>, failure::Error> {
        let res = self.get("/api/library/playlists").await?;

        Ok(res)
    }

    async fn get_playlist(&self, cursor: &str) -> Result<Option<PlaylistModel>, failure::Error> {
        let res = self.get(&format!("/api/library/playlists/{}", cursor)).await?;

        Ok(res)
    }

    async fn get_tracks(&self) -> Result<Vec<TrackModel>, failure::Error> {
        let res = self.get("/api/library/tracks").await?;

        Ok(res)
    }

    async fn get_track(&self, cursor: &str) -> Result<Option<TrackModel>, failure::Error> {
        let res = self.get(&format!("/api/library/tracks/{}", cursor)).await?;

        Ok(res)
    }
}

#[async_trait]
impl<T> QueueApiClient for RusticHttpClient<T> where T: HttpClient {
    async fn get_queue(&self, player_id: Option<&str>) -> Result<Vec<TrackModel>, failure::Error> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}", id),
            None => format!("/api/queue")
        };
        let res = self.get(&url).await?;

        Ok(res)
    }

    async fn queue_track(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>, failure::Error> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}/track/{}", id, cursor),
            None => format!("/api/queue/track/{}", cursor)
        };
        // TODO: handle 404
        self.post::<(), ()>(&url, ()).await?;

        Ok(Some(()))
    }

    async fn queue_album(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>, failure::Error> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}/album/{}", id, cursor),
            None => format!("/api/queue/album/{}", cursor)
        };
        // TODO: handle 404
        self.post::<(), ()>(&url, ()).await?;

        Ok(Some(()))
    }

    async fn queue_playlist(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>, failure::Error> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}/playlist/{}", id, cursor),
            None => format!("/api/queue/playlist/{}", cursor)
        };
        // TODO: handle 404
        self.post::<(), ()>(&url, ()).await?;

        Ok(Some(()))
    }

    async fn clear_queue(&self, player_id: Option<&str>) -> Result<(), failure::Error> {
        self.post::<(), ()>("/api/queue/clear", ()).await?;

        Ok(())
    }

    async fn remove_queue_item(&self, _player_id: Option<&str>, _item: usize) -> Result<(), failure::Error> {
        unimplemented!()
    }
}