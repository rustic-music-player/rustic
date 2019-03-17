use rustic_core::Track;

#[derive(Debug, Serialize)]
pub struct MpdSong {
    file: String,
    #[serde(rename = "Title")]
    title: Option<String>,
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
            id: track.id,
            track: 0,
            time: track.duration,
        }
    }
}
