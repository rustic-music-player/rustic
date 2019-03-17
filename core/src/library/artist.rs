use serde_derive::Serialize;
use crate::library::MetaValue;
use std::sync::Arc;
use std::collections::HashMap;
use crate::Rustic;

#[derive(Clone, Debug, Serialize)]
pub struct Artist {
    pub id: Option<usize>,
    pub name: String,
    pub uri: String,
    pub image_url: Option<String>,
    pub meta: HashMap<&'static str, MetaValue>
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
