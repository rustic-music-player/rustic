use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::library::{Identifiable, MetaValue};
use crate::{Album, Playlist, ProviderType, SingleQueryIdentifier};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artist {
    pub id: Option<usize>,
    pub name: String,
    pub uri: String,
    pub image_url: Option<String>,
    pub meta: HashMap<String, MetaValue>,
    pub provider: ProviderType,
    pub albums: Vec<Album>,
    pub playlists: Vec<Playlist>,
}

impl PartialEq for Artist {
    fn eq(&self, other: &Artist) -> bool {
        self.uri == other.uri
    }
}

impl Eq for Artist {}

impl Identifiable for Artist {
    fn get_identifier(&self) -> SingleQueryIdentifier {
        if let Some(id) = self.id {
            SingleQueryIdentifier::Id(id)
        } else {
            SingleQueryIdentifier::Uri(self.uri.clone())
        }
    }
}
