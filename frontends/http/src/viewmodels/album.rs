use failure::Error;
use rustic_core::library::Album;
use rustic_core::provider::Provider;
use rustic_core::Rustic;
use std::sync::Arc;
use viewmodels::{ArtistModel, TrackModel};

#[derive(Clone, Debug, Serialize)]
pub struct AlbumModel {
    pub id: Option<usize>,
    pub title: String,
    pub artist: Option<ArtistModel>,
    pub tracks: Option<Vec<TrackModel>>,
    pub provider: Provider,
    pub coverart: Option<String>,
    pub uri: String,
}

impl AlbumModel {
    pub fn new(album: Album, app: &Arc<Rustic>) -> AlbumModel {
        let coverart = album.coverart(app);
        AlbumModel {
            id: album.id,
            title: album.title,
            artist: album.artist.map(|artist| ArtistModel::new(artist, app)),
            tracks: None,
            provider: album.provider,
            coverart,
            uri: album.uri,
        }
    }

    pub fn new_with_joins(album: Album, app: &Arc<Rustic>) -> Result<AlbumModel, Error> {
        let tracks = app.library.get_tracks()?;
        let artists = app.library.get_artists()?;
        let tracks = tracks
            .iter()
            .filter(|track| track.album_id == album.id)
            .cloned()
            .map(|track| TrackModel::new(track, app))
            .collect();
        let artist = artists
            .iter()
            .cloned()
            .find(|artist| artist.id == album.artist_id)
            .or_else(|| album.artist.clone())
            .map(|artist| ArtistModel::new(artist, app));
        let coverart = album.coverart(app);
        Ok(AlbumModel {
            id: album.id,
            title: album.title,
            artist,
            tracks: Some(tracks),
            provider: album.provider,
            coverart,
            uri: album.uri,
        })
    }
}
