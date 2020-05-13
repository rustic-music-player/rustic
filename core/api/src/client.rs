use async_trait::async_trait;

use crate::models::*;
use futures::stream::BoxStream;

pub type Result<T> = std::result::Result<T, failure::Error>;

#[async_trait]
pub trait RusticApiClient: Sync + Send + QueueApiClient + LibraryApiClient + PlayerApiClient {
    async fn search(&self, query: &str, providers: Option<&Vec<ProviderType>>) -> Result<SearchResults>;

    async fn get_extensions(&self) -> Result<Vec<ExtensionModel>>;

    async fn get_providers(&self) -> Result<Vec<ProviderModel>>;

    async fn get_available_providers(&self) -> Result<Vec<AvailableProviderModel>>;
}

#[async_trait]
pub trait LibraryApiClient: Sync + Send {
    async fn get_albums(&self) -> Result<Vec<AlbumModel>>;

    async fn get_album(&self, cursor: &str) -> Result<Option<AlbumModel>>;

    async fn get_artists(&self) -> Result<Vec<ArtistModel>>;

    async fn get_playlists(&self) -> Result<Vec<PlaylistModel>>;

    async fn get_playlist(&self, cursor: &str) -> Result<Option<PlaylistModel>>;

    async fn get_tracks(&self) -> Result<Vec<TrackModel>>;

    async fn get_track(&self, cursor: &str) -> Result<Option<TrackModel>>;

    fn sync_state(&self) -> BoxStream<'static, SyncStateModel>;
}

#[async_trait]
pub trait QueueApiClient: Sync + Send {
    async fn get_queue(&self, player_id: Option<&str>) -> Result<Vec<TrackModel>>;

    async fn queue_track(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>>;

    async fn queue_album(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>>;

    async fn queue_playlist(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>>;

    async fn clear_queue(&self, player_id: Option<&str>) -> Result<()>;

    async fn remove_queue_item(&self, player_id: Option<&str>, item: usize) -> Result<()>;
}

#[async_trait]
pub trait PlayerApiClient: Sync + Send {
    async fn get_players(&self) -> Result<Vec<PlayerModel>>;

    async fn get_player(&self, player_id: Option<&str>) -> Result<Option<PlayerModel>>;

    async fn player_control_next(&self, player_id: Option<&str>) -> Result<Option<()>>;

    async fn player_control_prev(&self, player_id: Option<&str>) -> Result<Option<()>>;

    async fn player_control_play(&self, player_id: Option<&str>) -> Result<()>;

    async fn player_control_pause(&self, player_id: Option<&str>) -> Result<()>;
}
