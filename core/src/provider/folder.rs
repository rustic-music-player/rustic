use crate::provider::item::ProviderItem;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderFolder {
    pub folders: Vec<String>,
    pub items: Vec<ProviderItem>,
}

impl ProviderFolder {
    pub fn new(folders: Vec<String>, items: Vec<ProviderItem>) -> ProviderFolder {
        ProviderFolder { folders, items }
    }

    pub fn empty() -> ProviderFolder {
        ProviderFolder {
            folders: vec![],
            items: vec![],
        }
    }
}
