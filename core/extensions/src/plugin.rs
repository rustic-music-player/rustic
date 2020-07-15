use async_trait::async_trait;
use tokio::sync::mpsc;

use rustic_core::{Track, Album, Artist, Playlist};

use crate::api::*;
use crate::host::ExtensionPlugin;
use crate::runtime::ExtensionRuntime;

#[async_trait]
impl<T: ExtensionLibrary + 'static> ExtensionPlugin for T {
    async fn handle_message(&mut self, message: ExtensionCommand) -> std::option::Option<u8> {
        match message {
            ExtensionCommand::Setup(runtime) => {
                self.setup(&runtime);
            }
            ExtensionCommand::GetMetadata(mut tx) => {
                let meta = T::metadata();
                tx.send(meta).await;
            }
            ExtensionCommand::Enable(mut response) => {
                let result = self.on_enable().await;
                response.send(result).await;
            }
            ExtensionCommand::Disable(mut response) => {
                let result = self.on_disable().await;
                response.send(result).await;
            }
            ExtensionCommand::AddToQueue(tracks, mut response) => {
                let result = self.on_add_to_queue(tracks).await;
                response.send(result).await;
            }
            ExtensionCommand::ResolveTrack(track, mut response) => {
                let result = self.resolve_track(track).await;
                response.send(result).await;
            }
            ExtensionCommand::ResolveAlbum(album, mut response) => {
                let result = self.resolve_album(album).await;
                response.send(result).await;
            }
            ExtensionCommand::ResolveArtist(artist, mut response) => {
                let result = self.resolve_artist(artist).await;
                response.send(result).await;
            }
            ExtensionCommand::ResolvePlaylist(playlist, mut response) => {
                let result = self.resolve_playlist(playlist).await;
                response.send(result).await;
            }
        }
        None
    }
}

#[derive(Debug)]
pub enum ExtensionCommand {
    Setup(ExtensionRuntime),
    GetMetadata(mpsc::Sender<ExtensionMetadata>),
    Enable(mpsc::Sender<Result<(), failure::Error>>),
    Disable(mpsc::Sender<Result<(), failure::Error>>),
    AddToQueue(Vec<Track>, mpsc::Sender<Result<Vec<Track>, failure::Error>>),
    ResolveTrack(Track, mpsc::Sender<Result<Track, failure::Error>>),
    ResolveAlbum(Album, mpsc::Sender<Result<Album, failure::Error>>),
    ResolveArtist(Artist, mpsc::Sender<Result<Artist, failure::Error>>),
    ResolvePlaylist(Playlist, mpsc::Sender<Result<Playlist, failure::Error>>),
}
