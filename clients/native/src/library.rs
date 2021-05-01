use std::convert::TryInto;

use async_trait::async_trait;
use futures::future;
use futures::stream::{BoxStream, StreamExt};
use itertools::Itertools;
use log::debug;

use rustic_api::client::{LibraryApiClient, Result};
use rustic_api::cursor::{from_cursor, Cursor};
use rustic_api::models::*;
use rustic_core::provider::InternalUri;
use rustic_core::{MultiQuery, ProviderType, QueryJoins, SingleQuery};
use rustic_extension_api::ExtensionApi;

use crate::RusticNativeClient;

#[async_trait]
impl LibraryApiClient for RusticNativeClient {
    async fn get_albums(
        &self,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<Vec<AlbumModel>> {
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

        let albums = future::try_join_all(
            albums
                .into_iter()
                .map(|album| self.extensions.resolve_album(album)),
        )
        .await?;

        let albums = albums.into_iter().map(AlbumModel::from).collect();

        Ok(albums)
    }

    async fn get_album(&self, cursor: &str) -> Result<Option<AggregatedAlbum>> {
        let sw = stopwatch::Stopwatch::start_new();

        let cursors = RusticNativeClient::get_cursors(cursor);

        let mut albums = Vec::new();
        for cursor in cursors {
            let uri = from_cursor(cursor)?;
            let mut query: SingleQuery = uri.into();
            query.join_all();
            if let Some(album) = self.query_album(query).await? {
                let album = self.extensions.resolve_album(album).await?;
                albums.push(album.into());
            }
        }
        let album: Option<AlbumCollection> = Aggregate::aggregate_single(albums);
        debug!("Fetching album took {}ms", sw.elapsed_ms());

        Ok(album.map(AggregatedAlbum::from))
    }

    async fn get_artists(&self) -> Result<Vec<ArtistModel>> {
        let sw = stopwatch::Stopwatch::start_new();
        let artists = self.app.library.query_artists(MultiQuery::new())?;
        debug!("Fetching artists took {}ms", sw.elapsed_ms());

        let artists = future::try_join_all(
            artists
                .into_iter()
                .map(|artist| self.extensions.resolve_artist(artist)),
        )
        .await?;

        let artists = artists.into_iter().map(ArtistModel::from).collect();
        Ok(artists)
    }

    async fn get_artist(&self, cursor: &str) -> Result<Option<AggregatedArtist>> {
        let sw = stopwatch::Stopwatch::start_new();

        let cursors = RusticNativeClient::get_cursors(cursor);

        let mut artists = Vec::new();
        for cursor in cursors {
            let uri = from_cursor(cursor)?;
            let mut query: SingleQuery = uri.into();
            query.join_all();
            if let Some(artist) = self.query_artist(query).await? {
                let artist = self.extensions.resolve_artist(artist).await?;
                artists.push(artist.into());
            }
        }
        let artist: Option<ArtistCollection> = Aggregate::aggregate_single(artists);
        debug!("Fetching artist took {}ms", sw.elapsed_ms());

        Ok(artist.map(AggregatedArtist::from))
    }

    async fn get_playlists(
        &self,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<Vec<PlaylistModel>> {
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
        let playlists = future::try_join_all(
            playlists
                .into_iter()
                .map(|playlist| self.extensions.resolve_playlist(playlist)),
        )
        .await?;
        let playlists = playlists
            .into_iter()
            .map(PlaylistModel::from)
            .sorted() // TODO: sorting should probably happen in library
            .collect();

        Ok(playlists)
    }

    async fn get_playlist(&self, cursor: &str) -> Result<Option<PlaylistModel>> {
        let sw = stopwatch::Stopwatch::start_new();

        let uri = from_cursor(cursor)?;
        let mut query: SingleQuery = uri.into();
        query.join_all();
        let playlist = self.query_playlist(query).await?;
        let playlist =  if let Some(playlist) = playlist {
            let playlist = self.extensions.resolve_playlist(playlist).await?;
            Some(PlaylistModel::from(playlist))
        }else { None };
        debug!("Fetching playlist took {}ms", sw.elapsed_ms());

        Ok(playlist)
    }

    async fn get_tracks(
        &self,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<Vec<TrackModel>> {
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
        let tracks = future::try_join_all(
            tracks
                .into_iter()
                .map(|track| self.extensions.resolve_track(track)),
        )
        .await?;
        let tracks = tracks.into_iter().map(TrackModel::from).collect();
        Ok(tracks)
    }

    async fn get_track(&self, cursor: &str) -> Result<Option<AggregatedTrack>> {
        let sw = stopwatch::Stopwatch::start_new();

        let cursors = RusticNativeClient::get_cursors(cursor);

        let mut tracks = Vec::new();
        for cursor in cursors {
            let uri = from_cursor(cursor)?;
            if let Some(track) = self.query_track(uri.into()).await? {
                let track = self.extensions.resolve_track(track).await?;
                tracks.push(track.into());
            }
        }
        let track: Option<TrackCollection> = Aggregate::aggregate_single(tracks);
        debug!("Fetching track took {}ms", sw.elapsed_ms());

        Ok(track.map(AggregatedTrack::from))
    }

    async fn add_to_library(&self, cursor: Cursor) -> Result<()> {
        match cursor.try_into()? {
            InternalUri::Track(uri) => self.add_track_to_library(uri.into()).await?,
            InternalUri::Album(uri) => self.add_album_to_library(uri.into()).await?,
            InternalUri::Artist(uri) => self.add_artist_to_library(uri.into()).await?,
            InternalUri::Playlist(uri) => self.add_playlist_to_library(uri.into()).await?,
        }
        self.app.library.flush()?;
        Ok(())
    }

    async fn remove_from_library(&self, cursor: Cursor) -> Result<()> {
        match cursor.try_into()? {
            InternalUri::Track(uri) => self.remove_track_from_library(uri.into()).await?,
            InternalUri::Album(uri) => self.remove_album_from_library(uri.into()).await?,
            InternalUri::Artist(uri) => self.remove_artist_from_library(uri.into()).await?,
            InternalUri::Playlist(uri) => self.remove_playlist_from_library(uri.into()).await?,
        }
        self.app.library.flush()?;
        Ok(())
    }

    async fn search_library(&self, query: &str) -> Result<SearchResults> {
        let results = self.app.library.search(query.into())?;

        let tracks = future::try_join_all(
            results
                .tracks
                .into_iter()
                .map(|track| self.extensions.resolve_track(track)),
        )
        .await?;
        let albums = future::try_join_all(
            results
                .albums
                .into_iter()
                .map(|album| self.extensions.resolve_album(album)),
        )
        .await?;
        let artists = future::try_join_all(
            results
                .artists
                .into_iter()
                .map(|artist| self.extensions.resolve_artist(artist)),
        )
        .await?;
        let playlists = future::try_join_all(
            results
                .playlists
                .into_iter()
                .map(|playlist| self.extensions.resolve_playlist(playlist)),
        )
        .await?;

        Ok(SearchResults {
            tracks: tracks.into_iter().map(TrackModel::from).collect(),
            albums: albums.into_iter().map(AlbumModel::from).collect(),
            artists: artists.into_iter().map(ArtistModel::from).collect(),
            playlists: playlists.into_iter().map(PlaylistModel::from).collect(),
        })
    }

    fn sync_state(&self) -> BoxStream<'static, SyncStateModel> {
        self.app
            .sync
            .events
            .stream()
            .map(SyncStateModel::from)
            .boxed()
    }

    fn observe_library(&self) -> BoxStream<'static, LibraryEventModel> {
        self.app
            .library
            .observe()
            .map(LibraryEventModel::from)
            .boxed()
    }
}

impl RusticNativeClient {
    async fn add_track_to_library(&self, query: SingleQuery) -> Result<()> {
        let track = self.app.query_track(query).await?;
        if let Some(mut track) = track {
            self.app.library.add_track(&mut track)?;
        }
        Ok(())
    }

    async fn add_album_to_library(&self, query: SingleQuery) -> Result<()> {
        let album = self.app.query_album(query).await?;
        if let Some(mut album) = album {
            self.app.library.add_album(&mut album)?;
            if let Some(artist) = album.artist {
                if let Some(mut artist) = self.app.query_artist(artist.uri.into()).await? {
                    self.app.library.sync_artist(&mut artist)?;
                }
            }
        }
        Ok(())
    }

    async fn add_artist_to_library(&self, query: SingleQuery) -> Result<()> {
        let artist = self.app.query_artist(query).await?;
        if let Some(mut artist) = artist {
            self.app.library.add_artist(&mut artist)?;
        }
        Ok(())
    }

    async fn add_playlist_to_library(&self, query: SingleQuery) -> Result<()> {
        let playlist = self.app.query_playlist(query).await?;
        if let Some(mut playlist) = playlist {
            self.app.library.add_playlist(&mut playlist)?;
        }
        Ok(())
    }

    async fn remove_track_from_library(&self, query: SingleQuery) -> Result<()> {
        let track = self.app.query_track(query).await?;
        if let Some(track) = track {
            self.app.library.remove_track(&track)?;
        }
        Ok(())
    }

    async fn remove_album_from_library(&self, query: SingleQuery) -> Result<()> {
        let album = self.app.query_album(query).await?;
        if let Some(album) = album {
            self.app.library.remove_album(&album)?;
        }
        Ok(())
    }

    async fn remove_artist_from_library(&self, query: SingleQuery) -> Result<()> {
        let artist = self.app.query_artist(query).await?;
        if let Some(artist) = artist {
            self.app.library.remove_artist(&artist)?;
        }
        Ok(())
    }

    async fn remove_playlist_from_library(&self, query: SingleQuery) -> Result<()> {
        let playlist = self.app.query_playlist(query).await?;
        if let Some(playlist) = playlist {
            self.app.library.remove_playlist(&playlist)?;
        }
        Ok(())
    }

    fn get_cursors(cursor: &str) -> Vec<&str> {
        if cursor.starts_with("a:") {
            cursor.split(':').skip(1).collect()
        } else {
            vec![cursor]
        }
    }
}
