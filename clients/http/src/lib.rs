use std::marker::PhantomData;

use failure::format_err;
use futures::stream::BoxStream;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::json;

use async_trait::async_trait;
use rustic_api::client::*;
use rustic_api::cursor::{Cursor, to_cursor};
pub use rustic_api::models;
use rustic_api::models::*;

#[derive(Clone)]
pub struct RusticHttpClient<T, TRes>
    where
        T: HttpClient<TRes>,
        TRes: HttpResponse
{
    pub client: T,
    pub _marker: PhantomData<TRes>,
}

impl<T, TRes> RusticHttpClient<T, TRes>
    where
        T: HttpClient<TRes>,
        TRes: HttpResponse {
    // Waiting for https://github.com/rust-lang/rfcs/pull/2632 to resolve before it can be const
    pub fn new(client: T) -> Self {
        RusticHttpClient {
            client,
            _marker: PhantomData,
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(? Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait HttpResponse: Clone + Sync + Send {
    fn no_content(self) -> Result<()>;

    async fn json<TRes>(self) -> Result<TRes>
        where
            TRes: DeserializeOwned;
}

#[cfg_attr(target_arch = "wasm32", async_trait(? Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait HttpClient<TRes>: Clone + Sync + Send where TRes: HttpResponse {
    async fn get<T>(&self, url: &str) -> Result<T>
        where
            T: DeserializeOwned;

    async fn post<TReq>(&self, url: &str, req: TReq) -> Result<TRes>
        where
            TReq: Serialize + Send + Sync;

    async fn put<TReq>(&self, url: &str, req: TReq) -> Result<TRes>
        where
            TReq: Serialize + Send + Sync;

    async fn delete(&self, url: &str) -> Result<()>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(? Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<T, TRes> HttpClient<TRes> for RusticHttpClient<T, TRes>
    where
        T: HttpClient<TRes>,
        TRes: HttpResponse
{
    async fn get<TReq>(&self, url: &str) -> Result<TReq>
        where
            TReq: DeserializeOwned,
    {
        self.client.get(url).await
    }

    async fn post<TReq>(&self, url: &str, req: TReq) -> Result<TRes>
        where
            TReq: Serialize + Send + Sync,
    {
        self.client.post(url, req).await
    }

    async fn put<TReq>(&self, url: &str, req: TReq) -> Result<TRes>
        where
            TReq: Serialize + Send + Sync,
    {
        self.client.put(url, req).await
    }

    async fn delete(&self, url: &str) -> Result<()> {
        self.client.delete(url).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchQuery<'a> {
    query: &'a str,
    providers: Option<Vec<ProviderTypeModel>>,
}

#[cfg_attr(target_arch = "wasm32", async_trait(? Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<T, TRes> RusticApiClient for RusticHttpClient<T, TRes>
    where
        T: HttpClient<TRes>,
        TRes: HttpResponse
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
        let query = serde_qs::to_string(&query)
            .map_err(|e| format_err!("Query String serialization failed: {:?}", e))?;
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

    async fn get_thumbnail(&self, _cursor: Cursor) -> Result<Option<CoverArtModel>> {
        unimplemented!()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(? Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<T, TRes> ProviderApiClient for RusticHttpClient<T, TRes>
    where
        T: HttpClient<TRes>,
        TRes: HttpResponse
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
        _provider: ProviderTypeModel,
        _path: &str,
    ) -> Result<ProviderFolderModel> {
        unimplemented!()
    }

    async fn authenticate_provider(
        &self,
        provider: ProviderTypeModel,
        auth: ProviderAuthModel,
    ) -> Result<()> {
        match auth {
            ProviderAuthModel::UserPass { username, password } => {
                self.provider_basic_auth(provider, username, password).await
            }
            ProviderAuthModel::OAuthToken { code, state, scope } => {
                self.provider_oauth_token(
                    provider,
                    ProviderAuthModel::OAuthToken { code, state, scope },
                )
                    .await
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProviderFilterQuery {
    providers: Option<Vec<ProviderTypeModel>>,
}

impl From<Option<Vec<ProviderTypeModel>>> for ProviderFilterQuery {
    fn from(providers: Option<Vec<ProviderTypeModel>>) -> Self {
        ProviderFilterQuery { providers }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(? Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<T, TRes> LibraryApiClient for RusticHttpClient<T, TRes>
    where
        T: HttpClient<TRes>,
        TRes: HttpResponse
{
    async fn get_albums(
        &self,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<Vec<AlbumModel>> {
        let query = ProviderFilterQuery::from(providers);
        let query = serde_qs::to_string(&query)
            .map_err(|e| format_err!("Query String serialization failed: {:?}", e))?;
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
        let res = self
            .get(&format!("/api/library/artists/{}", cursor))
            .await?;

        Ok(res)
    }

    async fn get_playlists(
        &self,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<Vec<PlaylistModel>> {
        let query = ProviderFilterQuery::from(providers);
        let query = serde_qs::to_string(&query)
            .map_err(|e| format_err!("Query String serialization failed: {:?}", e))?;
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
        let query = serde_qs::to_string(&query)
            .map_err(|e| format_err!("Query String serialization failed: {:?}", e))?;
        let url = format!("/api/library/tracks?{}", &query);
        let res = self.get(&url).await?;

        Ok(res)
    }

    async fn get_track(&self, cursor: &str) -> Result<Option<TrackModel>> {
        let res = self.get(&format!("/api/library/tracks/{}", cursor)).await?;

        Ok(res)
    }

    async fn add_to_library(&self, cursor: Cursor) -> Result<()> {
        let url = match cursor {
            Cursor::Track(cursor) => format!("/api/library/tracks/{}", cursor),
            Cursor::Album(cursor) => format!("/api/library/albums/{}", cursor),
            Cursor::Artist(cursor) => format!("/api/library/artists/{}", cursor),
            Cursor::Playlist(cursor) => format!("/api/library/playlists/{}", cursor),
        };

        self.post(&url, ()).await?.no_content()?;

        Ok(())
    }

    fn sync_state(&self) -> BoxStream<'static, SyncStateModel> {
        unimplemented!("requires socket api")
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(? Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<T, TRes> QueueApiClient for RusticHttpClient<T, TRes>
    where
        T: HttpClient<TRes>,
        TRes: HttpResponse
{
    async fn get_queue(&self, player_id: Option<&str>) -> Result<Vec<QueuedTrackModel>> {
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
        self.post(&url, ()).await?.no_content()?;

        Ok(Some(()))
    }

    async fn queue_album(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}/album/{}", id, cursor),
            None => format!("/api/queue/album/{}", cursor),
        };
        // TODO: handle 404
        self.post(&url, ()).await?.no_content()?;

        Ok(Some(()))
    }

    async fn queue_playlist(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}/playlist/{}", id, cursor),
            None => format!("/api/queue/playlist/{}", cursor),
        };
        // TODO: handle 404
        self.post(&url, ()).await?.no_content()?;

        Ok(Some(()))
    }

    async fn clear_queue(&self, player_id: Option<&str>) -> Result<()> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}/clear", id),
            None => "/api/queue/clear".to_string(),
        };
        self.post(&url, ()).await?.no_content()?;

        Ok(())
    }

    async fn select_queue_item(&self, player_id: Option<&str>, item: usize) -> Result<()> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}/select/{}", id, item),
            None => format!("/api/queue/select/{}", item).to_string(),
        };
        self.put(&url, ()).await?.no_content()?;

        Ok(())
    }

    async fn remove_queue_item(&self, player_id: Option<&str>, item: usize) -> Result<()> {
        let url = match player_id {
            Some(id) => format!("/api/queue/{}/{}", id, item),
            None => format!("/api/queue/{}", item).to_string(),
        };
        self.delete(&url).await?;

        Ok(())
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
        self.post(&url, ()).await?.no_content()?;

        Ok(())
    }

    fn observe_queue(&self, _player_id: Option<&str>) -> BoxStream<'static, QueueEventModel> {
        unimplemented!("requires socket api")
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(? Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<T, TRes> PlaylistApiClient for RusticHttpClient<T, TRes>
    where
        T: HttpClient<TRes>,
        TRes: HttpResponse
{
    async fn add_playlist(&self, name: &str) -> Result<PlaylistModel> {
        let res = self.post("/api/library/playlists", json!({
            "name": name
        })).await?.json().await?;

        Ok(res)
    }

    async fn remove_playlist(&self, cursor: &str) -> Result<()> {
        self.delete(&format!("/api/library/playlists/{}", cursor)).await?;

        Ok(())
    }

    async fn add_track_to_playlist(&self, cursor: &str, track: &str) -> Result<()> {
        self.put(&format!("/api/library/playlists/{}/{}", cursor, track), ()).await?.no_content()?;

        Ok(())
    }

    async fn remove_track_from_playlist(&self, cursor: &str, track: &str) -> Result<()> {
        self.delete(&format!("/api/library/playlists/{}/{}", cursor, track)).await?;

        Ok(())
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(? Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<T, TRes> PlayerApiClient for RusticHttpClient<T, TRes>
    where
        T: HttpClient<TRes>,
        TRes: HttpResponse
{
    async fn get_players(&self) -> Result<Vec<PlayerModel>> {
        let res = self.get("/api/players").await?;

        Ok(res)
    }

    async fn get_player(&self, player_id: Option<&str>) -> Result<Option<PlayerModel>> {
        let url = match player_id {
            Some(id) => format!("/api/players/{}", id),
            None => "/api/player".to_string(),
        };
        let res = self.get(&url).await?;

        Ok(res)
    }

    async fn player_control_next(&self, player_id: Option<&str>) -> Result<Option<()>> {
        let url = match player_id {
            Some(id) => format!("/api/players/{}/next", id),
            None => "/api/player/next".to_string(),
        };
        self.post(&url, ()).await?;

        Ok(Some(()))
    }

    async fn player_control_prev(&self, player_id: Option<&str>) -> Result<Option<()>> {
        let url = match player_id {
            Some(id) => format!("/api/players/{}/prev", id),
            None => "/api/player/prev".to_string(),
        };
        self.post(&url, ()).await?;

        Ok(Some(()))
    }

    async fn player_control_play(&self, player_id: Option<&str>) -> Result<()> {
        let url = match player_id {
            Some(id) => format!("/api/players/{}/play", id),
            None => "/api/player/play".to_string(),
        };
        self.post(&url, ()).await?;

        Ok(())
    }

    async fn player_control_pause(&self, player_id: Option<&str>) -> Result<()> {
        let url = match player_id {
            Some(id) => format!("/api/players/{}/pause", id),
            None => "/api/player/pause".to_string(),
        };
        self.post(&url, ()).await?;

        Ok(())
    }

    async fn player_set_volume(&self, player_id: Option<&str>, volume: f32) -> Result<()> {
        let url = match player_id {
            Some(id) => format!("/api/players/{}/volume", id),
            None => "/api/player/volume".to_string(),
        };
        self.post(&url, volume).await?;

        Ok(())
    }

    fn observe_player(&self, _player_id: Option<&str>) -> BoxStream<'static, PlayerEventModel> {
        unimplemented!("requires socket api")
    }
}

impl<T, TRes> RusticHttpClient<T, TRes>
    where
        T: HttpClient<TRes>,
        TRes: HttpResponse
{
    async fn provider_basic_auth(
        &self,
        provider: ProviderTypeModel,
        username: String,
        password: String,
    ) -> Result<()> {
        let url = format!("/api/providers/{}/auth", serde_json::to_string(&provider)?);
        let model = ProviderAuthModel::UserPass { username, password };

        self.post(&url, model).await?;

        Ok(())
    }

    async fn provider_oauth_token(
        &self,
        provider: ProviderTypeModel,
        auth: ProviderAuthModel,
    ) -> Result<()> {
        let query = serde_qs::to_string(&auth)
            .map_err(|e| format_err!("Query String serialization failed: {:?}", e))?;
        let url = format!(
            "/api/providers/{}/auth/redirect?{}",
            serde_json::to_string(&provider)?,
            query
        );

        self.get(&url).await?;

        Ok(())
    }
}
