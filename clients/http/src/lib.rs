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
    async fn get<T>(&self, url: &str) -> Result<T>
        where T: DeserializeOwned;

    async fn post<TReq, TRes>(&self, url: &str, req: TReq) -> Result<TRes>
        where TRes: DeserializeOwned,
              TReq: Serialize + Send + Sync;
}

#[async_trait]
impl<T> HttpClient for RusticHttpClient<T> where T: HttpClient {
    async fn get<TReq>(&self, url: &str) -> Result<TReq>
        where TReq: DeserializeOwned {
        self.client.get(url).await
    }

    async fn post<TReq, TRes>(&self, url: &str, req: TReq) -> Result<TRes>
        where TRes: DeserializeOwned,
              TReq: Serialize + Send + Sync {
        self.client.post(url, req).await
    }
}

#[async_trait]
impl<T> RusticApiClient for RusticHttpClient<T> where T: HttpClient {
    async fn search(&self, query: &str, provider: Option<&Vec<ProviderType>>) -> Result<SearchResults> {
        let url = format!("/api/search?query={}", query);
        let res = self.get(&url).await?;

        Ok(res)
    }

    async fn get_extensions(&self) -> Result<Vec<ExtensionModel>> {
        let res = self.get("/api/extensions").await?;

        Ok(res)
    }

    async fn get_providers(&self) -> Result<Vec<ProviderModel>> {
        let res = self.get("/api/providers").await?;

        Ok(res)
    }

    async fn get_available_providers(&self) -> Result<Vec<AvailableProviderModel>> {
        let res = self.get("/api/providers/available").await?;

        Ok(res)
    }
}

#[async_trait]
impl<T> LibraryApiClient for RusticHttpClient<T> where T: HttpClient {
    async fn get_albums(&self) -> Result<Vec<AlbumModel>> {
        let res = self.get("/api/library/albums").await?;

        Ok(res)
    }

    async fn get_album(&self, cursor: &str) -> Result<Option<AlbumModel>> {
        let res = self.get(&format!("/api/library/albums/{}", cursor)).await?;

        Ok(res)
    }

    async fn get_artists(&self) -> Result<Vec<ArtistModel>> {
        let res = self.get("/api/library/artists").await?;

        Ok(res)
    }

    async fn get_playlists(&self) -> Result<Vec<PlaylistModel>> {
        let res = self.get("/api/library/playlists").await?;

        Ok(res)
    }

    async fn get_playlist(&self, cursor: &str) -> Result<Option<PlaylistModel>> {
        let res = self.get(&format!("/api/library/playlists/{}", cursor)).await?;

        Ok(res)
    }

    async fn get_tracks(&self) -> Result<Vec<TrackModel>> {
        let res = self.get("/api/library/tracks").await?;

        Ok(res)
    }

    async fn get_track(&self, cursor: &str) -> Result<Option<TrackModel>> {
        let res = self.get(&format!("/api/library/tracks/{}", cursor)).await?;

        Ok(res)
    }
}

#[async_trait]
impl<T> QueueApiClient for RusticHttpClient<T> where T: HttpClient {
    async fn get_queue(&self, player_id: Option<&str>) -> Result<Vec<TrackModel>> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}", id),
            None => format!("/api/queue")
        };
        let res = self.get(&url).await?;

        Ok(res)
    }

    async fn queue_track(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}/track/{}", id, cursor),
            None => format!("/api/queue/track/{}", cursor)
        };
        // TODO: handle 404
        self.post::<(), ()>(&url, ()).await?;

        Ok(Some(()))
    }

    async fn queue_album(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}/album/{}", id, cursor),
            None => format!("/api/queue/album/{}", cursor)
        };
        // TODO: handle 404
        self.post::<(), ()>(&url, ()).await?;

        Ok(Some(()))
    }

    async fn queue_playlist(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}/playlist/{}", id, cursor),
            None => format!("/api/queue/playlist/{}", cursor)
        };
        // TODO: handle 404
        self.post::<(), ()>(&url, ()).await?;

        Ok(Some(()))
    }

    async fn clear_queue(&self, player_id: Option<&str>) -> Result<()> {
        self.post::<(), ()>("/api/queue/clear", ()).await?;

        Ok(())
    }

    async fn remove_queue_item(&self, _player_id: Option<&str>, _item: usize) -> Result<()> {
        unimplemented!()
    }
}

#[async_trait]
impl<T> PlayerApiClient for RusticHttpClient<T> where T: HttpClient {
    async fn get_players(&self) -> Result<Vec<PlayerModel>> {
        let res = self.get("/api/players").await?;

        Ok(res)
    }

    async fn get_player(&self, player_id: Option<&str>) -> Result<Option<PlayerModel>> {
        let url = match player_id {
            Some(id) => format!("/api/player/{}", id),
            None => format!("/api/player")
        };
        let res = self.get(&url).await?;

        Ok(res)
    }

    async fn player_control_next(&self, player_id: Option<&str>) -> Result<Option<()>> {
        let url = match player_id {
            Some(id) => format!("/api/player/{}/next", id),
            None => format!("/api/player/next")
        };
        self.post(&url, ()).await?;

        Ok(Some(()))
    }

    async fn player_control_prev(&self, player_id: Option<&str>) -> Result<Option<()>> {
        let url = match player_id {
            Some(id) => format!("/api/player/{}/prev", id),
            None => format!("/api/player/prev")
        };
        self.post(&url, ()).await?;

        Ok(Some(()))
    }

    async fn player_control_play(&self, player_id: Option<&str>) -> Result<()> {
        let url = match player_id {
            Some(id) => format!("/api/player/{}/play", id),
            None => format!("/api/player/play")
        };
        self.post(&url, ()).await?;

        Ok(())
    }

    async fn player_control_pause(&self, player_id: Option<&str>) -> Result<()> {
        let url = match player_id {
            Some(id) => format!("/api/player/{}/pause", id),
            None => format!("/api/player/pause")
        };
        self.post(&url, ()).await?;

        Ok(())
    }
}
