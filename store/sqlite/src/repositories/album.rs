use std::sync::{Arc, Mutex};

use diesel::{insert_into, SqliteConnection};
use diesel::prelude::*;
use failure::Error;

use rustic_core::{Album, MultiQuery, SingleQuery};
use rustic_core::library::LibraryItemIdentifier;

use crate::entities::album::*;
use crate::repositories::Repository;

#[derive(Clone)]
pub struct AlbumRepository {
    connection: Arc<Mutex<SqliteConnection>>,
}

impl AlbumRepository {
    pub fn new(connection: Arc<Mutex<SqliteConnection>>) -> Self {
        AlbumRepository { connection }
    }
}

impl Repository<Album> for AlbumRepository {
    fn query(&self, query: SingleQuery) -> Result<Option<Album>, Error> {
        use schema::albums::dsl::*;

        let connection = self.connection.lock().unwrap();

        let album = match query.identifier {
            LibraryItemIdentifier::Id(album_id) => albums
                .find(album_id as i32)
                .first::<AlbumEntity>(&*connection),
            LibraryItemIdentifier::Uri(query_uri) => albums
                .filter(uri.eq(query_uri))
                .first::<AlbumEntity>(&*connection),
        }
        .optional()?;

        let album = match album {
            Some(album) => {
                let meta = AlbumMeta::belonging_to(&album).load::<AlbumMeta>(&*connection)?;
                Some(album.into_album(&meta))
            }
            None => None,
        };

        Ok(album)
    }

    // TODO: use query
    fn query_all(&self, query: MultiQuery) -> Result<Vec<Album>, Error> {
        use schema::albums::dsl::*;

        let connection = self.connection.lock().unwrap();

        let album_list = albums.load::<AlbumEntity>(&*connection)?;
        let meta = AlbumMeta::belonging_to(&album_list)
            .load::<AlbumMeta>(&*connection)?
            .grouped_by(&album_list);
        let data = album_list.into_iter().zip(meta).collect::<Vec<_>>();

        let album_list = data
            .into_iter()
            .map(|(album, meta)| album.into_album(&meta))
            .collect();

        Ok(album_list)
    }

    fn insert(&self, album: &mut Album) -> Result<(), Error> {
        use crate::schema::albums::dsl::*;

        let connection = self.connection.lock().unwrap();

        let entity: AlbumInsert = album.clone().into();

        insert_into(albums).values(&entity).execute(&*connection)?;

        // TODO: update model id

        Ok(())
    }

    fn insert_all(&self, models: &mut Vec<Album>) -> Result<(), Error> {
        use crate::schema::albums::dsl::*;

        let connection = self.connection.lock().unwrap();

        let entities = models
            .iter()
            .cloned()
            .map(AlbumInsert::from)
            .collect::<Vec<_>>();

        insert_into(albums)
            .values(&entities)
            .execute(&*connection)?;

        // TODO: update model ids

        Ok(())
    }

    fn update(&self, model: &mut Album) -> Result<(), Error> {
        unimplemented!()
    }

    fn update_all(&self, models: &mut Vec<Album>) -> Result<(), Error> {
        unimplemented!()
    }
}
