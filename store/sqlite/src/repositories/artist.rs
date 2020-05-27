use std::sync::{Arc, Mutex};

use diesel::prelude::*;
use diesel::{SqliteConnection, insert_into};
use failure::Error;

use rustic_core::{Artist, MultiQuery, SingleQuery, SingleQueryIdentifier};

use crate::entities::artist::*;
use crate::repositories::Repository;

#[derive(Clone)]
pub struct ArtistRepository {
    connection: Arc<Mutex<SqliteConnection>>
}

impl ArtistRepository {
    pub fn new(connection: Arc<Mutex<SqliteConnection>>) -> Self {
        ArtistRepository {
            connection
        }
    }
}

impl Repository<Artist> for ArtistRepository {
    fn query(&self, query: SingleQuery) -> Result<Option<Artist>, Error> {
        use crate::schema::artists::dsl::*;

        let connection = self.connection.lock().unwrap();

        let artist = match query.identifier {
            SingleQueryIdentifier::Id(artist_id) => artists
                .find(artist_id as i32)
                .first::<ArtistEntity>(&*connection),
            SingleQueryIdentifier::Uri(query_uri) => artists
                .filter(uri.eq(query_uri))
                .first::<ArtistEntity>(&*connection),
        }.optional()?;

        let artist = match artist {
            Some(artist) => {
                let meta = ArtistMeta::belonging_to(&artist).load::<ArtistMeta>(&*connection)?;
                Some(artist.into_artist(&meta))
            },
            None => None
        };

        Ok(artist)
    }

    // TODO: use query
    fn query_all(&self, query: MultiQuery) -> Result<Vec<Artist>, Error> {
        use crate::schema::artists::dsl::*;

        let connection = self.connection.lock().unwrap();

        let artist_list = artists.load::<ArtistEntity>(&*connection)?;
        let meta = ArtistMeta::belonging_to(&artist_list).load::<ArtistMeta>(&*connection)?.grouped_by(&artist_list);
        let data = artist_list.into_iter().zip(meta).collect::<Vec<_>>();

        let artist_list = data
            .into_iter()
            .map(|(artist, meta)| artist.into_artist(&meta))
            .collect();

        Ok(artist_list)
    }

    fn insert(&self, artist: &mut Artist) -> Result<(), Error> {
        use crate::schema::artists::dsl::*;

        let connection = self.connection.lock().unwrap();

        let entity: ArtistInsert = artist.clone().into();

        insert_into(artists).values(&entity).execute(&*connection)?;

        // TODO: update model id

        Ok(())
    }

    fn insert_all(&self, models: &mut Vec<Artist>) -> Result<(), Error> {
        use crate::schema::artists::dsl::*;

        let connection = self.connection.lock().unwrap();

        let entities = models
            .iter()
            .cloned()
            .map(ArtistInsert::from)
            .collect::<Vec<_>>();

        insert_into(artists).values(&entities).execute(&*connection)?;

        // TODO: update model ids

        Ok(())
    }

    fn update(&self, model: &mut Artist) -> Result<(), Error> {
        unimplemented!()
    }

    fn update_all(&self, models: &mut Vec<Artist>) -> Result<(), Error> {
        unimplemented!()
    }
}
