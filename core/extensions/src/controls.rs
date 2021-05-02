use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ExtensionControls {
    pub actions: Vec<ExtensionAction>,
    pub infos: Vec<ExtensionInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExtensionAction {
    pub key: String,
    pub label: String,
}

impl From<(&str, &str)> for ExtensionAction {
    fn from((key, label): (&str, &str)) -> Self {
        ExtensionAction {
            key: key.to_string(),
            label: label.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ExtensionInfo {
    Link(String),
}
