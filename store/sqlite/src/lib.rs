#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;
extern crate rustic_core;
extern crate rustic_core as core;

embed_migrations!();

mod entities;
mod schema;

use diesel::insert_into;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use core::{Album, Artist, Playlist, SearchResults, Track};
use std::sync::{Arc, Mutex};

use failure::Error;

#[derive(Clone)]
pub struct SqliteLibrary {
    connection: Arc<Mutex<SqliteConnection>>,
}

impl SqliteLibrary {
    pub fn new(url: String) -> Result<SqliteLibrary, Error> {
        debug!("Initializing connection");
        let connection = SqliteConnection::establish(&url)?;

        debug!("Migrating Database");
        embedded_migrations::run(&connection)?;

        Ok(SqliteLibrary {
            connection: Arc::new(Mutex::new(connection)),
        })
    }
}

impl core::Library for SqliteLibrary {
    fn get_track(&self, track_id: usize) -> Result<Option<Track>, Error> {
        use schema::tracks::dsl::*;

        let connection = self.connection.lock().unwrap();

        tracks
            .find(track_id as i32)
            .first::<entities::TrackEntity>(&*connection)
            .optional()
            .map_err(Error::from)
            .and_then(|track| match track {
                Some(entity) => Ok(Some(entity.into_track()?)),
                None => Ok(None),
            })
    }

    fn get_tracks(&self) -> Result<Vec<Track>, Error> {
        use schema::tracks::dsl::*;

        let connection = self.connection.lock().unwrap();

        let track_list = tracks.load::<entities::TrackEntity>(&*connection)?;

        track_list
            .into_iter()
            .map(|entity| entity.into_track())
            .collect()
    }

    fn get_album(&self, album_id: usize) -> Result<Option<Album>, Error> {
        use schema::albums::dsl::*;

        let connection = self.connection.lock().unwrap();

        albums
            .find(album_id as i32)
            .first::<entities::AlbumEntity>(&*connection)
            .optional()
            .map_err(Error::from)
            .and_then(|album| match album {
                Some(entity) => Ok(Some(entity.into_album()?)),
                None => Ok(None),
            })
    }

    fn get_albums(&self) -> Result<Vec<Album>, Error> {
        use schema::albums::dsl::*;

        let connection = self.connection.lock().unwrap();

        let album_list = albums.load::<entities::AlbumEntity>(&*connection)?;

        album_list
            .into_iter()
            .map(|entity| entity.into_album())
            .collect()
    }

    fn get_artist(&self, artist_id: usize) -> Result<Option<Artist>, Error> {
        use schema::artists::dsl::*;

        let connection = self.connection.lock().unwrap();

        artists
            .find(artist_id as i32)
            .first::<entities::ArtistEntity>(&*connection)
            .optional()
            .map_err(Error::from)
            .and_then(|artist| match artist {
                Some(entity) => Ok(Some(entity.into_artist()?)),
                None => Ok(None),
            })
    }

    fn get_artists(&self) -> Result<Vec<Artist>, Error> {
        use schema::artists::dsl::*;

        let connection = self.connection.lock().unwrap();

        let artist_list = artists.load::<entities::ArtistEntity>(&*connection)?;

        artist_list
            .into_iter()
            .map(|entity| entity.into_artist())
            .collect()
    }

    fn get_playlist(&self, _playlist_id: usize) -> Result<Option<Playlist>, Error> {
        unimplemented!()
    }

    fn get_playlists(&self) -> Result<Vec<Playlist>, Error> {
        unimplemented!()
    }

    fn add_track(&self, _track: &mut Track) -> Result<(), Error> {
        unimplemented!()
    }

    fn add_album(&self, _album: &mut Album) -> Result<(), Error> {
        unimplemented!()
    }

    fn add_artist(&self, artist: &mut Artist) -> Result<(), Error> {
        use schema::artists::dsl::*;

        let connection = self.connection.lock().unwrap();

        let entity: entities::ArtistInsert = artist.clone().into();

        insert_into(artists).values(&entity).execute(&*connection)?;

        Ok(())
    }

    fn add_playlist(&self, _playlist: &mut Playlist) -> Result<(), Error> {
        unimplemented!()
    }

    fn add_tracks(&self, _tracks: &mut Vec<Track>) -> Result<(), Error> {
        unimplemented!()
    }

    fn add_albums(&self, _albums: &mut Vec<Album>) -> Result<(), Error> {
        unimplemented!()
    }

    fn add_artists(&self, _artists: &mut Vec<Artist>) -> Result<(), Error> {
        unimplemented!()
    }

    fn add_playlists(&self, _playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        unimplemented!()
    }

    fn sync_track(&self, _track: &mut Track) -> Result<(), Error> {
        unimplemented!()
    }

    fn sync_album(&self, _album: &mut Album) -> Result<(), Error> {
        unimplemented!()
    }

    fn sync_artist(&self, _artist: &mut Artist) -> Result<(), Error> {
        unimplemented!()
    }

    fn sync_playlist(&self, _playlist: &mut Playlist) -> Result<(), Error> {
        unimplemented!()
    }

    fn sync_tracks(&self, _tracks: &mut Vec<Track>) -> Result<(), Error> {
        unimplemented!()
    }

    fn sync_albums(&self, _albums: &mut Vec<Album>) -> Result<(), Error> {
        unimplemented!()
    }

    fn sync_artists(&self, _artists: &mut Vec<Artist>) -> Result<(), Error> {
        unimplemented!()
    }

    fn sync_playlists(&self, _playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        unimplemented!()
    }

    fn search(&self, _query: String) -> Result<SearchResults, Error> {
        unimplemented!()
    }
}
