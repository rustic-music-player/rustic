use std::sync::Arc;

use failure;

use async_trait::async_trait;
use rustic_api::RusticApiClient;
use rustic_core::Rustic;
use rustic_api::models::{ExtensionModel, TrackModel, SearchResults, AvailableProviderModel, ArtistModel, PlayerModel, AlbumModel, PlaylistModel, ProviderModel};
use failure::_core::fmt::Formatter;

#[derive(Clone)]
pub struct RusticNativeClient {
    app: Arc<Rustic>
}

impl RusticNativeClient {
    pub fn new(app: Arc<Rustic>) -> RusticNativeClient {
        RusticNativeClient {
            app
        }
    }
}

#[async_trait(? Send)]
impl RusticApiClient for RusticNativeClient {
    type Error = failure::Error;

    async fn get_players(&self) -> Result<Vec<PlayerModel>, Self::Error> {
        unimplemented!()
    }

    async fn search<S: Into<String> + Send>(&self, query: S) -> Result<SearchResults, Self::Error> {
        unimplemented!()
    }

    async fn get_extensions(&self) -> Result<Vec<ExtensionModel>, Self::Error> {
        let extensions = self.app.extensions.iter().map(ExtensionModel::from).collect();

        Ok(extensions)
    }

    async fn get_albums(&self) -> Result<Vec<AlbumModel>, Self::Error> {
        unimplemented!()
    }

    async fn get_album<S: Into<String> + Send>(&self, cursor: S) -> Result<Option<AlbumModel>, Self::Error> {
        unimplemented!()
    }

    async fn get_artists(&self) -> Result<Vec<ArtistModel>, Self::Error> {
        unimplemented!()
    }

    async fn get_playlists(&self) -> Result<Vec<PlaylistModel>, Self::Error> {
        unimplemented!()
    }

    async fn get_playlist<S: Into<String> + Send>(&self, cursor: S) -> Result<Option<PlaylistModel>, Self::Error> {
        unimplemented!()
    }

    async fn get_tracks(&self) -> Result<Vec<TrackModel>, Self::Error> {
        unimplemented!()
    }

    async fn get_track<S: Into<String> + Send>(&self, cursor: S) -> Result<Option<TrackModel>, Self::Error> {
        unimplemented!()
    }

    async fn get_providers(&self) -> Result<Vec<ProviderModel>, Self::Error> {
        unimplemented!()
    }

    async fn get_available_providers(&self) -> Result<Vec<AvailableProviderModel>, Self::Error> {
        unimplemented!()
    }

    async fn get_queue(&self) -> Result<Vec<TrackModel>, Self::Error> {
        unimplemented!()
    }

    async fn queue_track<S: Into<String> + Send>(&self, cursor: S) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn queue_album<S: Into<String> + Send>(&self, cursor: S) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn queue_playlist<S: Into<String> + Send>(&self, cursor: S) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn clear_queue(&self) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

impl std::fmt::Debug for RusticNativeClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RusticNativeClient")
            .finish()
    }
}