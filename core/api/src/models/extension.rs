use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionModel {
    pub name: String,
    pub id: String,
    pub version: String,
    pub enabled: bool,
}
