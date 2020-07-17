use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::library::{Artist, Identifiable, MetaValue};
use crate::provider::{ProviderType, ThumbnailState};
use crate::Track;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Album {
    pub id: Option<usize>,
    pub title: String,
    pub artist_id: Option<usize>,
    pub artist: Option<Artist>,
    pub tracks: Vec<Track>,
    pub provider: ProviderType,
    pub thumbnail: ThumbnailState,
    pub uri: String,
    pub meta: HashMap<String, MetaValue>,
    pub explicit: Option<bool>,
    pub description: Option<String>,
}

impl PartialEq for Album {
    fn eq(&self, other: &Album) -> bool {
        self.uri == other.uri
    }
}

impl Eq for Album {}

impl Identifiable for Album {
    fn get_uri(&self) -> String {
        self.uri.clone()
    }

    fn get_id(&self) -> Option<usize> {
        self.id
    }
}
