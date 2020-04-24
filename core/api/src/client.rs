use async_trait::async_trait;

use crate::models::*;

#[async_trait(?Send)]
pub trait RusticApiClient: Clone {
    type Error;

    async fn get_players(&self) -> Result<Vec<PlayerModel>, Self::Error>;

    async fn search<S: Into<String> + Send>(&self, query: S) -> Result<SearchResults, Self::Error>;

    async fn get_extensions(&self) -> Result<Vec<ExtensionModel>, Self::Error>;

    async fn get_albums(&self) -> Result<Vec<AlbumModel>, Self::Error>;

    async fn get_album<S: Into<String> + Send>(&self, cursor: S) -> Result<Option<AlbumModel>, Self::Error>;

    async fn get_artists(&self) -> Result<Vec<ArtistModel>, Self::Error>;

    async fn get_playlists(&self) -> Result<Vec<PlaylistModel>, Self::Error>;

    async fn get_playlist<S: Into<String> + Send>(&self, cursor: S) -> Result<Option<PlaylistModel>, Self::Error>;

    async fn get_tracks(&self) -> Result<Vec<TrackModel>, Self::Error>;

    async fn get_track<S: Into<String> + Send>(&self, cursor: S) -> Result<Option<TrackModel>, Self::Error>;

    async fn get_providers(&self) -> Result<Vec<ProviderModel>, Self::Error>;

    async fn get_available_providers(&self) -> Result<Vec<AvailableProviderModel>, Self::Error>;

    async fn get_queue(&self) -> Result<Vec<TrackModel>, Self::Error>;

    async fn queue_track<S: Into<String> + Send>(&self, cursor: S) -> Result<(), Self::Error>;

    async fn queue_album<S: Into<String> + Send>(&self, cursor: S) -> Result<(), Self::Error>;

    async fn queue_playlist<S: Into<String> + Send>(&self, cursor: S) -> Result<(), Self::Error>;

    async fn clear_queue(&self) -> Result<(), Self::Error>;
}
