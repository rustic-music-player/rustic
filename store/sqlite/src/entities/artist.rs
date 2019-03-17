use core::Artist;
use failure::Error;
use schema::artists;

#[derive(Queryable)]
pub struct ArtistEntity {
    pub id: i32,
    pub name: String,
    pub image_url: Option<String>,
    pub uri: String,
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
            meta: unimplemented!()
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
