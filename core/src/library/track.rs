use crate::library::{Album, Artist, MetaValue};
use crate::provider::Provider;
use crate::Rustic;
use serde_derive::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;

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
    pub image_url: Option<String>,
    pub duration: Option<u64>,
    pub meta: HashMap<String, MetaValue>,
}

impl Track {
    pub fn coverart(&self, app: &Arc<Rustic>) -> Option<String> {
        self.image_url
            .clone()
            .and_then(|uri| app.cache.fetch_coverart(uri).ok())
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
