use crate::library::MetaValue;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::ProviderType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artist {
    pub id: Option<usize>,
    pub name: String,
    pub uri: String,
    pub image_url: Option<String>,
    pub meta: HashMap<String, MetaValue>,
    pub provider: ProviderType,
}

impl PartialEq for Artist {
    fn eq(&self, other: &Artist) -> bool {
        self.uri == other.uri
    }
}

impl Eq for Artist {}
