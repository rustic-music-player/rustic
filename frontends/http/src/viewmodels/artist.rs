use rustic_core::library::Artist;
use rustic_core::Rustic;
use std::sync::Arc;
use viewmodels::{AlbumModel, TrackModel};

#[derive(Clone, Debug, Serialize)]
pub struct ArtistModel {
    pub id: Option<usize>,
    pub name: String,
    pub albums: Option<Vec<AlbumModel>>,
    pub tracks: Option<Vec<TrackModel>>,
    pub uri: String,
    pub image: Option<String>,
}

impl ArtistModel {
    pub fn new(artist: Artist, app: &Arc<Rustic>) -> ArtistModel {
        let image = artist.image(app);
        ArtistModel {
            id: artist.id,
            name: artist.name,
            albums: None,
            tracks: None,
            uri: artist.uri,
            image,
        }
    }
}
