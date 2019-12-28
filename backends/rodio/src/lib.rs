extern crate crossbeam_channel as channel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate pinboard;
extern crate rodio;
extern crate rustic_core as core;
extern crate url;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use channel::{Receiver, Sender};
use failure::Error;
use failure::_core::any::Any;
use pinboard::NonEmptyPinboard;
use url::Url;

use core::{player::MemoryQueue, PlayerBackend, PlayerEvent, PlayerState, Track};

pub struct RodioBackend {
    core: Arc<core::Rustic>,
    queue: MemoryQueue,
    state: NonEmptyPinboard<PlayerState>,
    blend_time: Duration,
    current_sink: Arc<Mutex<Option<rodio::Sink>>>,
    device: rodio::Device,
    tx: Sender<PlayerEvent>,
    rx: Receiver<PlayerEvent>,
}

impl std::fmt::Debug for RodioBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "RodioBackend {{ queue: {:?}, state: {:?}, blend_time: {:?} }}",
            self.queue, self.state, self.blend_time
        )
    }
}

impl RodioBackend {
    pub fn new(core: Arc<core::Rustic>) -> Result<Arc<Box<dyn PlayerBackend>>, Error> {
        let device = rodio::default_output_device().unwrap();
        let (tx, rx) = channel::unbounded();
        let backend = RodioBackend {
            core,
            queue: MemoryQueue::new(),
            state: NonEmptyPinboard::new(PlayerState::Stop),
            blend_time: Duration::default(),
            current_sink: Arc::new(Mutex::new(None)),
            device,
            tx,
            rx,
        };

        Ok(Arc::new(Box::new(backend)))
    }

    fn decode_stream(stream_url: String) -> Result<rodio::Decoder<BufReader<File>>, Error> {
        trace!("Decoding stream {}", &stream_url);
        let url = Url::parse(&stream_url)?;
        match url.scheme() {
            "file" => {
                let mut path = stream_url;
                path.replace_range(..7, "");
                trace!("Decoding file {}", &path);
                let file = File::open(path)?;
                let decoder = rodio::Decoder::new(BufReader::new(file))?;
                Ok(decoder)
            }
            scheme => bail!("Invalid scheme: {}", scheme),
        }
    }

    fn write_state(&self, state: PlayerState) {
        self.state.set(state);
        self.tx.send(PlayerEvent::StateChanged(state));
    }

    fn set_track(&self, track: &Track) -> Result<(), Error> {
        debug!("Selecting {:?}", track);
        let source = RodioBackend::decode_stream(self.core.stream_url(track)?)?;
        let sink = rodio::Sink::new(&self.device);
        sink.append(source);
        let mut current_sink = self.current_sink.lock().unwrap();
        if let Some(prev_sink) = current_sink.deref_mut() {
            prev_sink.stop();
        }
        *current_sink = Some(sink);
        self.tx.send(PlayerEvent::TrackChanged(track.clone()));

        if self.state.read() == PlayerState::Play {
            self.play();
        }

        Ok(())
    }

    fn play(&self) {
        if let Some(sink) = self.current_sink.lock().unwrap().deref_mut() {
            sink.play();
        }
    }

    fn pause(&self) {
        if let Some(sink) = self.current_sink.lock().unwrap().deref_mut() {
            sink.pause();
        }
    }

    fn stop(&self) {
        if let Some(sink) = self.current_sink.lock().unwrap().deref_mut() {
            sink.stop();
        }
    }
}

impl PlayerBackend for RodioBackend {
    fn queue_single(&self, track: &Track) {
        self.queue.queue_single(track);
    }

    fn queue_multiple(&self, tracks: &[Track]) {
        self.queue.queue_multiple(tracks);
    }

    fn queue_next(&self, track: &Track) {
        self.queue.queue_next(track);
    }

    fn get_queue(&self) -> Vec<Track> {
        self.queue.get_queue()
    }

    fn clear_queue(&self) {
        self.queue.clear();
    }

    fn current(&self) -> Option<Track> {
        self.queue.get_current_track()
    }

    fn prev(&self) -> Result<Option<()>, Error> {
        if let Some(track) = self.queue.prev() {
            self.set_track(&track)?;
            Ok(Some(()))
        } else {
            self.stop();
            Ok(None)
        }
    }

    fn next(&self) -> Result<Option<()>, Error> {
        if let Some(track) = self.queue.next() {
            self.set_track(&track)?;
            Ok(Some(()))
        } else {
            self.stop();
            Ok(None)
        }
    }

    fn set_state(&self, state: PlayerState) -> Result<(), Error> {
        match state {
            PlayerState::Play => {
                if let Some(track) = self.current() {
                    self.set_track(&track)?;
                }
                self.play();
            }
            PlayerState::Pause => self.pause(),
            PlayerState::Stop => self.stop(),
        }
        self.write_state(state);
        Ok(())
    }

    fn state(&self) -> PlayerState {
        self.state.read()
    }

    fn set_volume(&self, volume: f32) -> Result<(), Error> {
        if let Some(sink) = self.current_sink.lock().unwrap().deref_mut() {
            sink.set_volume(volume);
        }
        Ok(())
    }

    fn volume(&self) -> f32 {
        if let Some(sink) = self.current_sink.lock().unwrap().deref_mut() {
            sink.volume()
        } else {
            1f32
        }
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

    fn observe(&self) -> Receiver<PlayerEvent> {
        self.rx.clone()
    }

    fn as_any(&self) -> &Any {
        self
    }
}
