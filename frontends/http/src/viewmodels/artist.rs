use failure::Error;
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
    pub fn new_with_joins(artist: Artist, app: &Arc<Rustic>) -> Result<ArtistModel, Error> {
        let albums = app.library.get_albums()?;
        let tracks = app.library.get_tracks()?;
        let albums = albums
            .iter()
            .filter(|albums| albums.artist_id == artist.id)
            .cloned()
            .map(|album| AlbumModel::new(album, app))
            .collect();
        let tracks = tracks
            .iter()
            .filter(|track| track.artist_id == artist.id)
            .cloned()
            .map(|track| TrackModel::new(track, app))
            .collect();
        let image = artist.image(app);
        Ok(ArtistModel {
            id: artist.id,
            name: artist.name,
            albums: Some(albums),
            tracks: Some(tracks),
            uri: artist.uri,
            image,
        })
    }

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
