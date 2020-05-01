use async_trait::async_trait;

use crate::models::*;

#[async_trait]
pub trait RusticApiClient: Sync + Send + QueueApiClient + LibraryApiClient {
    async fn get_players(&self) -> Result<Vec<PlayerModel>, failure::Error>;

    async fn search(&self, query: &str, providers: Option<&Vec<ProviderType>>) -> Result<SearchResults, failure::Error>;

    async fn get_extensions(&self) -> Result<Vec<ExtensionModel>, failure::Error>;

    async fn get_providers(&self) -> Result<Vec<ProviderModel>, failure::Error>;

    async fn get_available_providers(&self) -> Result<Vec<AvailableProviderModel>, failure::Error>;
}

#[async_trait]
pub trait LibraryApiClient: Sync + Send {
    async fn get_albums(&self) -> Result<Vec<AlbumModel>, failure::Error>;

    async fn get_album(&self, cursor: &str) -> Result<Option<AlbumModel>, failure::Error>;

    async fn get_artists(&self) -> Result<Vec<ArtistModel>, failure::Error>;

    async fn get_playlists(&self) -> Result<Vec<PlaylistModel>, failure::Error>;

    async fn get_playlist(&self, cursor: &str) -> Result<Option<PlaylistModel>, failure::Error>;

    async fn get_tracks(&self) -> Result<Vec<TrackModel>, failure::Error>;

    async fn get_track(&self, cursor: &str) -> Result<Option<TrackModel>, failure::Error>;
}

#[async_trait]
pub trait QueueApiClient: Sync + Send {
    async fn get_queue(&self, player_id: Option<&str>) -> Result<Vec<TrackModel>, failure::Error>;

    async fn queue_track(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>, failure::Error>;

    async fn queue_album(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>, failure::Error>;

    async fn queue_playlist(&self, player_id: Option<&str>, cursor: &str) -> Result<Option<()>, failure::Error>;

    async fn clear_queue(&self, player_id: Option<&str>) -> Result<(), failure::Error>;

    async fn remove_queue_item(&self, player_id: Option<&str>, item: usize) -> Result<(), failure::Error>;
}