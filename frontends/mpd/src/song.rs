use serde::Serialize;
use rustic_api::models::TrackModel;
use rustic_core::Track;

#[derive(Debug, Serialize)]
pub struct MpdSong {
    file: String,
    #[serde(rename = "Title")]
    title: Option<String>,
    #[serde(rename = "Artist")]
    artist: Option<String>,
    #[serde(rename = "Album")]
    album: Option<String>,
    #[serde(rename = "Id")]
    id: Option<usize>,
    #[serde(rename = "Track")]
    track: usize,
    #[serde(rename = "Time")]
    time: Option<u64>,
}

impl From<Track> for MpdSong {
    fn from(track: Track) -> MpdSong {
        MpdSong {
            file: track.uri,
            title: Some(track.title),
            artist: track.artist.map(|artist| artist.name),
            album: track.album.map(|album| album.title),
            id: track.id,
            track: track.position.and_then(|p| p.track).unwrap_or_default() as usize,
            time: track.duration,
        }
    }
}

impl From<TrackModel> for MpdSong {
    fn from(track: TrackModel) -> MpdSong {
        MpdSong {
            file: track.cursor,
            title: Some(track.title),
            artist: track.artist.map(|artist| artist.name),
            album: track.album.map(|album| album.title),
            id: None,
            track: 0,
            time: track.duration,
        }
    }
}
