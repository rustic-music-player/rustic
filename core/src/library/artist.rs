use crate::library::MetaValue;
use crate::Rustic;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artist {
    pub id: Option<usize>,
    pub name: String,
    pub uri: String,
    pub image_url: Option<String>,
    pub meta: HashMap<String, MetaValue>,
}

impl Artist {
    pub fn image(&self, app: &Arc<Rustic>) -> Option<String> {
        self.image_url
            .clone()
            .and_then(|uri| app.cache.fetch_coverart(uri).ok())
    }
}

impl PartialEq for Artist {
    fn eq(&self, other: &Artist) -> bool {
        self.uri == other.uri
    }
}

impl Eq for Artist {}
