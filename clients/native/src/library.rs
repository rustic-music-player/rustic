use futures::stream::{BoxStream, StreamExt};
use itertools::Itertools;
use log::debug;

use async_trait::async_trait;
use rustic_api::client::LibraryApiClient;
use rustic_api::cursor::from_cursor;
use rustic_api::models::{
    AlbumModel, ArtistModel, PlaylistModel, ProviderTypeModel, SyncStateModel, TrackModel,
};
use rustic_core::{MultiQuery, ProviderType, QueryJoins, SingleQuery};

use crate::stream_util::from_channel;
use crate::RusticNativeClient;

#[async_trait]
impl LibraryApiClient for RusticNativeClient {
    async fn get_albums(
        &self,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<Vec<AlbumModel>, failure::Error> {
        let sw = stopwatch::Stopwatch::start_new();
        let mut query = MultiQuery::new();
        query.join_artists();
        let providers = providers
            .unwrap_or_default()
            .into_iter()
            .map(ProviderType::from)
            .collect();
        query.with_providers(providers);
        let albums = self.app.library.query_albums(query)?;
        debug!("Fetching albums took {}ms", sw.elapsed_ms());

        let albums = albums.into_iter().map(AlbumModel::from).collect();

        Ok(albums)
    }

    async fn get_album(&self, cursor: &str) -> Result<Option<AlbumModel>, failure::Error> {
        let sw = stopwatch::Stopwatch::start_new();

        let uri = from_cursor(cursor)?;
        let mut query = SingleQuery::uri(uri);
        query.join_all();
        let album = self.app.query_album(query).await?.map(AlbumModel::from);
        debug!("Fetching album took {}ms", sw.elapsed_ms());

        Ok(album)
    }

    async fn get_artists(&self) -> Result<Vec<ArtistModel>, failure::Error> {
        let sw = stopwatch::Stopwatch::start_new();
        let artists = self.app.library.query_artists(MultiQuery::new())?;
        debug!("Fetching artists took {}ms", sw.elapsed_ms());

        let artists = artists.into_iter().map(ArtistModel::from).collect();
        Ok(artists)
    }
    
    async fn get_artist(&self, cursor: &str) -> Result<Option<ArtistModel>, failure::Error> {
        let sw = stopwatch::Stopwatch::start_new();

        let uri = from_cursor(cursor)?;
        let mut query = SingleQuery::uri(uri);
        query.join_all();
        let artist = self.app.query_artist(query).await?.map(ArtistModel::from);
        debug!("Fetching artist took {}ms", sw.elapsed_ms());

        Ok(artist)
    }

    async fn get_playlists(
        &self,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<Vec<PlaylistModel>, failure::Error> {
        let sw = stopwatch::Stopwatch::start_new();
        let mut query = MultiQuery::new();
        query.join_tracks();
        let providers = providers
            .unwrap_or_default()
            .into_iter()
            .map(ProviderType::from)
            .collect();
        query.with_providers(providers);
        let playlists = self.app.library.query_playlists(query)?;
        debug!("Fetching playlists took {}ms", sw.elapsed_ms());
        let playlists = playlists
            .into_iter()
            .map(PlaylistModel::from)
            .sorted() // TODO: sorting should probably happen in library
            .collect();

        Ok(playlists)
    }

    async fn get_playlist(&self, cursor: &str) -> Result<Option<PlaylistModel>, failure::Error> {
        let sw = stopwatch::Stopwatch::start_new();

        let uri = from_cursor(cursor)?;
        let mut query = SingleQuery::uri(uri);
        query.join_all();
        let playlist = self
            .app
            .query_playlist(query)
            .await?
            .map(PlaylistModel::from);
        debug!("Fetching playlist took {}ms", sw.elapsed_ms());

        Ok(playlist)
    }

    async fn get_tracks(
        &self,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<Vec<TrackModel>, failure::Error> {
        let sw = stopwatch::Stopwatch::start_new();
        let mut query = MultiQuery::new();
        query.join_artists();
        let providers = providers
            .unwrap_or_default()
            .into_iter()
            .map(ProviderType::from)
            .collect();
        query.with_providers(providers);
        let tracks = self.app.library.query_tracks(query)?;
        debug!("Fetching tracks took {}ms", sw.elapsed_ms());
        let tracks = tracks.into_iter().map(TrackModel::from).collect();
        Ok(tracks)
    }

    async fn get_track(&self, cursor: &str) -> Result<Option<TrackModel>, failure::Error> {
        let uri = from_cursor(cursor)?;
        let query = SingleQuery::uri(uri);
        let track = self.app.query_track(query).await?;
        let track = track.map(TrackModel::from);

        Ok(track)
    }

    fn sync_state(&self) -> BoxStream<'static, SyncStateModel> {
        from_channel(self.app.sync.events.clone())
            .map(SyncStateModel::from)
            .boxed()
    }
}
