extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rustic_core;
extern crate serde_mpd;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;

mod commands;
mod song;

use rustic_core::Rustic;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use commands::MpdCommand;

#[derive(Deserialize, Clone, Debug)]
pub struct MpdConfig {
    pub ip: String,
    pub port: i32,
}

fn open(config: &MpdConfig, app: Arc<Rustic>) -> Result<(), failure::Error> {
    let listener = TcpListener::bind(format!("{}:{}", config.ip, config.port))?;
    info!("Listening on Port {}", config.port);

    for stream in listener.incoming() {
        debug!("Connection opened");

        let app = app.clone();

        thread::spawn(move || handle_client(stream.unwrap(), &app));
    }

    Ok(())
}

pub fn start(config: Option<MpdConfig>, app: Arc<Rustic>) -> thread::JoinHandle<()> {
    let config = config.unwrap_or(MpdConfig {
        ip: "0.0.0.0".to_owned(),
        port: 6600,
    });
    thread::spawn(move || {
        open(&config, app).unwrap();
    })
}

fn handle_client(stream: TcpStream, app: &Arc<Rustic>) {
    let mut reader = BufReader::new(stream);
    let header = "OK MPD 0.16.0\n";
    let result = reader.get_ref().write(header.as_bytes());
    match result {
        Ok(_) => trace!("< {:?}", &header),
        Err(e) => error!("{:?}", &e),
    }

    loop {
        let line = reader.by_ref().lines().next();
        match line {
            Some(line) => {
                let res: Result<Option<()>, failure::Error> =
                    line.map_err(failure::Error::from).and_then(|line| {
                        trace!("> {:?}", &line);
                        let cmd: Result<MpdCommands, failure::Error> = if line
                            == "command_list_ok_begin"
                        {
                            let mut current =
                                reader.by_ref().lines().next().expect("line").expect("line");
                            trace!("> {:?}", &current);
                            let mut cmds: Vec<MpdCommands> = vec![];
                            while current.as_str() != "command_list_end" {
                                if let Ok(cmd) = parse_single(&current) {
                                    cmds.push(cmd)
                                }
                                current =
                                    reader.by_ref().lines().next().expect("line").expect("line");
                                trace!("> {:?}", &current);
                            }
                            Ok(MpdCommands::CommandList(cmds))
                        } else {
                            parse_single(&line)
                        };
                        cmd.and_then(|cmd| match cmd {
                            MpdCommands::Idle => Ok(Some(())),
                            MpdCommands::Close => Ok(None),
                            cmd => {
                                let mut result = handle_mpd_command(cmd, &app)?;
                                result += "OK\n";
                                trace!("< {:?}", &result);
                                reader.get_ref().write_all(result.as_bytes())?;
                                Ok(Some(()))
                            }
                        })
                    });

                match res {
                    Ok(None) => break,
                    Err(err) => {
                        error!("{:?}", &err);
                        break;
                    }
                    Ok(Some(())) => {}
                }
            }
            None => break,
        }
    }

    debug!("Connection closed");
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum MpdCommands {
    Status,
    CurrentSong,
    CommandList(Vec<MpdCommands>),
    #[serde(rename = "plchanges")]
    PlaylistChanges(String),
    Outputs,
    Decoders,
    Idle,
    NoIdle,
    ListPlaylists,
    ListPlaylist(String),
    ListPlaylistInfo(String),
    LoadPlaylist(String),
    #[serde(rename = "lsinfo")]
    ListInfo(String),
    Next,
    Pause,
    // Pause(bool), Spec says bool argument exists, ncmpcpp doesn't send it
    Play(u64),
    Previous,
    Stop,
    List(String),
    Add(String),
    AddId(String),
    #[serde(rename = "volume")]
    ChangeVolumeBy(i32),
    #[serde(rename = "setvol")]
    ChangeVolume(u32),
    Commands,
    TagTypes,
    Close,
}

fn parse_single(line: &str) -> Result<MpdCommands, failure::Error> {
    Ok(serde_mpd::from_str(line)?)
}

fn handle_mpd_command(cmd: MpdCommands, app: &Arc<Rustic>) -> Result<String, failure::Error> {
    debug!("Command: {:?}", &cmd);
    match cmd {
        MpdCommands::Status => commands::StatusCommand::new()
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        MpdCommands::CurrentSong => commands::CurrentSongCommand::new()
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        // MpdCommands::Pause(true) => commands::PauseCommand::new().handle(app)
        MpdCommands::Pause => commands::PauseCommand::new()
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        MpdCommands::Play(_) => commands::PlayCommand::new()
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        MpdCommands::Stop => commands::StopCommand::new()
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        MpdCommands::ListInfo(path) => commands::ListInfoCommand::new(path)
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        MpdCommands::ListPlaylists => commands::ListPlaylistsCommand::new()
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        MpdCommands::ListPlaylist(name) => commands::ListPlaylistCommand::new(name)
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        MpdCommands::ListPlaylistInfo(name) => commands::ListPlaylistInfoCommand::new(name)
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        MpdCommands::LoadPlaylist(name) => commands::LoadPlaylistCommand::new(name)
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        MpdCommands::Previous => commands::PreviousCommand::new()
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        MpdCommands::Next => commands::NextCommand::new()
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        MpdCommands::Outputs => commands::OutputsCommand::new()
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        MpdCommands::List(ref t) if t == "Artist" => commands::ListArtistCommand::new()
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        MpdCommands::ChangeVolumeBy(volume) => commands::ChangeVolumeCommand::new(volume)
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        MpdCommands::ChangeVolume(volume) => commands::SetVolumeCommand::new(volume)
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        MpdCommands::Commands => commands::CommandsCommand::new()
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        MpdCommands::TagTypes => commands::TagTypesCommand::new()
            .handle(app)
            .and_then(|res| serde_mpd::to_string(&res).map_err(failure::Error::from)),
        MpdCommands::CommandList(commands) => {
            let mut result = String::new();
            for command in commands {
                result += handle_mpd_command(command, app)?.as_str();
                result += "list_OK\n";
            }
            Ok(result)
        }
        _ => Ok(String::new()),
    }
}
