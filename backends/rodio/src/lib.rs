use std::any::Any;
use std::io::BufReader;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use failure::{bail, Error};
use log::{debug, trace, error};
use pinboard::NonEmptyPinboard;
use url::Url;

use rustic_core::{player::MemoryQueue, PlayerBackend, PlayerEvent, PlayerState, Rustic, Track};

use crate::file::RodioFile;

mod file;

pub struct RodioBackend {
    core: Arc<Rustic>,
    queue: MemoryQueue,
    state: NonEmptyPinboard<PlayerState>,
    blend_time: Duration,
    current_sink: Arc<Mutex<Option<rodio::Sink>>>,
    device: rodio::Device,
    tx: Sender<PlayerEvent>,
    rx: Receiver<PlayerEvent>,
    next_sender: Sender<()>,
}

impl std::fmt::Debug for RodioBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "RodioBackend {{ queue: {:?}, state: {:?}, blend_time: {:?} }}",
            self.queue, self.state, self.blend_time
        )
    }
}

impl RodioBackend {
    pub fn new(core: Arc<Rustic>) -> Result<Arc<Box<dyn PlayerBackend>>, Error> {
        let device = rodio::default_output_device().unwrap();
        let (tx, rx) = crossbeam_channel::unbounded();
        let (next_sender, next_receiver) = crossbeam_channel::unbounded();
        let backend = RodioBackend {
            core,
            queue: MemoryQueue::new(),
            state: NonEmptyPinboard::new(PlayerState::Stop),
            blend_time: Duration::default(),
            current_sink: Arc::new(Mutex::new(None)),
            device,
            tx,
            rx,
            next_sender,
        };
        let backend: Arc<Box<dyn PlayerBackend>> = Arc::new(Box::new(backend));

        let receiver_backend = Arc::clone(&backend);
        thread::spawn(move || {
            let backend = receiver_backend;
            for _ in next_receiver {
                if let Err(err) = backend.next() {
                    error!("Failed to select next track {}", err)
                }
            }
        });

        Ok(backend)
    }

    fn decode_stream(
        &self,
        track: &Track,
        stream_url: String,
    ) -> Result<rodio::Decoder<BufReader<RodioFile>>, Error> {
        trace!("Decoding stream {} for track {}", &stream_url, track);
        let url = Url::parse(&stream_url)?;
        match url.scheme() {
            "file" => self.decode_file(stream_url),
            "http" | "https" => {
                let path = self.core.cache.fetch_track(track, &stream_url)?;
                self.decode_file(path)
            }
            scheme => bail!("Invalid scheme: {}", scheme),
        }
    }

    fn decode_file(&self, mut path: String) -> Result<rodio::Decoder<BufReader<RodioFile>>, Error> {
        path.replace_range(..7, "");
        trace!("Decoding file {}", &path);
        let file = RodioFile::open(path, self.next_sender.clone())?;
        let decoder = rodio::Decoder::new(BufReader::new(file))?;
        Ok(decoder)
    }

    fn write_state(&self, state: PlayerState) {
        if self.state.read() == state {
            return;
        }
        self.state.set(state);
        self.tx.send(PlayerEvent::StateChanged(state));
    }

    fn set_track(&self, track: &Track) -> Result<(), Error> {
        debug!("Selecting {:?}", track);
        {
            self.tx.send(PlayerEvent::TrackChanged(track.clone()))?;
            let source = self.decode_stream(track, self.core.stream_url(track)?)?;
            let sink = rodio::Sink::new(&self.device);
            sink.append(source);
            let mut current_sink = self.current_sink.lock().unwrap();
            if let Some(prev_sink) = current_sink.take() {
                trace!("Removing previous sink");
                prev_sink.stop();
            }
            *current_sink = Some(sink);
        } // Drop the lock
        if self.state.read() == PlayerState::Play {
            self.play();
        }
        Ok(())
    }

    fn play(&self) {
        if let Some(sink) = self.current_sink.lock().unwrap().deref_mut() {
            sink.play();
            self.write_state(PlayerState::Play);
        }
    }

    fn pause(&self) {
        if let Some(sink) = self.current_sink.lock().unwrap().deref_mut() {
            sink.pause();
            self.write_state(PlayerState::Pause);
        }
    }

    fn stop(&self) {
        if let Some(sink) = self.current_sink.lock().unwrap().deref_mut() {
            sink.stop();
            self.write_state(PlayerState::Stop);
        }
    }

    fn queue_changed(&self) {
        self.tx
            .send(PlayerEvent::QueueUpdated(self.queue.get_queue()));
    }
}

impl PlayerBackend for RodioBackend {
    fn queue_single(&self, track: &Track) {
        self.queue.queue_single(track);
        self.queue_changed();
    }

    fn queue_multiple(&self, tracks: &[Track]) {
        self.queue.queue_multiple(tracks);
        self.queue_changed();
    }

    fn queue_next(&self, track: &Track) {
        self.queue.queue_next(track);
        self.queue_changed();
    }

    fn get_queue(&self) -> Vec<Track> {
        self.queue.get_queue()
    }

    fn clear_queue(&self) {
        self.queue.clear();
        self.set_state(PlayerState::Stop);
        self.queue_changed();
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
                let is_paused = self.state.read() == PlayerState::Pause;
                if is_paused {
                    self.play();
                } else if let Some(track) = self.current() {
                    self.set_track(&track)?;
                    self.play();
                }
            }
            PlayerState::Pause => self.pause(),
            PlayerState::Stop => self.stop(),
        }
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

    fn set_blend_time(&self, _duration: Duration) -> Result<(), Error> {
        unimplemented!()
    }

    fn blend_time(&self) -> Duration {
        unimplemented!()
    }

    fn seek(&self, _duration: Duration) -> Result<(), Error> {
        unimplemented!()
    }

    fn observe(&self) -> Receiver<PlayerEvent> {
        self.rx.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
