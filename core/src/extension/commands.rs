use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ExtensionCommands {
    Load,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExtensionResponses {
    Load(ExtensionMetadata),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub hooks: Vec<Hook>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Hook {
    AddToQueue,
}
