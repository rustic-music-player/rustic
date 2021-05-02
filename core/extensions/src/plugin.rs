use async_trait::async_trait;
use rustic_queue::Sender;

use rustic_core::{Album, Artist, Playlist, Track};

use crate::api::*;
use crate::host::ExtensionPlugin;
use crate::runtime::ExtensionRuntime;
use crate::controls::ExtensionControls;

#[async_trait]
impl<T: ExtensionLibrary + 'static> ExtensionPlugin for T {
    async fn handle_message(&mut self, message: ExtensionCommand) -> std::option::Option<u8> {
        match message {
            ExtensionCommand::Setup(runtime, tx) => {
                let result = self.setup(&runtime);
                tx.send_async(result).await;
            }
            ExtensionCommand::GetMetadata(tx) => {
                let meta = T::metadata();
                tx.send_async(meta).await;
            }
            ExtensionCommand::Enable(response) => {
                let result = self.on_enable().await;
                response.send_async(result).await;
            }
            ExtensionCommand::Disable(response) => {
                let result = self.on_disable().await;
                response.send_async(result).await;
            }
            ExtensionCommand::AddToQueue(tracks, response) => {
                let result = self.on_add_to_queue(tracks).await;
                response.send_async(result).await;
            }
            ExtensionCommand::ResolveTrack(track, response) => {
                let result = self.resolve_track(track).await;
                response.send_async(result).await;
            }
            ExtensionCommand::ResolveAlbum(album, response) => {
                let result = self.resolve_album(album).await;
                response.send_async(result).await;
            }
            ExtensionCommand::ResolveArtist(artist, response) => {
                let result = self.resolve_artist(artist).await;
                response.send_async(result).await;
            }
            ExtensionCommand::ResolvePlaylist(playlist, response) => {
                let result = self.resolve_playlist(playlist).await;
                response.send_async(result).await;
            }
            ExtensionCommand::GetControls(response) => {
                let result = self.get_controls().await;
                response.send_async(result).await;
            }
        }
        None
    }
}

#[derive(Debug)]
pub enum ExtensionCommand {
    Setup(ExtensionRuntime, Sender<Result<(), failure::Error>>),
    GetMetadata(Sender<ExtensionMetadata>),
    GetControls(Sender<Result<ExtensionControls, failure::Error>>),
    Enable(Sender<Result<(), failure::Error>>),
    Disable(Sender<Result<(), failure::Error>>),
    AddToQueue(Vec<Track>, Sender<Result<Vec<Track>, failure::Error>>),
    ResolveTrack(Track, Sender<Result<Track, failure::Error>>),
    ResolveAlbum(Album, Sender<Result<Album, failure::Error>>),
    ResolveArtist(Artist, Sender<Result<Artist, failure::Error>>),
    ResolvePlaylist(Playlist, Sender<Result<Playlist, failure::Error>>),
}
