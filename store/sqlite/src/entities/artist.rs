use core::Artist;
use failure::Error;
use schema::{artists, artists_meta};

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "artists"]
pub struct ArtistEntity {
    pub id: i32,
    pub name: String,
    pub image_url: Option<String>,
    pub uri: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(ArtistEntity, foreign_key = "artist_id")]
#[table_name = "artists_meta"]
#[primary_key(artist_id, key)]
pub struct ArtistMeta {
    pub artist_id: i32,
    pub key: String,
    pub bool_variant: Option<i32>,
    pub float_variant: Option<f32>,
    pub string_variant: Option<String>,
    pub int_variant: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "artists"]
pub struct ArtistInsert {
    pub name: String,
    pub image_url: Option<String>,
    pub uri: String,
}

impl ArtistEntity {
    pub fn into_artist(self) -> Result<Artist, Error> {
        Ok(Artist {
            id: Some(self.id as usize),
            name: self.name,
            uri: self.uri,
            image_url: self.image_url,
            meta: unimplemented!(),
        })
    }
}

impl From<Artist> for ArtistInsert {
    fn from(artist: Artist) -> ArtistInsert {
        ArtistInsert {
            name: artist.name,
            image_url: artist.image_url,
            uri: artist.uri,
        }
    }
}
