use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use log::info;
use serde::{Deserialize, Serialize};

use rustic_core::Track;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ExtensionConfigValue {
    Bool(bool),
    String(String),
    Float(f64),
    Int(i64),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtensionMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
}

pub trait ExtensionLibrary {
    fn metadata() -> ExtensionMetadata;

    fn new(config: HashMap<String, ExtensionConfigValue>) -> Box<dyn Extension>;
}

pub trait Extension: std::fmt::Debug + Send + Sync + ExtensionApi {
    fn setup(&mut self) -> Result<(), failure::Error> {
        Ok(())
    }
}

#[async_trait]
pub trait ExtensionApi {
    async fn on_add_to_queue(&self, tracks: Vec<Track>) -> Result<Vec<Track>, failure::Error> {
        Ok(tracks)
    }
}

#[derive(Debug, Default)]
pub struct ExtensionManagerBuilder {
    extensions: Vec<(ExtensionMetadata, Box<dyn Extension>)>,
}

impl ExtensionManagerBuilder {
    pub fn load<T>(&mut self, config: HashMap<String, ExtensionConfigValue>)
    where
        T: ExtensionLibrary,
    {
        let extension = T::new(config);
        let metadata = T::metadata();
        info!("Loaded Extension: {} v{}", metadata.name, metadata.version);
        self.extensions.push((metadata, extension));
    }

    pub fn build(self) -> ExtensionManager {
        ExtensionManager {
            extensions: Arc::new(self.extensions),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExtensionManager {
    extensions: Arc<Vec<(ExtensionMetadata, Box<dyn Extension>)>>,
}

impl ExtensionManager {
    pub fn get_extensions(&self) -> Vec<ExtensionMetadata> {
        self.extensions.iter().map(|(m, _)| m).cloned().collect()
    }
}

#[async_trait]
impl ExtensionApi for ExtensionManager {
    async fn on_add_to_queue(&self, tracks: Vec<Track>) -> Result<Vec<Track>, failure::Error> {
        let mut tracks = tracks;
        for (_, extension) in self.extensions.iter() {
            tracks = extension.on_add_to_queue(tracks).await?;
        }

        Ok(tracks)
    }
}

#[macro_export]
macro_rules! crate_version {
    () => {
        format!(
            "{}.{}.{}{}",
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR"),
            env!("CARGO_PKG_VERSION_PATCH"),
            option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")
        )
    };
}
