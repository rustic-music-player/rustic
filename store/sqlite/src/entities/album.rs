use failure::Error;

use core::Album;
use entities::provider::int_to_provider;
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
    pub bool_variant: Option<i32>,
    pub float_variant: Option<f32>,
    pub string_variant: Option<String>,
    pub int_variant: Option<i32>
}

impl AlbumEntity {
    pub fn into_album(self) -> Result<Album, Error> {
        Ok(Album {
            id: Some(self.id as usize),
            title: self.title,
            artist_id: self.artist_id.map(|id| id as usize),
            artist: None,
            provider: int_to_provider(self.provider)?,
            uri: self.uri,
            tracks: vec![],
            image_url: self.image_url,
            meta: unimplemented!(),
        })
    }
}
