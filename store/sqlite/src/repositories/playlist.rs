use std::sync::{Arc, Mutex};

use diesel::prelude::*;
use diesel::{insert_into, SqliteConnection};
use failure::Error;

use rustic_core::{MultiQuery, Playlist, SingleQuery, SingleQueryIdentifier};

use crate::entities::playlist::*;
use crate::repositories::Repository;

#[derive(Clone)]
pub struct PlaylistRepository {
    connection: Arc<Mutex<SqliteConnection>>,
}

impl PlaylistRepository {
    pub fn new(connection: Arc<Mutex<SqliteConnection>>) -> Self {
        PlaylistRepository { connection }
    }
}

impl Repository<Playlist> for PlaylistRepository {
    fn query(&self, query: SingleQuery) -> Result<Option<Playlist>, Error> {
        use schema::playlists::dsl::*;

        let connection = self.connection.lock().unwrap();

        let playlist = match query.identifier {
            SingleQueryIdentifier::Id(playlist_id) => playlists
                .find(playlist_id as i32)
                .first::<PlaylistEntity>(&*connection),
            SingleQueryIdentifier::Uri(query_uri) => playlists
                .filter(uri.eq(query_uri))
                .first::<PlaylistEntity>(&*connection),
        }
        .optional()?;

        let playlist = match playlist {
            Some(playlist) => {
                // let tracks = PlaylistTrack::belonging_to(&playlist).load::<PlaylistTrack>(&*connection)?;
                Some(playlist.into_playlist(vec![]))
            }
            None => None,
        };

        Ok(playlist)
    }

    // TODO: use query
    fn query_all(&self, query: MultiQuery) -> Result<Vec<Playlist>, Error> {
        use schema::playlists::dsl::*;

        let connection = self.connection.lock().unwrap();

        let playlist_list = playlists.load::<PlaylistEntity>(&*connection)?;

        let playlist_list = playlist_list
            .into_iter()
            .map(|entity| entity.into_playlist(vec![]))
            .collect();

        Ok(playlist_list)
    }

    fn insert(&self, playlist: &mut Playlist) -> Result<(), Error> {
        use crate::schema::playlists::dsl::*;

        let connection = self.connection.lock().unwrap();

        let entity: PlaylistInsert = playlist.clone().into();

        insert_into(playlists)
            .values(&entity)
            .execute(&*connection)?;

        // TODO: update model id

        Ok(())
    }

    fn insert_all(&self, models: &mut Vec<Playlist>) -> Result<(), Error> {
        use crate::schema::playlists::dsl::*;

        let connection = self.connection.lock().unwrap();

        let entities = models
            .iter()
            .cloned()
            .map(PlaylistInsert::from)
            .collect::<Vec<_>>();

        insert_into(playlists)
            .values(&entities)
            .execute(&*connection)?;

        // TODO: update model ids

        Ok(())
    }

    fn update(&self, model: &mut Playlist) -> Result<(), Error> {
        unimplemented!()
    }

    fn update_all(&self, models: &mut Vec<Playlist>) -> Result<(), Error> {
        unimplemented!()
    }
}
