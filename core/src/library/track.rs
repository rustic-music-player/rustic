use std::cmp::Ordering;
use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::library::{Album, Artist, MetaValue};
use crate::provider::Provider;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Track {
    pub id: Option<usize>,
    pub title: String,
    pub artist_id: Option<usize>,
    pub artist: Option<Artist>,
    pub album_id: Option<usize>,
    pub album: Option<Album>,
    pub provider: Provider,
    pub uri: String,
    pub has_coverart: bool,
    pub duration: Option<u64>,
    pub meta: HashMap<String, MetaValue>,
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
