use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::library::{Artist, Identifiable, MetaValue};
use crate::provider::ProviderType;
use crate::{SingleQueryIdentifier, Track};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Album {
    pub id: Option<usize>,
    pub title: String,
    pub artist_id: Option<usize>,
    pub artist: Option<Artist>,
    pub tracks: Vec<Track>,
    pub provider: ProviderType,
    pub image_url: Option<String>,
    pub uri: String,
    pub meta: HashMap<String, MetaValue>,
}

impl PartialEq for Album {
    fn eq(&self, other: &Album) -> bool {
        self.uri == other.uri
    }
}

impl Eq for Album {}

impl Identifiable for Album {
    fn get_identifier(&self) -> SingleQueryIdentifier {
        if let Some(id) = self.id {
            SingleQueryIdentifier::Id(id)
        } else {
            SingleQueryIdentifier::Uri(self.uri.clone())
        }
    }
}
