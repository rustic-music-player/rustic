use std::collections::HashMap;

use entities::provider::{int_to_provider, provider_to_int};
use rustic_core::library::MetaValue;
use rustic_core::provider::ThumbnailState;
use rustic_core::Track;
use schema::{tracks, tracks_meta};
use std::convert::TryInto;

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
    pub bool_variant: Option<bool>,
    pub float_variant: Option<f32>,
    pub string_variant: Option<String>,
    pub int_variant: Option<i32>,
}

impl TrackMeta {
    fn to_meta_map(items: &[TrackMeta]) -> HashMap<String, MetaValue> {
        let mut map = HashMap::new();
        for item in items {
            map.insert(item.key.clone(), item.into());
        }

        map
    }
}

impl From<&TrackMeta> for MetaValue {
    fn from(meta: &TrackMeta) -> Self {
        if let Some(bool) = meta.bool_variant {
            MetaValue::Bool(bool)
        } else if let Some(ref float) = meta.float_variant {
            MetaValue::Float((*float).into())
        } else if let Some(ref string) = meta.string_variant {
            MetaValue::String(string.to_string())
        } else if let Some(ref int) = meta.int_variant {
            MetaValue::Int((*int).try_into().unwrap())
        } else {
            unreachable!()
        }
    }
}

impl TrackEntity {
    pub fn into_track(self, meta: &[TrackMeta]) -> Track {
        Track {
            id: Some(self.id as usize),
            title: self.title,
            artist_id: self.artist_id.map(|id| id as usize),
            artist: None,
            album_id: self.album_id.map(|id| id as usize),
            album: None,
            provider: int_to_provider(self.provider),
            uri: self.uri,
            thumbnail: self.image_url.map(ThumbnailState::Url).unwrap_or_default(),
            duration: self.duration.map(|duration| duration as u64),
            meta: TrackMeta::to_meta_map(meta),
        }
    }
}

#[derive(Insertable)]
#[table_name = "tracks"]
pub struct TrackInsert {
    pub title: String,
    pub artist_id: Option<i32>,
    pub album_id: Option<i32>,
    pub uri: String,
    pub image_url: Option<String>,
    pub duration: Option<i32>,
    pub provider: i32,
}

impl From<Track> for TrackInsert {
    fn from(track: Track) -> Self {
        TrackInsert {
            title: track.title,
            artist_id: track.artist_id.map(|id| id as i32),
            album_id: track.album_id.map(|id| id as i32),
            uri: track.uri,
            image_url: track.thumbnail.to_url(),
            duration: track.duration.map(|id| id as i32),
            provider: provider_to_int(track.provider),
        }
    }
}
