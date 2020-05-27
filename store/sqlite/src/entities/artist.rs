use std::collections::HashMap;
use std::convert::TryInto;

use entities::provider::{int_to_provider, provider_to_int};
use rustic_core::Artist;
use rustic_core::library::MetaValue;
use schema::{artists, artists_meta};

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "artists"]
pub struct ArtistEntity {
    pub id: i32,
    pub name: String,
    pub image_url: Option<String>,
    pub uri: String,
    pub provider: i32,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(ArtistEntity, foreign_key = "artist_id")]
#[table_name = "artists_meta"]
#[primary_key(artist_id, key)]
pub struct ArtistMeta {
    pub artist_id: i32,
    pub key: String,
    pub bool_variant: Option<bool>,
    pub float_variant: Option<f32>,
    pub string_variant: Option<String>,
    pub int_variant: Option<i32>,
}

impl ArtistMeta {
    fn to_meta_map(items: &[ArtistMeta]) -> HashMap<String, MetaValue> {
        let mut map = HashMap::new();
        for item in items {
            map.insert(item.key.clone(), item.into());
        }

        map
    }
}

impl From<&ArtistMeta> for MetaValue {
    fn from(meta: &ArtistMeta) -> Self {
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

#[derive(Insertable)]
#[table_name = "artists"]
pub struct ArtistInsert {
    pub name: String,
    pub image_url: Option<String>,
    pub uri: String,
    pub provider: i32,
}

impl ArtistEntity {
    pub fn into_artist(self, meta: &[ArtistMeta]) -> Artist {
        Artist {
            id: Some(self.id as usize),
            name: self.name,
            uri: self.uri,
            image_url: self.image_url,
            meta: ArtistMeta::to_meta_map(meta),
            provider: int_to_provider(self.provider),
        }
    }
}

impl From<Artist> for ArtistInsert {
    fn from(artist: Artist) -> Self {
        ArtistInsert {
            name: artist.name,
            image_url: artist.image_url,
            uri: artist.uri,
            provider: provider_to_int(artist.provider),
        }
    }
}
