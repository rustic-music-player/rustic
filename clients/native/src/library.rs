use async_trait::async_trait;
use log::debug;

use rustic_api::client::LibraryApiClient;
use rustic_api::models::{TrackModel, PlaylistModel, ArtistModel, AlbumModel};
use crate::RusticNativeClient;
use rustic_core::{MultiQuery, QueryJoins, SingleQuery};
use rustic_api::cursor::from_cursor;
use itertools::Itertools;

#[async_trait]
impl LibraryApiClient for RusticNativeClient {
    async fn get_albums(&self) -> Result<Vec<AlbumModel>, failure::Error> {
        let sw = stopwatch::Stopwatch::start_new();
        let mut query = MultiQuery::new();
        query.join_artists();
        let albums = self.app.library.query_albums(query)?;
        debug!("Fetching albums took {}ms", sw.elapsed_ms());

        let albums = albums
            .into_iter()
            .map(AlbumModel::from)
            .collect();

        Ok(albums)
    }

    async fn get_album(&self, cursor: &str) -> Result<Option<AlbumModel>, failure::Error> {
        let sw = stopwatch::Stopwatch::start_new();

        let uri = from_cursor(cursor)?;
        let mut query = SingleQuery::uri(uri);
        query.join_all();
        let album = self.app
            .query_album(query)?
            .map(AlbumModel::from);
        debug!("Fetching album took {}ms", sw.elapsed_ms());

        Ok(album)
    }

    async fn get_artists(&self) -> Result<Vec<ArtistModel>, failure::Error> {
        let sw = stopwatch::Stopwatch::start_new();
        let artists = self.app.library.query_artists(MultiQuery::new())?;
        debug!("Fetching artists took {}ms", sw.elapsed_ms());

        let artists = artists
            .into_iter()
            .map(ArtistModel::from)
            .collect();
        Ok(artists)
    }

    async fn get_playlists(&self) -> Result<Vec<PlaylistModel>, failure::Error> {
        let sw = stopwatch::Stopwatch::start_new();
        let mut query = MultiQuery::new();
        query.join_tracks();
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
        let playlist = self.app
            .query_playlist(query)?
            .map(PlaylistModel::from);
        debug!("Fetching playlist took {}ms", sw.elapsed_ms());

        Ok(playlist)
    }

    async fn get_tracks(&self) -> Result<Vec<TrackModel>, failure::Error> {
        let sw = stopwatch::Stopwatch::start_new();
        let mut query = MultiQuery::new();
        query.join_artists();
        let tracks = self.app.library.query_tracks(query)?;
        debug!("Fetching tracks took {}ms", sw.elapsed_ms());
        let tracks = tracks
            .into_iter()
            .map(TrackModel::from)
            .collect();
        Ok(tracks)
    }

    async fn get_track(&self, cursor: &str) -> Result<Option<TrackModel>, failure::Error> {
        let uri = from_cursor(cursor)?;
        let query = SingleQuery::uri(uri);
        let track = self.app.query_track(query)?;
        let track = track.map(TrackModel::from);

        Ok(track)
    }
}