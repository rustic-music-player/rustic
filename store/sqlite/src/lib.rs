#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate rustic_core;

use std::sync::{Arc, Mutex};

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use failure::Error;

use rustic_core::{Album, Artist, MultiQuery, Playlist, SearchResults, SingleQuery, Track};

use crate::repositories::*;

embed_migrations!();

mod entities;
mod repositories;
mod schema;

#[derive(Clone)]
pub struct SqliteLibrary {
    connection: Arc<Mutex<SqliteConnection>>,
    albums: AlbumRepository,
    artists: ArtistRepository,
    tracks: TrackRepository,
    playlists: PlaylistRepository,
}

impl SqliteLibrary {
    pub fn new(url: String) -> Result<SqliteLibrary, Error> {
        debug!("Initializing connection");
        let connection = SqliteConnection::establish(&url)?;

        debug!("Migrating Database");
        embedded_migrations::run(&connection)?;

        let connection = Arc::new(Mutex::new(connection));
        let album_repository = AlbumRepository::new(Arc::clone(&connection));
        let artist_repository = ArtistRepository::new(Arc::clone(&connection));
        let track_repository = TrackRepository::new(Arc::clone(&connection));
        let playlist_repository = PlaylistRepository::new(Arc::clone(&connection));

        Ok(SqliteLibrary {
            connection,
            albums: album_repository,
            artists: artist_repository,
            tracks: track_repository,
            playlists: playlist_repository,
        })
    }
}

impl rustic_core::Library for SqliteLibrary {
    fn query_track(&self, query: SingleQuery) -> Result<Option<Track>, Error> {
        self.tracks.query(query)
    }

    fn query_tracks(&self, query: MultiQuery) -> Result<Vec<Track>, Error> {
        self.tracks.query_all(query)
    }

    fn query_album(&self, query: SingleQuery) -> Result<Option<Album>, Error> {
        self.albums.query(query)
    }

    fn query_albums(&self, query: MultiQuery) -> Result<Vec<Album>, Error> {
        self.albums.query_all(query)
    }

    fn query_artist(&self, query: SingleQuery) -> Result<Option<Artist>, Error> {
        self.artists.query(query)
    }

    fn query_artists(&self, query: MultiQuery) -> Result<Vec<Artist>, Error> {
        self.artists.query_all(query)
    }

    fn query_playlist(&self, query: SingleQuery) -> Result<Option<Playlist>, Error> {
        self.playlists.query(query)
    }

    fn query_playlists(&self, query: MultiQuery) -> Result<Vec<Playlist>, Error> {
        self.playlists.query_all(query)
    }

    fn add_track(&self, track: &mut Track) -> Result<(), Error> {
        self.tracks.insert(track)
    }

    fn add_album(&self, album: &mut Album) -> Result<(), Error> {
        self.albums.insert(album)
    }

    fn add_artist(&self, artist: &mut Artist) -> Result<(), Error> {
        self.artists.insert(artist)
    }

    fn add_playlist(&self, playlist: &mut Playlist) -> Result<(), Error> {
        self.playlists.insert(playlist)
    }

    fn add_tracks(&self, tracks: &mut Vec<Track>) -> Result<(), Error> {
        self.tracks.insert_all(tracks)
    }

    fn add_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error> {
        self.albums.insert_all(albums)
    }

    fn add_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error> {
        self.artists.insert_all(artists)
    }

    fn add_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        self.playlists.insert_all(playlists)
    }

    fn sync_track(&self, track: &mut Track) -> Result<(), Error> {
        self.tracks.sync(track)
    }

    fn sync_album(&self, album: &mut Album) -> Result<(), Error> {
        self.albums.sync(album)
    }

    fn sync_artist(&self, artist: &mut Artist) -> Result<(), Error> {
        self.artists.sync(artist)
    }

    fn sync_playlist(&self, playlist: &mut Playlist) -> Result<(), Error> {
        self.playlists.sync(playlist)
    }

    fn sync_tracks(&self, tracks: &mut Vec<Track>) -> Result<(), Error> {
        self.tracks.sync_all(tracks)
    }

    fn sync_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error> {
        self.albums.sync_all(albums)
    }

    fn sync_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error> {
        self.artists.sync_all(artists)
    }

    fn sync_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        self.playlists.sync_all(playlists)
    }

    fn remove_track(&self, _track: &Track) -> Result<(), Error> {
        unimplemented!()
    }

    fn remove_album(&self, _album: &Album) -> Result<(), Error> {
        unimplemented!()
    }

    fn remove_artist(&self, _artist: &Artist) -> Result<(), Error> {
        unimplemented!()
    }

    fn remove_playlist(&self, _playlist: &Playlist) -> Result<(), Error> {
        unimplemented!()
    }

    fn search(&self, _query: String) -> Result<SearchResults, Error> {
        unimplemented!()
    }
}
