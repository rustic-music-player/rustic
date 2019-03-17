use failure::Error;

use core::Album;
use entities::provider::int_to_provider;

#[derive(Queryable)]
pub struct AlbumEntity {
    pub id: i32,
    pub title: String,
    pub artist_id: Option<i32>,
    pub image_url: Option<String>,
    pub uri: String,
    pub provider: i32,
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
            image_url: self.image_url,
            meta: unimplemented!(),
        })
    }
}
