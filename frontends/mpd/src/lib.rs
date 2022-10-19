#[macro_use]
extern crate failure;

mod commands;
mod song;
pub(crate) mod client_ext;

use serde::Deserialize;

use rustic_core::Rustic;
use rustic_api::ApiClient;

use std::sync::Arc;
use futures::FutureExt;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{BufReader, AsyncWriteExt, AsyncBufReadExt};

use crate::commands::MpdCommand;

#[derive(Deserialize, Clone, Debug)]
pub struct MpdConfig {
    pub ip: String,
    pub port: i32,
}

async fn open(config: MpdConfig, app: Arc<Rustic>, client: ApiClient) -> Result<(), failure::Error> {
    let listener = TcpListener::bind(format!("{}:{}", config.ip, config.port)).await?;
    log::info!("Listening on Port {}", config.port);

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                log::debug!("Connection opened");
                tokio::task::spawn(handle_client(socket, app.clone(), client.clone()));
            },
            Err(err) => log::error!("{:?}", err),
        }
    }
}

pub fn start(config: Option<MpdConfig>, app: Arc<Rustic>, client: ApiClient) -> tokio::task::JoinHandle<Result<(), failure::Error>> {
    let config = config.unwrap_or(MpdConfig {
        ip: "0.0.0.0".to_owned(),
        port: 6600,
    });
    tokio::task::spawn(open(config, app, client))
}

async fn handle_client(stream: TcpStream, app: Arc<Rustic>, client: ApiClient) {
    let mut reader = BufReader::new(stream);
    let header = "OK MPD 0.16.0\n";
    let result = reader.get_mut().write(header.as_bytes()).await;
    match result {
        Ok(_) => log::trace!("< {:?}", &header),
        Err(e) => log::error!("{:?}", &e),
    }

    loop {
        let res: Result<Option<()>, failure::Error> = handle_line(&mut reader, &app, &client).await;

        match res {
            Ok(None) => break,
            Err(err) => {
                log::error!("{:?}", &err);
                break;
            }
            Ok(Some(())) => {}
        }
    }

    log::debug!("Connection closed");
}

async fn handle_line(reader: &mut BufReader<TcpStream>, app: &Arc<Rustic>, client: &ApiClient) -> Result<Option<()>, failure::Error> {
    let mut line = String::new();
    reader.read_line(&mut line).await?;
    let line = line.trim();
    if line.is_empty() {
        return Ok(None);
    }
    log::trace!("> {:?}", line);
    let cmd: mpd_protocol::Request = if line
        == "command_list_ok_begin" || line == "command_list_begin"
    {
        let mut current = String::new();
        reader.read_line(&mut current).await?;
        log::trace!("> {:?}", &current);
        let mut cmds: Vec<mpd_protocol::Command> = vec![];
        while current.trim() != "command_list_end" {
            if let Ok((_, cmd)) = mpd_protocol::parse_command(&current.trim()) {
                cmds.push(cmd)
            }
            current.clear();
            reader.read_line(&mut current).await?;
            log::trace!("> {:?}", &current);
        }
        mpd_protocol::Request::CommandList(cmds, line == "command_list_ok_begin")
    } else {
        parse_single(&line)?
    };

    match cmd {
        mpd_protocol::Request::Command(mpd_protocol::Command::Idle(_)) => Ok(Some(())),
        mpd_protocol::Request::Command(mpd_protocol::Command::Close) => Ok(None),
        mpd_protocol::Request::Command(mpd_protocol::Command::AlbumArt(uri, offset)) => {
            let album_art = commands::AlbumArtCommand::new(uri, offset)
                .handle(app.clone(), client.clone())
                .await?;

            let byte_count = album_art.bytes.len();
            let bytes = album_art.bytes.as_slice();
            let writer = reader.get_mut();
            let result = format!("size: {}\ntype: {}\nbinary: {}\n", album_art.total_size, album_art.mime_type, byte_count);
            writer.write_all(result.as_bytes()).await?;
            writer.write_all(bytes).await?;
            writer.write_all("\nOK\n".as_bytes()).await?;

            Ok(Some(()))
        },
        mpd_protocol::Request::Command(cmd) => {
            let mut result = handle_mpd_command(cmd, app.clone(), client.clone()).await?;
            result += "OK\n";
            log::trace!("< {:?}", &result);
            reader.get_mut().write_all(result.as_bytes()).await?;

            Ok(Some(()))
        }
        mpd_protocol::Request::CommandList(commands, list_ok) => {
            let mut result = String::new();
            for command in commands {
                result += handle_mpd_command(command, app.clone(), client.clone()).await?.as_str();
                if list_ok {
                    result += "list_OK\n";
                }
            }
            result += "OK\n";
            log::trace!("< {:?}", &result);
            reader.get_mut().write_all(result.as_bytes()).await?;

            Ok(Some(()))
        }
    }
}

fn parse_single(line: &str) -> Result<mpd_protocol::Request, failure::Error> {
    match mpd_protocol::parse_command(line) {
        Ok((_, command)) => {
            Ok(mpd_protocol::Request::Command(command))
        }
        Err(err) => {
            failure::bail!("MPD Parse error: {}", err)
        }
    }
}

async fn handle_mpd_command(cmd: mpd_protocol::Command, app: Arc<Rustic>, client: ApiClient) -> Result<String, failure::Error> {
    use mpd_protocol::Command::*;
    log::debug!("Command: {:?}", &cmd);
    match cmd {
        Status => commands::StatusCommand::new()
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        CurrentSong => commands::CurrentSongCommand::new()
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        Pause(Some(true)) => commands::PauseCommand::new()
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        Pause(Some(false)) | Play(_) => commands::PlayCommand::new()
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        Pause(None) => commands::TogglePauseCommand::new()
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        Stop => commands::StopCommand::new()
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        ListInfo(path) => commands::ListInfoCommand::new(path)
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        ListPlaylists => commands::ListPlaylistsCommand::new()
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        ListPlaylist(name) => commands::ListPlaylistCommand::new(name)
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        ListPlaylistInfo(name) => commands::ListPlaylistInfoCommand::new(name)
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        PlaylistInfo => commands::PlaylistInfoCommand::new()
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        LoadPlaylist(name) => commands::LoadPlaylistCommand::new(name)
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        Previous => commands::PreviousCommand::new()
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        Next => commands::NextCommand::new()
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        Outputs => commands::OutputsCommand::new()
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        List(ref t) if t == "Artist" => commands::ListArtistCommand::new()
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        ChangeVolumeBy(volume) => commands::ChangeVolumeCommand::new(volume)
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        SetVolume(volume) => commands::SetVolumeCommand::new(volume)
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        Commands => commands::CommandsCommand::new()
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        TagTypes => commands::TagTypesCommand::new()
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        AddId(cursor) => commands::AddTrackCommand::new(cursor)
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        Clear => commands::ClearQueueCommand::new()
            .handle(app, client)
            .await
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        _ => Ok(String::new()),
    }
}
