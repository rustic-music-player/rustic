use std::collections::HashMap;

use futures::stream::BoxStream;
use simulacrum::create_expect_method;
use simulacrum::Expectations;

use async_trait::async_trait;

use crate::client::*;
use crate::cursor::Cursor;
use crate::models::*;

pub struct TestApiClient {
    pub extensions: Vec<ExtensionModel>,
    pub players: HashMap<String, PlayerModel>,
    e: Expectations,
}

impl TestApiClient {
    pub fn new() -> TestApiClient {
        TestApiClient {
            extensions: Vec::default(),
            players: HashMap::default(),
            e: Expectations::new(),
        }
    }

    create_expect_method! {
        expect_search("search") (String, Option<Vec<ProviderTypeModel>>) => Result<SearchResults>
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl RusticApiClient for TestApiClient {
    async fn search(
        &self,
        query: &str,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<SearchResults> {
        self.e.was_called_returning(
            "search",
            (
                query.to_owned(),
                providers,
            ),
        )
    }

    async fn get_extensions(&self) -> Result<Vec<ExtensionModel>> {
        Ok(self.extensions.clone())
    }

    async fn open_share_url(&self, url: &str) -> Result<Option<OpenResultModel>> {
        unimplemented!()
    }

    async fn get_thumbnail(&self, cursor: Cursor) -> Result<Option<CoverArtModel>> {
        unimplemented!()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl PlayerApiClient for TestApiClient {
    async fn get_players(&self) -> Result<Vec<PlayerModel>> {
        unimplemented!()
    }

    async fn get_player(&self, player_id: Option<&str>) -> Result<Option<PlayerModel>> {
        unimplemented!()
    }

    async fn player_control_next(&self, player_id: Option<&str>) -> Result<Option<()>> {
        unimplemented!()
    }

    async fn player_control_prev(&self, player_id: Option<&str>) -> Result<Option<()>> {
        unimplemented!()
    }

    async fn player_control_play(&self, player_id: Option<&str>) -> Result<()> {
        self.e
            .was_called::<_, ()>("player_control_play", player_id.map(|id| id.to_owned()));
        Ok(())
    }

    async fn player_control_pause(&self, player_id: Option<&str>) -> Result<()> {
        unimplemented!()
    }

    async fn player_set_volume(&self, player_id: Option<&str>, volume: f32) -> Result<()> {
        unimplemented!()
    }

    fn observe_player(&self, player_id: Option<&str>) -> BoxStream<'static, PlayerEventModel> {
        unimplemented!()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl QueueApiClient for TestApiClient {
    async fn get_queue(&self, player_id: Option<&str>) -> Result<Vec<QueuedTrackModel>> {
        unimplemented!()
    }

    async fn queue_track(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>> {
        unimplemented!()
    }

    async fn queue_album(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>> {
        unimplemented!()
    }

    async fn queue_playlist(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>> {
        unimplemented!()
    }

    async fn clear_queue(&self, player_id: Option<&str>) -> Result<()> {
        unimplemented!()
    }

    async fn select_queue_item(&self, player_id: Option<&str>, item: usize) -> Result<()> {
        unimplemented!()
    }

    async fn remove_queue_item(&self, player_id: Option<&str>, item: usize) -> Result<()> {
        unimplemented!()
    }

    async fn reorder_queue_item(
        &self,
        player_id: Option<&str>,
        before: usize,
        after: usize,
    ) -> Result<()> {
        unimplemented!()
    }

    fn observe_queue(&self, player_id: Option<&str>) -> BoxStream<'static, QueueEventModel> {
        unimplemented!()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl LibraryApiClient for TestApiClient {
    async fn get_albums(
        &self,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<Vec<AlbumModel>> {
        unimplemented!()
    }

    async fn get_album(&self, cursor: &str) -> Result<Option<AlbumModel>> {
        unimplemented!()
    }

    async fn get_artists(&self) -> Result<Vec<ArtistModel>> {
        unimplemented!()
    }

    async fn get_artist(&self, cursor: &str) -> Result<Option<ArtistModel>> {
        unimplemented!()
    }

    async fn get_playlists(
        &self,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<Vec<PlaylistModel>> {
        unimplemented!()
    }

    async fn get_playlist(&self, cursor: &str) -> Result<Option<PlaylistModel>> {
        unimplemented!()
    }

    async fn get_tracks(
        &self,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<Vec<TrackModel>> {
        unimplemented!()
    }

    async fn get_track(&self, cursor: &str) -> Result<Option<TrackModel>> {
        unimplemented!()
    }

    async fn add_to_library(&self, _cursor: Cursor) -> Result<()> {
        unimplemented!()
    }

    fn sync_state(&self) -> BoxStream<'static, SyncStateModel> {
        unimplemented!()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ProviderApiClient for TestApiClient {
    async fn get_providers(&self) -> Result<Vec<ProviderModel>> {
        unimplemented!()
    }

    async fn get_available_providers(&self) -> Result<Vec<AvailableProviderModel>> {
        unimplemented!()
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
        unimplemented!()
    }
}
