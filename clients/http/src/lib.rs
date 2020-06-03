use failure::format_err;
use futures::stream::BoxStream;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use async_trait::async_trait;
use rustic_api::client::*;
use rustic_api::cursor::{to_cursor, Cursor};
pub use rustic_api::models;
use rustic_api::models::*;

#[derive(Clone)]
pub struct RusticHttpClient<T>
where
    T: HttpClient,
{
    pub client: T,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait HttpClient: Clone + Sync + Send {
    async fn get<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned;

    async fn post<TReq, TRes>(&self, url: &str, req: TReq) -> Result<TRes>
    where
        TRes: DeserializeOwned,
        TReq: Serialize + Send + Sync;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<T> HttpClient for RusticHttpClient<T>
where
    T: HttpClient,
{
    async fn get<TReq>(&self, url: &str) -> Result<TReq>
    where
        TReq: DeserializeOwned,
    {
        self.client.get(url).await
    }

    async fn post<TReq, TRes>(&self, url: &str, req: TReq) -> Result<TRes>
        where
            TRes: DeserializeOwned,
            TReq: Serialize + Send + Sync,
    {
        self.client.post(url, req).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchQuery<'a> {
    query: &'a str,
    providers: Option<Vec<ProviderTypeModel>>,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<T> RusticApiClient for RusticHttpClient<T>
    where
        T: HttpClient,
{
    async fn search(
        &self,
        query: &str,
        provider: Option<Vec<ProviderTypeModel>>,
    ) -> Result<SearchResults> {
        let query = SearchQuery {
            query,
            providers: provider,
        };
        let query = serde_qs::to_string(&query).map_err(|e| format_err!("Query String serialization failed: {:?}", e))?;
        let url = format!("/api/search?{}", query);
        let res = self.get(&url).await?;

        Ok(res)
    }

    async fn get_extensions(&self) -> Result<Vec<ExtensionModel>> {
        let res = self.get("/api/extensions").await?;

        Ok(res)
    }

    async fn open_share_url(&self, url: &str) -> Result<Option<OpenResultModel>> {
        let url = format!("/api/open/{}", to_cursor(url));
        let res = self.get(&url).await?;

        Ok(res)
    }

    async fn get_thumbnail(&self, cursor: Cursor) -> Result<Option<CoverArtModel>> {
        unimplemented!()
    }
}


#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<T> ProviderApiClient for RusticHttpClient<T>
where
    T: HttpClient,
{
    async fn get_providers(&self) -> Result<Vec<ProviderModel>> {
        let res = self.get("/api/providers").await?;

        Ok(res)
    }

    async fn get_available_providers(&self) -> Result<Vec<AvailableProviderModel>> {
        let res = self.get("/api/providers/available").await?;

        Ok(res)
    }

    async fn navigate_provider(
        &self,
        provider: ProviderTypeModel,
        path: &str,
    ) -> Result<ProviderFolderModel> {
        unimplemented!()
    }

    async fn authenticate_provider(
        &self,
        provider: ProviderTypeModel,
        auth: ProviderAuthModel,
    ) -> Result<()> {
        match auth {
            ProviderAuthModel::UserPass { username, password } => self.provider_basic_auth(provider, username, password).await,
            ProviderAuthModel::OAuthToken { code, state, scope } => self.provider_oauth_token(provider, ProviderAuthModel::OAuthToken { code, state, scope }).await,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProviderFilterQuery {
    providers: Option<Vec<ProviderTypeModel>>
}

impl From<Option<Vec<ProviderTypeModel>>> for ProviderFilterQuery {
    fn from(providers: Option<Vec<ProviderTypeModel>>) -> Self {
        ProviderFilterQuery {
            providers
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<T> LibraryApiClient for RusticHttpClient<T>
    where
        T: HttpClient,
{
    async fn get_albums(
        &self,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<Vec<AlbumModel>> {
        let query = ProviderFilterQuery::from(providers);
        let query = serde_qs::to_string(&query).map_err(|e| format_err!("Query String serialization failed: {:?}", e))?;
        let url = format!("/api/library/albums?{}", &query);
        let res = self.get(&url).await?;

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

    async fn get_artist(&self, cursor: &str) -> Result<Option<ArtistModel>> {
        let res = self.get(&format!("/api/library/artists/{}", cursor)).await?;

        Ok(res)
    }

    async fn get_playlists(
        &self,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<Vec<PlaylistModel>> {
        let query = ProviderFilterQuery::from(providers);
        let query = serde_qs::to_string(&query).map_err(|e| format_err!("Query String serialization failed: {:?}", e))?;
        let url = format!("/api/library/playlists?{}", &query);
        let res = self.get(&url).await?;

        Ok(res)
    }

    async fn get_playlist(&self, cursor: &str) -> Result<Option<PlaylistModel>> {
        let res = self
            .get(&format!("/api/library/playlists/{}", cursor))
            .await?;

        Ok(res)
    }

    async fn get_tracks(
        &self,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<Vec<TrackModel>> {
        let query = ProviderFilterQuery::from(providers);
        let query = serde_qs::to_string(&query).map_err(|e| format_err!("Query String serialization failed: {:?}", e))?;
        let url = format!("/api/library/tracks?{}", &query);
        let res = self.get(&url).await?;

        Ok(res)
    }

    async fn get_track(&self, cursor: &str) -> Result<Option<TrackModel>> {
        let res = self.get(&format!("/api/library/tracks/{}", cursor)).await?;

        Ok(res)
    }

    fn sync_state(&self) -> BoxStream<'static, SyncStateModel> {
        unimplemented!("requires socket api")
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<T> QueueApiClient for RusticHttpClient<T>
where
    T: HttpClient,
{
    async fn get_queue(&self, player_id: Option<&str>) -> Result<Vec<TrackModel>> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}", id),
            None => "/api/queue".to_string(),
        };
        let res = self.get(&url).await?;

        Ok(res)
    }

    async fn queue_track(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}/track/{}", id, cursor),
            None => format!("/api/queue/track/{}", cursor),
        };
        // TODO: handle 404
        self.post::<(), ()>(&url, ()).await?;

        Ok(Some(()))
    }

    async fn queue_album(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}/album/{}", id, cursor),
            None => format!("/api/queue/album/{}", cursor),
        };
        // TODO: handle 404
        self.post::<(), ()>(&url, ()).await?;

        Ok(Some(()))
    }

    async fn queue_playlist(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}/playlist/{}", id, cursor),
            None => format!("/api/queue/playlist/{}", cursor),
        };
        // TODO: handle 404
        self.post::<(), ()>(&url, ()).await?;

        Ok(Some(()))
    }

    async fn clear_queue(&self, player_id: Option<&str>) -> Result<()> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}/clear", id),
            None => "/api/queue/clear".to_string(),
        };
        self.post::<(), ()>(&url, ()).await?;

        Ok(())
    }

    async fn remove_queue_item(&self, _player_id: Option<&str>, _item: usize) -> Result<()> {
        unimplemented!("required delete implementation")
    }

    async fn reorder_queue_item(
        &self,
        player_id: Option<&str>,
        before: usize,
        after: usize,
    ) -> Result<()> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}/reorder/{}/{}", id, before, after),
            None => format!("/api/queue/reorder/{}/{}", before, after),
        };
        self.post::<(), ()>(&url, ()).await?;

        Ok(())
    }

    fn observe_queue(&self, _player_id: Option<&str>) -> BoxStream<'static, QueueEventModel> {
        unimplemented!("requires socket api")
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<T> PlayerApiClient for RusticHttpClient<T>
where
    T: HttpClient,
{
    async fn get_players(&self) -> Result<Vec<PlayerModel>> {
        let res = self.get("/api/players").await?;

        Ok(res)
    }

    async fn get_player(&self, player_id: Option<&str>) -> Result<Option<PlayerModel>> {
        let url = match player_id {
            Some(id) => format!("/api/player/{}", id),
            None => "/api/player".to_string(),
        };
        let res = self.get(&url).await?;

        Ok(res)
    }

    async fn player_control_next(&self, player_id: Option<&str>) -> Result<Option<()>> {
        let url = match player_id {
            Some(id) => format!("/api/player/{}/next", id),
            None => "/api/player/next".to_string(),
        };
        self.post(&url, ()).await?;

        Ok(Some(()))
    }

    async fn player_control_prev(&self, player_id: Option<&str>) -> Result<Option<()>> {
        let url = match player_id {
            Some(id) => format!("/api/player/{}/prev", id),
            None => "/api/player/prev".to_string(),
        };
        self.post(&url, ()).await?;

        Ok(Some(()))
    }

    async fn player_control_play(&self, player_id: Option<&str>) -> Result<()> {
        let url = match player_id {
            Some(id) => format!("/api/player/{}/play", id),
            None => "/api/player/play".to_string(),
        };
        self.post(&url, ()).await?;

        Ok(())
    }

    async fn player_control_pause(&self, player_id: Option<&str>) -> Result<()> {
        let url = match player_id {
            Some(id) => format!("/api/player/{}/pause", id),
            None => "/api/player/pause".to_string(),
        };
        self.post(&url, ()).await?;

        Ok(())
    }

    async fn player_set_volume(&self, player_id: Option<&str>, volume: f32) -> Result<()> {
        let url = match player_id {
            Some(id) => format!("/api/player/{}/volume", id),
            None => "/api/player/volume".to_string(),
        };
        self.post(&url, volume).await?;

        Ok(())
    }

    fn observe_player(&self, _player_id: Option<&str>) -> BoxStream<'static, PlayerEventModel> {
        unimplemented!("requires socket api")
    }
}

impl<T> RusticHttpClient<T>
    where
        T: HttpClient, {
    async fn provider_basic_auth(
        &self,
        provider: ProviderTypeModel,
        username: String,
        password: String) -> Result<()> {
        let url = format!("/api/providers/{}/auth", serde_json::to_string(&provider)?);
        let model = ProviderAuthModel::UserPass { username, password };

        self.post(&url, model).await?;

        Ok(())
    }

    async fn provider_oauth_token(
        &self,
        provider: ProviderTypeModel,
        auth: ProviderAuthModel) -> Result<()> {
        let query = serde_qs::to_string(&auth).map_err(|e| format_err!("Query String serialization failed: {:?}", e))?;
        let url = format!("/api/providers/{}/auth/redirect?{}", serde_json::to_string(&provider)?, query);

        self.get(&url).await?;

        Ok(())
    }
}
