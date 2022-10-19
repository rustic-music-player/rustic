use serde::Serialize;
use crate::commands::MpdCommand;
use failure::Error;
use rustic_core::Rustic;
use std::sync::Arc;
use futures::future::BoxFuture;
use rustic_api::ApiClient;
use crate::FutureExt;

// TODO: parse MpdCommands enum

pub struct CommandsCommand {}

#[derive(Serialize, Debug)]
pub struct Command {
    command: String,
}

impl Command {
    fn new(label: &'static str) -> Command {
        Command {
            command: label.to_owned(),
        }
    }
}

impl CommandsCommand {
    pub fn new() -> CommandsCommand {
        CommandsCommand {}
    }
}

impl MpdCommand<Vec<Command>> for CommandsCommand {
    fn handle(&self, _app: Arc<Rustic>, _client: ApiClient) -> BoxFuture<Result<Vec<Command>, Error>> {
        async move {
            Ok(vec![
                Command::new("status"),
                Command::new("currentsong"),
                Command::new("commandlist"),
                Command::new("plchanges"),
                Command::new("outputs"),
                Command::new("decoders"),
                Command::new("idle"),
                Command::new("noidle"),
                Command::new("listplaylists"),
                Command::new("listplaylist"),
                Command::new("listplaylistinfo"),
                Command::new("playlistinfo"),
                Command::new("load"),
                Command::new("lsinfo"),
                Command::new("next"),
                Command::new("pause"),
                Command::new("play"),
                Command::new("previous"),
                Command::new("stop"),
                Command::new("list"),
                Command::new("find"),
                Command::new("add"),
                Command::new("addid"),
                Command::new("volume"),
                Command::new("setvol"),
                Command::new("commands"),
                Command::new("tagtypes"),
                Command::new("albumart"),
            ])
        }.boxed()
    }
}
