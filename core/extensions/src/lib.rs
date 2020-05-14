use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use async_trait::async_trait;
use rustic_core::Track;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ExtensionConfigValue {
    Bool(bool),
    String(String),
    Float(f64),
    Int(i64),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
}

pub trait ExtensionLibrary {
    fn new() -> Box<dyn Extension>;
}

#[async_trait]
pub trait Extension: std::fmt::Debug + Send + Sync {
    fn metadata(&self) -> ExtensionMetadata;

    fn setup(
        &mut self,
        _config: Option<HashMap<String, ExtensionConfigValue>>,
    ) -> Result<(), failure::Error> {
        Ok(())
    }

    fn on_add_to_queue(&mut self, tracks: Vec<Track>) -> Result<Vec<Track>, failure::Error> {
        Ok(tracks)
    }
}

#[derive(Debug, Default)]
pub struct ExtensionManagerBuilder {
    extensions: Vec<Box<dyn Extension>>
}

impl ExtensionManagerBuilder {
    pub fn load<T>(&mut self) where T: ExtensionLibrary {
        let extension = T::new();
        self.extensions.push(extension);
    }

    pub fn build(self) -> ExtensionManager {
        ExtensionManager {
            extensions: Arc::new(self.extensions)
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExtensionManager {
    extensions: Arc<Vec<Box<dyn Extension>>>
}

impl ExtensionManager {
    pub fn get_extensions(&self) -> Vec<ExtensionMetadata> {
        self.extensions.iter()
            .map(|e| e.metadata())
            .collect()
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
