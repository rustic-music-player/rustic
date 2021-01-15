use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use async_trait::async_trait;
use failure::{bail, Error};
use flume::{bounded, Receiver};
use log::{info, trace};
use tokio::sync::Mutex;

use rustic_core::{Album, Artist, Playlist, StorageCollection, Track};

use crate::api::*;
use crate::host::{construct_plugin, ExtensionHost};
use crate::plugin::*;
use crate::runtime::ExtensionRuntime;
use std::time::Instant;

const EXTENSION_COLLECTION_KEY: &str = "extensions";

#[derive(Clone)]
pub struct HostedExtension(
    ExtensionMetadata,
    Arc<AtomicBool>,
    Arc<Mutex<ExtensionHost>>,
);

impl HostedExtension {
    pub fn get_metadata(&self) -> (ExtensionMetadata, bool) {
        (self.0.clone(), self.1.load(Ordering::Relaxed))
    }

    pub fn is_enabled(&self) -> bool {
        self.1.load(Ordering::Relaxed)
    }

    pub async fn send(&self, message: ExtensionCommand) {
        let mut host = self.2.lock().await;
        host.send(message).await;
    }

    async fn rpc<T>(
        &self,
        message: ExtensionCommand,
        rx: Receiver<Result<T, Error>>,
    ) -> Result<T, Error> {
        self.send(message).await;
        rx.recv_async().await?
    }
}

impl From<(ExtensionMetadata, ExtensionHost)> for HostedExtension {
    fn from((metadata, host): (ExtensionMetadata, ExtensionHost)) -> Self {
        HostedExtension(
            metadata,
            Arc::new(AtomicBool::new(false)),
            Arc::new(Mutex::new(host)),
        )
    }
}

#[async_trait]
impl ExtensionApi for HostedExtension {
    async fn on_enable(&self) -> Result<(), Error> {
        let (tx, rx) = bounded(1);
        self.rpc(ExtensionCommand::Enable(tx), rx).await?;
        self.1.store(true, Ordering::Relaxed);
        info!("Enabled {} extension", &self.0.name);
        Ok(())
    }

    async fn on_disable(&self) -> Result<(), Error> {
        let (tx, rx) = bounded(1);
        self.rpc(ExtensionCommand::Disable(tx), rx).await?;
        self.1.store(false, Ordering::Relaxed);
        info!("Disabled {} extension", &self.0.name);
        Ok(())
    }

    async fn on_add_to_queue(&self, tracks: Vec<Track>) -> Result<Vec<Track>, Error> {
        let (tx, rx) = bounded(1);
        self.rpc(ExtensionCommand::AddToQueue(tracks, tx), rx).await
    }

    async fn resolve_track(&self, track: Track) -> Result<Track, Error> {
        let (tx, rx) = bounded(1);
        self.rpc(ExtensionCommand::ResolveTrack(track, tx), rx)
            .await
    }

    async fn resolve_album(&self, album: Album) -> Result<Album, Error> {
        let (tx, rx) = bounded(1);
        self.rpc(ExtensionCommand::ResolveAlbum(album, tx), rx)
            .await
    }

    async fn resolve_artist(&self, artist: Artist) -> Result<Artist, Error> {
        let (tx, rx) = bounded(1);
        self.rpc(ExtensionCommand::ResolveArtist(artist, tx), rx)
            .await
    }

    async fn resolve_playlist(&self, playlist: Playlist) -> Result<Playlist, Error> {
        let (tx, rx) = bounded(1);
        self.rpc(ExtensionCommand::ResolvePlaylist(playlist, tx), rx)
            .await
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
    ) -> Result<(), Error> {
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
                file_name.ends_with("extension.so")
                    || file_name.ends_with("extension.dylib")
                    || file_name.ends_with("extension.dll")
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
    ) -> Result<(), Error> {
        let plugin = construct_plugin(&path, config)?;
        let mut host = ExtensionHost::new(plugin);
        let (tx, rx) = bounded(1);
        host.send(ExtensionCommand::GetMetadata(tx)).await;

        let metadata = rx.recv_async().await?;
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
            runtime: None,
        }
    }
}

#[derive(Clone)]
pub struct ExtensionManager {
    extensions: Vec<HostedExtension>,
    runtime: Option<ExtensionRuntime>,
}

impl std::fmt::Debug for ExtensionManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtensionManager")
            .field("extensions", &self.get_extensions())
            .finish()
    }
}

impl ExtensionManager {
    pub fn get_extensions(&self) -> Vec<(ExtensionMetadata, bool)> {
        self.extensions
            .iter()
            .map(HostedExtension::get_metadata)
            .collect()
    }

    pub async fn setup(&mut self, runtime: ExtensionRuntime) -> Result<(), Error> {
        let collection = runtime
            .storage
            .open_collection(EXTENSION_COLLECTION_KEY)
            .await?;
        for extension in self.extensions.iter() {
            let (tx, rx) = bounded(1);
            extension
                .rpc(
                    ExtensionCommand::Setup(runtime.for_extension(extension.0.clone()), tx),
                    rx,
                )
                .await?;
            match collection.read(&extension.0.id).await? {
                Some(value) if value.bool().unwrap_or_default() => {
                    extension.on_enable().await?;
                }
                _ => {}
            }
        }
        self.runtime = Some(runtime);
        Ok(())
    }

    pub async fn enable_extension(&self, id: &str) -> Result<(), Error> {
        let collection = self.get_extensions_collection().await?;
        if let Some(extension) = self.extensions.iter().find(|e| e.0.id == id) {
            trace!("Enabling extension {}...", &extension.0.name);
            extension.on_enable().await?;
            collection.write(id, true.into()).await?;
        }
        Ok(())
    }

    pub async fn disable_extension(&self, id: &str) -> Result<(), Error> {
        let collection = self.get_extensions_collection().await?;
        if let Some(extension) = self.extensions.iter().find(|e| e.0.id == id) {
            trace!("Disabling extension {}...", &extension.0.name);
            extension.on_disable().await?;
            collection.write(id, false.into()).await?;
        }
        Ok(())
    }

    async fn get_extensions_collection(&self) -> Result<Box<dyn StorageCollection>, Error> {
        if self.runtime.is_none() {
            bail!("Tried to store extension state before setup");
        }
        let runtime = self.runtime.as_ref().unwrap();
        runtime.storage.open_collection("extensions").await
    }

    pub fn get_enabled_extensions(&self) -> impl Iterator<Item = &HostedExtension> {
        self.extensions
            .iter()
            .filter(|extension| extension.is_enabled())
    }
}

#[async_trait]
impl ExtensionApi for ExtensionManager {
    async fn on_add_to_queue(&self, tracks: Vec<Track>) -> Result<Vec<Track>, Error> {
        let mut tracks = tracks;

        for extension in self.get_enabled_extensions() {
            tracks = extension.on_add_to_queue(tracks).await?;
        }

        Ok(tracks)
    }

    async fn resolve_track(&self, mut track: Track) -> Result<Track, Error> {
        for extension in self.get_enabled_extensions() {
            let stopwatch = Instant::now();
            track = extension.resolve_track(track).await?;
            trace!(
                "Resolved track from extension {} in {}ms",
                extension.0.name,
                stopwatch.elapsed().as_millis()
            );
        }
        Ok(track)
    }

    async fn resolve_album(&self, mut album: Album) -> Result<Album, Error> {
        for extension in self.get_enabled_extensions() {
            let stopwatch = Instant::now();
            album = extension.resolve_album(album).await?;
            trace!(
                "Resolved album from extension {} in {}ms",
                extension.0.name,
                stopwatch.elapsed().as_millis()
            );
        }
        Ok(album)
    }

    async fn resolve_artist(&self, mut artist: Artist) -> Result<Artist, Error> {
        for extension in self.get_enabled_extensions() {
            let stopwatch = Instant::now();
            artist = extension.resolve_artist(artist).await?;
            trace!(
                "Resolved artist from extension {} in {}ms",
                extension.0.name,
                stopwatch.elapsed().as_millis()
            );
        }
        Ok(artist)
    }

    async fn resolve_playlist(&self, mut playlist: Playlist) -> Result<Playlist, Error> {
        for extension in self.get_enabled_extensions() {
            let stopwatch = Instant::now();
            playlist = extension.resolve_playlist(playlist).await?;
            trace!(
                "Resolved playlist from extension {} in {}ms",
                extension.0.name,
                stopwatch.elapsed().as_millis()
            );
        }
        Ok(playlist)
    }
}
