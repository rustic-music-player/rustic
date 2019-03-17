use failure::Error;
use rustic_core::library::Track;
use rustic_core::provider::Provider;
use rustic_core::Rustic;
use std::sync::Arc;
use viewmodels::AlbumModel;
use viewmodels::ArtistModel;

#[derive(Clone, Debug, Serialize)]
pub struct TrackModel {
    pub id: Option<usize>,
    pub title: String,
    pub artist: Option<ArtistModel>,
    pub album: Option<AlbumModel>,
    pub uri: String,
    pub provider: Provider,
    pub coverart: Option<String>,
    pub duration: Option<u64>,
}

impl TrackModel {
    pub fn new_with_joins(track: Track, app: &Arc<Rustic>) -> Result<TrackModel, Error> {
        let artist = match track.artist_id {
            Some(id) => app.library.get_artist(id)?,
            None => track.artist.clone(),
        }.map(|artist| ArtistModel::new(artist, app));
        let album = match track.album_id {
            Some(id) => app.library.get_album(id)?,
            None => track.album.clone(),
        }.map(|album| AlbumModel::new(album, app));
        let coverart = track.coverart(app);
        Ok(TrackModel {
            id: track.id,
            title: track.title,
            uri: track.uri,
            provider: track.provider,
            coverart,
            duration: track.duration,
            artist,
            album,
        })
    }

    pub fn new(track: Track, app: &Arc<Rustic>) -> TrackModel {
        let coverart = track.coverart(app);
        TrackModel {
            id: track.id,
            title: track.title,
            uri: track.uri,
            provider: track.provider,
            coverart,
            duration: track.duration,
            artist: track.artist.map(|artist| ArtistModel::new(artist, app)),
            album: track.album.map(|album| AlbumModel::new(album, app)),
        }
    }
}
