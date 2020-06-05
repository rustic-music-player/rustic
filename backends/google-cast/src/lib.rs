use std::any::Any;
use std::net::IpAddr;
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use failure::Error;
use log::error;
use pinboard::NonEmptyPinboard;
use rust_cast::CastDevice;

use rustic_core::player::{queue::MemoryQueueBuilder, PlayerBuilder, PlayerBus, PlayerState};
use rustic_core::Track;

use crate::cast_state::CastState;
use crate::discovery::DiscoverMessage;
use crate::internal_command::InternalCommand;
use crate::tasks::{CastCommandTask, CastStateSyncTask};

mod cast_state;
mod discovery;
mod internal_command;
mod tasks;

pub struct GoogleCastBackend {
    bus: PlayerBus,
    internal_sender: crossbeam_channel::Sender<InternalCommand>,
    cast_state: Arc<NonEmptyPinboard<CastState>>,
}

impl GoogleCastBackend {
    pub fn start_discovery(core: Arc<rustic_core::Rustic>) {
        let (tx, rx) = crossbeam_channel::unbounded();
        thread::spawn(move || {
            discovery::discover(tx);
        });
        let core = Arc::clone(&core);
        thread::spawn(move || {
            for msg in rx {
                match msg {
                    DiscoverMessage::AddBackend(target) => {
                        let player = PlayerBuilder::new(Arc::clone(&core))
                            .with_name(&target.name)
                            .with_memory_queue()
                            .with_google_cast(target.addr)
                            .unwrap()
                            .build();
                        core.add_player(target.name, player)
                    }
                }
            }
        });
    }

    pub fn new(bus: PlayerBus, ip: IpAddr) -> Result<Box<dyn rustic_core::PlayerBackend>, Error> {
        let (internal_sender, internal_receiver) = crossbeam_channel::unbounded();
        let cast_state = Arc::new(NonEmptyPinboard::new(CastState::default()));
        {
            thread::spawn::<_, Result<(), failure::Error>>(move || {
                let device = CastDevice::connect_without_host_verification(ip.to_string(), 8009)?;
                let mut task = CastCommandTask::new(internal_receiver);

                loop {
                    if let Err(e) = task.next(&device) {
                        error!("CastStateTask failed {:?}", e);
                    }
                }
            });
        }
        {
            let state = Arc::clone(&cast_state);
            thread::spawn::<_, Result<(), failure::Error>>(move || {
                let device = CastDevice::connect_without_host_verification(ip.to_string(), 8009)?;
                let task = CastStateSyncTask::new(state);

                loop {
                    if let Err(e) = task.next(&device) {
                        error!("CastStateTask failed {:?}", e);
                    }

                    sleep(Duration::from_secs(1))
                }
            });
        }

        Ok(Box::new(GoogleCastBackend {
            bus,
            internal_sender,
            cast_state,
        }))
    }
}

impl rustic_core::PlayerBackend for GoogleCastBackend {
    fn set_track(&self, track: &Track, stream_url: String) -> Result<(), Error> {
        self.internal_sender
            .send(InternalCommand::Play(track.clone(), stream_url))?;
        Ok(())
    }

    fn set_state(&self, state: PlayerState) -> Result<(), Error> {
        self.internal_sender
            .send(InternalCommand::SetState(state))?;
        Ok(())
    }

    fn state(&self) -> PlayerState {
        let state = self.cast_state.read();
        state.state
    }

    fn set_volume(&self, volume: f32) -> Result<(), Error> {
        self.internal_sender.send(InternalCommand::Volume(volume))?;
        Ok(())
    }

    fn volume(&self) -> f32 {
        let state = self.cast_state.read();
        state.volume
    }

    fn set_blend_time(&self, duration: Duration) -> Result<(), Error> {
        unimplemented!()
    }

    fn blend_time(&self) -> Duration {
        Duration::new(0, 0)
    }

    fn seek(&self, duration: Duration) -> Result<(), Error> {
        unimplemented!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl std::fmt::Debug for GoogleCastBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GoogleCastBackend {{}}")
    }
}

pub trait GoogleCastBuilder {
    fn with_google_cast(&mut self, ip: IpAddr) -> Result<&mut Self, Error>;
}

impl GoogleCastBuilder for PlayerBuilder {
    fn with_google_cast(&mut self, ip: IpAddr) -> Result<&mut Self, Error> {
        self.with_player(|_, bus| GoogleCastBackend::new(bus, ip))
    }
}
