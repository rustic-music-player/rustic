use std::any::Any;
use std::net::IpAddr;
use std::sync::{atomic, Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use failure::Error;
use pinboard::NonEmptyPinboard;
use rust_cast::{
    channels::{media::*, receiver::CastDeviceApp},
    CastDevice,
};

use rustic_core::Track;
use rustic_core::player::{PlayerEvent, PlayerState, PlayerBuilder, queue::MemoryQueueBuilder};

use crate::discovery::DiscoverMessage;

mod discovery;

enum InternalCommand {
    Play(Track),
    Volume(f32),
}

pub struct GoogleCastBackend {
    player_events: Sender<PlayerEvent>,
    core: Arc<rustic_core::Rustic>,
    handle: thread::JoinHandle<Result<(), failure::Error>>,
    internal_sender: crossbeam_channel::Sender<InternalCommand>,
}

impl GoogleCastBackend {
    pub fn start_discovery(core: Arc<rustic_core::Rustic>) {
        let (tx, rx) = crossbeam_channel::unbounded();
        let running = core.running();
        thread::spawn(move || {
            discovery::discover(tx, running);
        });
        let core = Arc::clone(&core);
        thread::spawn(move || {
            for msg in rx {
                match msg {
                    DiscoverMessage::AddBackend(target) => {
                        let player = PlayerBuilder::new(Arc::clone(&core))
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

    pub fn new(
        core: Arc<rustic_core::Rustic>,
        player_events: Sender<PlayerEvent>,
        ip: IpAddr,
    ) -> Result<Box<dyn rustic_core::PlayerBackend>, Error> {
        let (internal_sender, internal_receiver) = crossbeam_channel::unbounded();
        let handle = {
            let core = Arc::clone(&core);
            thread::spawn(move || {
                let device = CastDevice::connect_without_host_verification(ip.to_string(), 8009)?;
                let app = device
                    .receiver
                    .launch_app(&CastDeviceApp::DefaultMediaReceiver)?;

                loop {
                    match internal_receiver.recv() {
                        Ok(InternalCommand::Play(track)) => {
                            device.connection.connect(app.transport_id.as_str())?;
                            let media = Media {
                                content_id: core.stream_url(&track)?,
                                stream_type: StreamType::None,
                                content_type: "audio/mp3".to_string(),
                                metadata: Some(Metadata::MusicTrack(MusicTrackMediaMetadata {
                                    album_name: track.album.map(|album| album.title),
                                    title: Some(track.title),
                                    album_artist: None,
                                    artist: track.artist.map(|artist| artist.name),
                                    composer: None,
                                    track_number: None,
                                    disc_number: None,
                                    images: vec![],
                                    release_date: None,
                                })),
                                duration: None,
                            };
                            device.media.load(
                                app.transport_id.as_str(),
                                app.session_id.as_str(),
                                &media,
                            )?;
                        }
                        Ok(InternalCommand::Volume(volume)) => {
                            device.receiver.set_volume(volume)?;
                        }
                        _ => (),
                    }
                }
            })
        };

        Ok(Box::new(GoogleCastBackend {
            player_events,
            core,
            handle,
            internal_sender,
        }))
    }
}

impl rustic_core::PlayerBackend for GoogleCastBackend {
    fn set_track(&self, track: &Track) -> Result<(), Error> {
        unimplemented!()
    }

    fn set_state(&self, state: PlayerState) -> Result<(), Error> {
        unimplemented!()
    }

    fn state(&self) -> PlayerState {
        unimplemented!()
    }

    fn set_volume(&self, volume: f32) -> Result<(), Error> {
        unimplemented!()
    }

    fn volume(&self) -> f32 {
        unimplemented!()
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
        self.with_player(|core, _, _, events_tx| {
            GoogleCastBackend::new(core, events_tx, ip)
        })
    }
}