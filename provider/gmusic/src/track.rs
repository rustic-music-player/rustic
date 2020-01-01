use gmusic::Track;

#[derive(Debug, Clone)]
pub struct GmusicTrack(Track);

impl From<Track> for GmusicTrack {
    fn from(track: Track) -> Self {
        GmusicTrack(track)
    }
}