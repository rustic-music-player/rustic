use async_trait::async_trait;
use tokio::sync::mpsc;

use rustic_core::Track;

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
            ExtensionCommand::AddToQueue(tracks, mut response) => {
                let tracks = self.on_add_to_queue(tracks).await;
                response.send(tracks).await;
            }
            ExtensionCommand::Enable(mut response) => {
                let result = self.on_enable().await;
                response.send(result).await;
            }
            ExtensionCommand::Disable(mut response) => {
                let result = self.on_disable().await;
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
    AddToQueue(Vec<Track>, mpsc::Sender<Result<Vec<Track>, failure::Error>>),
    Enable(mpsc::Sender<Result<(), failure::Error>>),
    Disable(mpsc::Sender<Result<(), failure::Error>>),
}
