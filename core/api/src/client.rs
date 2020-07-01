use async_trait::async_trait;
use futures::stream::BoxStream;

use rustic_reflect_macros::reflect_trait;

use crate::cursor::Cursor;
use crate::models::*;

pub type Result<T> = std::result::Result<T, failure::Error>;

#[reflect_trait]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait RusticApiClient:
    Sync
    + Send
    + QueueApiClient
    + LibraryApiClient
    + PlayerApiClient
    + ProviderApiClient
    + PlaylistApiClient
{
    async fn search(
        &self,
        query: &str,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<SearchResults>;

    async fn aggregated_search(
        &self,
        query: &str,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<AggregatedSearchResults>;

    async fn get_extensions(&self) -> Result<Vec<ExtensionModel>>;

    async fn open_share_url(&self, url: &str) -> Result<Option<OpenResultModel>>;

    async fn get_thumbnail(&self, cursor: Cursor) -> Result<Option<CoverArtModel>>;
}

#[reflect_trait]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ProviderApiClient: Sync + Send {
    async fn get_providers(&self) -> Result<Vec<ProviderModel>>;

    async fn get_available_providers(&self) -> Result<Vec<AvailableProviderModel>>;

    async fn navigate_provider(
        &self,
        provider: ProviderTypeModel,
        path: &str,
    ) -> Result<ProviderFolderModel>;

    async fn authenticate_provider(
        &self,
        provider: ProviderTypeModel,
        auth: ProviderAuthModel,
    ) -> Result<()>;
}

#[reflect_trait]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait LibraryApiClient: Sync + Send {
    async fn get_albums(
        &self,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<Vec<AlbumModel>>;

    async fn get_album(&self, cursor: &str) -> Result<Option<AggregatedAlbum>>;

    async fn get_artists(&self) -> Result<Vec<ArtistModel>>;

    async fn get_artist(&self, cursor: &str) -> Result<Option<AggregatedArtist>>;

    async fn get_playlists(
        &self,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<Vec<PlaylistModel>>;

    async fn get_playlist(&self, cursor: &str) -> Result<Option<PlaylistModel>>;

    async fn get_tracks(
        &self,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<Vec<TrackModel>>;

    async fn get_track(&self, cursor: &str) -> Result<Option<AggregatedTrack>>;

    async fn add_to_library(&self, cursor: Cursor) -> Result<()>;

    async fn remove_from_library(&self, cursor: Cursor) -> Result<()>;

    fn sync_state(&self) -> BoxStream<'static, SyncStateModel>;
}

#[reflect_trait]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait QueueApiClient: Sync + Send {
    async fn get_queue(&self, player_id: Option<&str>) -> Result<Vec<QueuedTrackModel>>;

    async fn queue_track(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>>;

    async fn queue_album(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>>;

    async fn queue_playlist(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>>;

    async fn clear_queue(&self, player_id: Option<&str>) -> Result<()>;

    async fn select_queue_item(&self, player_id: Option<&str>, item: usize) -> Result<()>;

    async fn remove_queue_item(&self, player_id: Option<&str>, item: usize) -> Result<()>;

    async fn reorder_queue_item(
        &self,
        player_id: Option<&str>,
        before: usize,
        after: usize,
    ) -> Result<()>;

    fn observe_queue(&self, player_id: Option<&str>) -> BoxStream<'static, QueueEventModel>;
}

#[reflect_trait]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait PlaylistApiClient: Sync + Send {
    async fn add_playlist(&self, name: &str) -> Result<PlaylistModel>;

    async fn remove_playlist(&self, cursor: &str) -> Result<()>;

    async fn add_track_to_playlist(&self, cursor: &str, track: &str) -> Result<()>;

    async fn remove_track_from_playlist(&self, cursor: &str, track: &str) -> Result<()>;
}

#[reflect_trait]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait PlayerApiClient: Sync + Send {
    async fn get_players(&self) -> Result<Vec<PlayerModel>>;

    async fn get_player(&self, player_id: Option<&str>) -> Result<Option<PlayerModel>>;

    async fn player_control_next(&self, player_id: Option<&str>) -> Result<Option<()>>;

    async fn player_control_prev(&self, player_id: Option<&str>) -> Result<Option<()>>;

    async fn player_control_play(&self, player_id: Option<&str>) -> Result<()>;

    async fn player_control_pause(&self, player_id: Option<&str>) -> Result<()>;

    async fn player_set_volume(&self, player_id: Option<&str>, volume: f32) -> Result<()>;

    async fn player_set_repeat(
        &self,
        player_id: Option<&str>,
        repeat: RepeatModeModel,
    ) -> Result<()>;

    fn observe_player(&self, player_id: Option<&str>) -> BoxStream<'static, PlayerEventModel>;
}
