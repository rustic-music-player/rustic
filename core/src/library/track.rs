use std::cmp::Ordering;
use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::library::{Album, Artist, Identifiable, MetaValue, Rating};
use crate::provider::{self, ProviderType};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Track {
    pub id: Option<usize>,
    pub title: String,
    pub artist_id: Option<usize>,
    pub artist: Option<Artist>,
    pub album_id: Option<usize>,
    pub album: Option<Album>,
    pub provider: ProviderType,
    pub uri: String,
    pub thumbnail: provider::ThumbnailState,
    pub duration: Option<u64>,
    pub meta: HashMap<String, MetaValue>,
    pub explicit: Option<bool>,
    pub rating: Rating,
    pub position: Option<TrackPosition>,
    pub share_url: Option<String>,
    pub lyrics: Lyrics,
    pub comments: Option<String>,
    pub chapters: Vec<Chapter>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Lyrics {
    None,
    Plain(String),
    Timestamped(Vec<TimestampedLyric>),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Ord)]
pub struct TimestampedLyric {
    pub text: String,
    /// timestamp in seconds
    pub timestamp: u64,
}

impl PartialOrd for TimestampedLyric {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Ord)]
pub struct Chapter {
    pub label: String,
    pub description: Option<String>,
    /// timestamp in seconds
    pub timestamp: u64,
}

impl PartialOrd for Chapter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackPosition {
    pub track: Option<u64>,
    pub disc: Option<u64>,
}

impl TrackPosition {
    pub fn new(track: Option<u64>, disc: Option<u64>) -> Option<Self> {
        match (track, disc) {
            (None, None) => None,
            (track, disc) => Some(TrackPosition { track, disc }),
        }
    }
}

impl PartialEq for Track {
    fn eq(&self, other: &Track) -> bool {
        self.uri == other.uri
    }
}

impl Eq for Track {}

impl PartialOrd for Track {
    fn partial_cmp(&self, other: &Track) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Track {
    fn cmp(&self, other: &Track) -> Ordering {
        self.title.cmp(&other.title)
    }
}

impl std::fmt::Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref album) = self.album {
            write!(f, "{} - ", album.title)?;
        }
        write!(f, "{}", &self.title)
    }
}

impl Identifiable for Track {
    fn get_uri(&self) -> String {
        self.uri.clone()
    }

    fn get_id(&self) -> Option<usize> {
        self.id
    }
}
