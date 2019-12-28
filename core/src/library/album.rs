use crate::library::{Artist, MetaValue};
use crate::provider::Provider;
use crate::{Rustic, Track};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Album {
    pub id: Option<usize>,
    pub title: String,
    pub artist_id: Option<usize>,
    pub artist: Option<Artist>,
    pub tracks: Vec<Track>,
    pub provider: Provider,
    pub image_url: Option<String>,
    pub uri: String,
    pub meta: HashMap<String, MetaValue>,
}

impl Album {
    pub fn coverart(&self, app: &Arc<Rustic>) -> Option<String> {
        self.image_url
            .clone()
            .and_then(|uri| app.cache.fetch_coverart(uri).ok())
    }
}

impl PartialEq for Album {
    fn eq(&self, other: &Album) -> bool {
        self.uri == other.uri
    }
}

impl Eq for Album {}
