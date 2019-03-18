use crossbeam_channel::{Receiver, Sender};
use failure::Error;
use std::any::Any;
use std::time::Duration;
use std::sync::{Arc, atomic, Mutex};
use std::thread;
use rust_cast::{CastDevice, channels::{
    receiver::CastDeviceApp,
    media::*
}};
use pinboard::NonEmptyPinboard;

use rustic_core::{Track, PlayerState, PlayerEvent};

enum InternalCommand {
    Play(Track),
    Volume(f32)
}

pub struct GoogleCastBackend {
    tx: Sender<PlayerEvent>,
    rx: Receiver<PlayerEvent>,
    core: Arc<rustic_core::Rustic>,
    queue: NonEmptyPinboard<Vec<Track>>,
    current_index: atomic::AtomicUsize,
    current_track: NonEmptyPinboard<Option<Track>>,
    handle: thread::JoinHandle<Result<(), failure::Error>>,
    internal_sender: crossbeam_channel::Sender<InternalCommand>
}

impl GoogleCastBackend {
    pub fn new(core: Arc<rustic_core::Rustic>, ip: String) -> Result<Arc<Box<rustic_core::PlayerBackend>>, Error> {
        let (internal_sender, internal_receiver) = crossbeam_channel::unbounded();
        let (tx, rx) = crossbeam_channel::unbounded();
        let handle = {
            let core = Arc::clone(&core);
            thread::spawn(move|| {
                let device = CastDevice::connect_without_host_verification(ip, 8009)?;
                let app = device.receiver.launch_app(&CastDeviceApp::DefaultMediaReceiver)?;

                loop {
                    match internal_receiver.recv() {
                        Some(InternalCommand::Play(track)) => {
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
                                    release_date: None
                                })),
                                duration: None
                            };
                            device.media.load(app.transport_id.as_str(), app.session_id.as_str(), &media)?;
                        },
                        Some(InternalCommand::Volume(volume)) => {
                            device.receiver.set_volume(volume)?;
                        },
                        _ => ()
                    }
                }
            })
        };

        Ok(Arc::new(Box::new(GoogleCastBackend {
            tx,
            rx,
            core,
            queue: NonEmptyPinboard::new(vec![]),
            current_index: atomic::AtomicUsize::new(0),
            current_track: NonEmptyPinboard::new(None),
            handle,
            internal_sender
        })))
    }
}

impl rustic_core::PlayerBackend for GoogleCastBackend {
    fn queue_single(&self, track: &Track) {
        let mut queue = self.queue.read();
        queue.push(track.clone());
        self.queue.set(queue);
    }

    fn queue_multiple(&self, tracks: &[Track]) {
        let mut queue = self.queue.read();
        queue.append(&mut tracks.to_vec());
        self.queue.set(queue);
    }

    fn queue_next(&self, track: &Track) {
        let mut queue = self.queue.read();
        let current_index = self.current_index.load(atomic::Ordering::Relaxed);
        queue.insert(current_index + 1, track.clone());
        self.queue.set(queue);
    }

    fn get_queue(&self) -> Vec<Track> {
        self.queue.read()
    }

    fn clear_queue(&self) {
        self.queue.set(vec![]);
    }

    fn current(&self) -> Option<Track> {
        self.current_track.read()
    }

    fn prev(&self) -> Result<Option<()>, Error> {
        unimplemented!()
    }

    fn next(&self) -> Result<Option<()>, Error> {
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

    fn observe(&self) -> Receiver<PlayerEvent> {
        self.rx.clone()
    }

    fn as_any(&self) -> &Any {
        self
    }
}