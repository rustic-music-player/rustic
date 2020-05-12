use async_trait::async_trait;
use crate::client::*;
use crate::models::*;
use std::collections::HashMap;
use simulacrum::Expectations;
use simulacrum::create_expect_method;

pub struct TestApiClient {
    pub extensions: Vec<ExtensionModel>,
    pub players: HashMap<String, PlayerModel>,
    e: Expectations
}

impl TestApiClient {
    pub fn new() -> TestApiClient {
        TestApiClient {
            extensions: Vec::default(),
            players: HashMap::default(),
            e: Expectations::new()
        }
    }

    create_expect_method! {
        expect_search("search") (String, Option<Vec<ProviderType>>) => Result<SearchResults>
    }
}

#[async_trait]
impl RusticApiClient for TestApiClient {
    async fn search(&self, query: &str, providers: Option<&Vec<ProviderType>>) -> Result<SearchResults> {
        self.e.was_called_returning("search", (query.to_owned(), providers.map(|providers| providers.clone())))
    }

    async fn get_extensions(&self) -> Result<Vec<ExtensionModel>> {
        Ok(self.extensions.clone())
    }

    async fn get_providers(&self) -> Result<Vec<ProviderModel>> {
        unimplemented!()
    }

    async fn get_available_providers(&self) -> Result<Vec<AvailableProviderModel>> {
        unimplemented!()
    }
}

#[async_trait]
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
        self.e.was_called::<_, ()>("player_control_play", player_id.map(|id| id.to_owned()));
        Ok(())
    }

    async fn player_control_pause(&self, player_id: Option<&str>) -> Result<()> {
        unimplemented!()
    }
}

#[async_trait]
impl QueueApiClient for TestApiClient {
    async fn get_queue(&self, player_id: Option<&str>) -> Result<Vec<TrackModel>> {
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

    async fn remove_queue_item(&self, player_id: Option<&str>, item: usize) -> Result<()> {
        unimplemented!()
    }
}

#[async_trait]
impl LibraryApiClient for TestApiClient {
    async fn get_albums(&self) -> Result<Vec<AlbumModel>> {
        unimplemented!()
    }

    async fn get_album(&self, cursor: &str) -> Result<Option<AlbumModel>> {
        unimplemented!()
    }

    async fn get_artists(&self) -> Result<Vec<ArtistModel>> {
        unimplemented!()
    }

    async fn get_playlists(&self) -> Result<Vec<PlaylistModel>> {
        unimplemented!()
    }

    async fn get_playlist(&self, cursor: &str) -> Result<Option<PlaylistModel>> {
        unimplemented!()
    }

    async fn get_tracks(&self) -> Result<Vec<TrackModel>> {
        unimplemented!()
    }

    async fn get_track(&self, cursor: &str) -> Result<Option<TrackModel>> {
        unimplemented!()
    }
}