use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::library::{Identifiable, MetaValue};
use crate::{Album, Playlist, ProviderType};

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
    pub description: Option<String>,
}

impl PartialEq for Artist {
    fn eq(&self, other: &Artist) -> bool {
        self.uri == other.uri
    }
}

impl Eq for Artist {}

impl Identifiable for Artist {
    fn get_uri(&self) -> String {
        self.uri.clone()
    }

    fn get_id(&self) -> Option<usize> {
        self.id
    }
}
