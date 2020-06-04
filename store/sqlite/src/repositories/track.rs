use std::sync::{Arc, Mutex};

use diesel::prelude::*;
use diesel::{insert_into, SqliteConnection};
use failure::Error;

use rustic_core::{MultiQuery, SingleQuery, SingleQueryIdentifier, Track};

use crate::entities::track::*;
use crate::repositories::Repository;

#[derive(Clone)]
pub struct TrackRepository {
    connection: Arc<Mutex<SqliteConnection>>,
}

impl TrackRepository {
    pub fn new(connection: Arc<Mutex<SqliteConnection>>) -> Self {
        TrackRepository { connection }
    }
}

impl Repository<Track> for TrackRepository {
    fn query(&self, query: SingleQuery) -> Result<Option<Track>, Error> {
        use schema::tracks::dsl::*;

        let connection = self.connection.lock().unwrap();

        let track = match query.identifier {
            SingleQueryIdentifier::Id(track_id) => tracks
                .find(track_id as i32)
                .first::<TrackEntity>(&*connection),
            SingleQueryIdentifier::Uri(query_uri) => tracks
                .filter(uri.eq(query_uri))
                .first::<TrackEntity>(&*connection),
        }
        .optional()?;

        let track = match track {
            Some(track) => {
                let meta = TrackMeta::belonging_to(&track).load::<TrackMeta>(&*connection)?;
                Some(track.into_track(&meta))
            }
            None => None,
        };

        Ok(track)
    }

    // TODO: use query
    fn query_all(&self, query: MultiQuery) -> Result<Vec<Track>, Error> {
        use schema::tracks::dsl::*;

        let connection = self.connection.lock().unwrap();

        let track_list = tracks.load::<TrackEntity>(&*connection)?;
        let meta = TrackMeta::belonging_to(&track_list)
            .load::<TrackMeta>(&*connection)?
            .grouped_by(&track_list);
        let data = track_list.into_iter().zip(meta).collect::<Vec<_>>();

        let track_list = data
            .into_iter()
            .map(|(entity, meta)| entity.into_track(&meta))
            .collect();

        Ok(track_list)
    }

    fn insert(&self, track: &mut Track) -> Result<(), Error> {
        use crate::schema::tracks::dsl::*;

        let connection = self.connection.lock().unwrap();

        let entity: TrackInsert = track.clone().into();

        insert_into(tracks).values(&entity).execute(&*connection)?;

        // TODO: update model id

        Ok(())
    }

    fn insert_all(&self, models: &mut Vec<Track>) -> Result<(), Error> {
        use crate::schema::tracks::dsl::*;

        let connection = self.connection.lock().unwrap();

        let entities = models
            .iter()
            .cloned()
            .map(TrackInsert::from)
            .collect::<Vec<_>>();

        insert_into(tracks)
            .values(&entities)
            .execute(&*connection)?;

        // TODO: update model ids

        Ok(())
    }

    fn update(&self, model: &mut Track) -> Result<(), Error> {
        unimplemented!()
    }

    fn update_all(&self, models: &mut Vec<Track>) -> Result<(), Error> {
        unimplemented!()
    }
}
