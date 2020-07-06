use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use failure::format_err;
use log::{info, trace};
use tokio::sync::{mpsc, Mutex};

use rustic_core::Track;

use crate::api::*;
use crate::host::{construct_plugin, ExtensionHost};
use crate::plugin::*;
use crate::runtime::ExtensionRuntime;

#[derive(Clone)]
pub struct HostedExtension(ExtensionMetadata, Arc<Mutex<ExtensionHost>>);

impl HostedExtension {
    pub fn get_metadata(&self) -> ExtensionMetadata {
        self.0.clone()
    }

    pub async fn send(&self, message: ExtensionCommand) {
        let mut host = self.1.lock().await;
        host.send(message).await;
    }
}

impl From<(ExtensionMetadata, ExtensionHost)> for HostedExtension {
    fn from((metadata, host): (ExtensionMetadata, ExtensionHost)) -> Self {
        HostedExtension(metadata, Arc::new(Mutex::new(host)))
    }
}

#[async_trait]
impl ExtensionApi for HostedExtension {
    async fn on_add_to_queue(&self, tracks: Vec<Track>) -> Result<Vec<Track>, failure::Error> {
        let (tx, mut rx) = mpsc::channel(1);
        self.send(ExtensionCommand::AddToQueue(tracks, tx)).await;
        rx.recv()
            .await
            .ok_or_else(|| format_err!("Channel closed"))?
    }
}

#[derive(Default)]
pub struct ExtensionManagerBuilder {
    extensions: Vec<(ExtensionMetadata, ExtensionHost)>,
}

impl ExtensionManagerBuilder {
    pub async fn load_dir(
        &mut self,
        dir: &Path,
        config: &HashMap<String, HashMap<String, ExtensionConfigValue>>,
    ) -> Result<(), failure::Error> {
        let extensions: Vec<_> = dir
            .read_dir()?
            .filter_map(|file| file.ok())
            .filter(|file| {
                file.file_type()
                    .map(|file_type| file_type.is_file())
                    .unwrap_or(false)
            })
            .filter(|file| {
                trace!("checking whether {:?} is an extension", file.file_name());
                let file_name = file.file_name().into_string().unwrap();
                file_name.ends_with(".so")
                    || file_name.ends_with(".dylib")
                    || file_name.ends_with(".dll")
            })
            .collect();
        for entry in extensions {
            self.load(&entry.path(), config).await?;
        }

        Ok(())
    }

    pub async fn load(
        &mut self,
        path: &Path,
        config: &HashMap<String, HashMap<String, ExtensionConfigValue>>,
    ) -> Result<(), failure::Error> {
        let plugin = construct_plugin(&path, config)?;
        let mut host = ExtensionHost::new(plugin);
        let (tx, mut rx) = mpsc::channel(1);
        host.send(ExtensionCommand::GetMetadata(tx)).await;

        let metadata = rx
            .recv()
            .await
            .ok_or_else(|| format_err!("Channel closed"))?;
        info!("Loaded Extension: {} v{}", metadata.name, metadata.version);
        self.extensions.push((metadata, host));
        Ok(())
    }

    pub fn build(self) -> ExtensionManager {
        ExtensionManager {
            extensions: self
                .extensions
                .into_iter()
                .map(HostedExtension::from)
                .collect(),
        }
    }
}

#[derive(Clone)]
pub struct ExtensionManager {
    extensions: Vec<HostedExtension>,
}

impl std::fmt::Debug for ExtensionManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtensionManager")
            .field("extensions", &self.get_extensions())
            .finish()
    }
}

impl ExtensionManager {
    pub fn get_extensions(&self) -> Vec<ExtensionMetadata> {
        self.extensions
            .iter()
            .map(HostedExtension::get_metadata)
            .collect()
    }

    pub async fn setup(&mut self, runtime: ExtensionRuntime) -> Result<(), failure::Error> {
        for extension in self.extensions.iter() {
            extension
                .send(ExtensionCommand::Setup(runtime.clone()))
                .await;
        }
        Ok(())
    }
}

#[async_trait]
impl ExtensionApi for ExtensionManager {
    async fn on_add_to_queue(&self, tracks: Vec<Track>) -> Result<Vec<Track>, failure::Error> {
        let mut tracks = tracks;

        for extension in self.extensions.iter() {
            tracks = extension.on_add_to_queue(tracks).await?;
        }

        Ok(tracks)
    }
}
