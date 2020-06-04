use std::collections::HashMap;
use std::convert::TryInto;

use entities::provider::{int_to_provider, provider_to_int};
use rustic_core::library::MetaValue;
use rustic_core::Album;
use schema::{albums, albums_meta};

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "albums"]
pub struct AlbumEntity {
    pub id: i32,
    pub title: String,
    pub artist_id: Option<i32>,
    pub image_url: Option<String>,
    pub uri: String,
    pub provider: i32,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(AlbumEntity, foreign_key = "album_id")]
#[table_name = "albums_meta"]
#[primary_key(album_id, key)]
pub struct AlbumMeta {
    pub album_id: i32,
    pub key: String,
    pub bool_variant: Option<bool>,
    pub float_variant: Option<f32>,
    pub string_variant: Option<String>,
    pub int_variant: Option<i32>,
}

impl AlbumMeta {
    fn to_meta_map(items: &[AlbumMeta]) -> HashMap<String, MetaValue> {
        let mut map = HashMap::new();
        for item in items {
            map.insert(item.key.clone(), item.into());
        }

        map
    }
}

impl From<&AlbumMeta> for MetaValue {
    fn from(meta: &AlbumMeta) -> Self {
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

impl AlbumEntity {
    pub fn into_album(self, meta: &[AlbumMeta]) -> Album {
        Album {
            id: Some(self.id as usize),
            title: self.title,
            artist_id: self.artist_id.map(|id| id as usize),
            artist: None,
            tracks: vec![],
            provider: int_to_provider(self.provider),
            image_url: self.image_url,
            uri: self.uri,
            meta: AlbumMeta::to_meta_map(meta),
        }
    }
}

#[derive(Insertable)]
#[table_name = "albums"]
pub struct AlbumInsert {
    pub title: String,
    pub artist_id: Option<i32>,
    pub image_url: Option<String>,
    pub uri: String,
    pub provider: i32,
}

impl From<Album> for AlbumInsert {
    fn from(album: Album) -> Self {
        AlbumInsert {
            title: album.title,
            artist_id: album.artist_id.map(|id| id as i32),
            uri: album.uri,
            image_url: album.image_url,
            provider: provider_to_int(album.provider),
        }
    }
}
