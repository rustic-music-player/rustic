use failure::Error;

use core::Track;
use entities::provider::int_to_provider;
use schema::{tracks, tracks_meta};

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "tracks"]
pub struct TrackEntity {
    pub id: i32,
    pub title: String,
    pub artist_id: Option<i32>,
    pub album_id: Option<i32>,
    pub uri: String,
    pub image_url: Option<String>,
    pub duration: Option<i32>,
    pub provider: i32,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(TrackEntity, foreign_key = "track_id")]
#[table_name = "tracks_meta"]
#[primary_key(track_id, key)]
pub struct TrackMeta {
    pub track_id: i32,
    pub key: String,
    pub bool_variant: Option<i32>,
    pub float_variant: Option<f32>,
    pub string_variant: Option<String>,
    pub int_variant: Option<i32>
}

impl TrackEntity {
    pub fn into_track(self) -> Result<Track, Error> {
        Ok(Track {
            id: Some(self.id as usize),
            title: self.title,
            artist_id: self.artist_id.map(|id| id as usize),
            artist: None,
            album_id: self.album_id.map(|id| id as usize),
            album: None,
            provider: int_to_provider(self.provider)?,
            uri: self.uri,
            image_url: self.image_url,
            duration: self.duration.map(|duration| duration as u64),
            meta: unimplemented!(),
        })
    }
}
