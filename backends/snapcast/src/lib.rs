mod symphonia_decoder;
mod background_job;
mod audio_transport;

use std::any::Any;
use std::sync::Arc;
use std::time::Duration;

use failure::Error;
use smol::channel::{Sender, TryRecvError};

use rustic_core::{PlayerBackend, PlayerState, Track};
use rustic_core::{player::{PlayerBuilder, PlayerBus}, Rustic};
use rustic_core::player::{QueueCommand};
use snapcast_api::SnapcastClient;
use pinboard::NonEmptyPinboard;
pub use crate::audio_transport::SnapcastAudioTransport;
use crate::background_job::BackgroundJob;

struct SnapcastBackend {
    stream_id: String,
    client: SnapcastClient,
    cmd_tx: Sender<BackgroundCommand>,
    current_state: NonEmptyPinboard<PlayerState>,
    transport: SnapcastAudioTransport,
}

#[derive(Debug, Clone)]
enum BackgroundCommand {
    SetState(PlayerState),
    Play(Track, String),
}

impl std::fmt::Debug for SnapcastBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SnapcastBackend")
            .field("client", &self.client)
            .finish()
    }
}

impl SnapcastBackend {
    fn new(core: Arc<Rustic>, bus: PlayerBus, api_url: String, transport: SnapcastAudioTransport, name: String) -> Result<Self, Error> {
        let client = SnapcastClient::http(api_url);
        let stream_id = smol::block_on(transport.add_stream(&name, &client))?;

        let (cmd_tx, mut cmd_rx) = smol::channel::unbounded::<BackgroundCommand>();

        let mut background = BackgroundJob::new(core, transport.clone());

        smol::spawn(async move {
            loop {
                if background.has_child_exited() {
                    bus.send_queue_msg(QueueCommand::Next).unwrap();
                }

                match cmd_rx.try_recv() {
                    Ok(cmd) => background.handle_cmd(cmd).unwrap(),
                    Err(TryRecvError::Empty) => {}
                    Err(e) => {
                        log::error!("{:?}", e);
                        return;
                    }
                }
            }
        }).detach();

        Ok(SnapcastBackend {
            stream_id,
            client,
            cmd_tx,
            transport,
            current_state: NonEmptyPinboard::new(PlayerState::Stop),
        })
    }
}


impl PlayerBackend for SnapcastBackend {
    fn set_track(&self, track: &Track, stream_url: String) -> Result<(), Error> {
        smol::block_on(self.cmd_tx.send(BackgroundCommand::Play(track.clone(), stream_url)))?;

        Ok(())
    }

    fn set_state(&self, state: PlayerState) -> Result<(), Error> {
        self.cmd_tx.try_send(BackgroundCommand::SetState(state))?;
        self.current_state.set(state);
        Ok(())
    }

    fn state(&self) -> PlayerState {
        self.current_state.read()
    }

    fn set_volume(&self, volume: f32) -> Result<(), Error> {
        unimplemented!()
    }

    fn volume(&self) -> f32 {
        log::warn!("unimplemented");
        1.0
    }

    fn set_blend_time(&self, duration: Duration) -> Result<(), Error> {
        unimplemented!()
    }

    fn blend_time(&self) -> Duration {
        unimplemented!()
    }

    fn seek(&self, duration: Duration) -> Result<(), Error> {
        unimplemented!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn close(&self) -> Result<(), Error> {
        self.cmd_tx.try_send(BackgroundCommand::SetState(PlayerState::Stop))?;
        smol::block_on(async {
            self.client.remove_stream(self.stream_id.clone()).await?;
            self.transport.close().await?;

            Ok::<_, Error>(())
        })?;
        Ok(())
    }
}

pub trait SnapcastPlayerBuilder {
    fn with_snapcast(&mut self, api_url: String, transport: SnapcastAudioTransport) -> Result<&mut Self, Error>;
}

impl SnapcastPlayerBuilder for PlayerBuilder {
    fn with_snapcast(&mut self, api_url: String, transport: SnapcastAudioTransport) -> Result<&mut Self, Error> {
        let name = self.name.clone().expect("name should already be set"); // TODO: generate default name?
        self.with_player(move |rustic, bus| {
            let backend = SnapcastBackend::new(rustic, bus, api_url, transport, name)?;

            Ok(Box::new(backend))
        })
    }
}
