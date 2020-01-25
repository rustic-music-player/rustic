use std::any::Any;
use std::io::BufReader;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crossbeam_channel::Sender;
use failure::{bail, Error, format_err};
use log::{debug, trace};
use pinboard::NonEmptyPinboard;
use url::Url;

use rustic_core::{PlayerEvent, PlayerState, Rustic, Track};
use rustic_core::player::{PlayerBackend, PlayerBuilder, QueueCommand};

use crate::file::RodioFile;

mod file;

pub struct RodioBackend {
    core: Arc<Rustic>,
    state: NonEmptyPinboard<PlayerState>,
    blend_time: Duration,
    current_sink: Arc<Mutex<Option<rodio::Sink>>>,
    device: rodio::Device,
    player_events: Sender<PlayerEvent>,
    next_sender: Sender<()>,
}

impl std::fmt::Debug for RodioBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "RodioBackend {{ state: {:?}, blend_time: {:?} }}",
            self.state, self.blend_time
        )
    }
}

impl RodioBackend {
    pub fn new(
        core: Arc<Rustic>,
        queue_tx: Sender<QueueCommand>,
        player_events: Sender<PlayerEvent>,
    ) -> Result<Box<dyn PlayerBackend>, Error> {
        let device = rodio::default_output_device()
            .ok_or_else(|| format_err!("Unable to open output device"))?;
        let (next_sender, next_receiver) = crossbeam_channel::unbounded();
        let backend = RodioBackend {
            core,
            state: NonEmptyPinboard::new(PlayerState::Stop),
            blend_time: Duration::default(),
            current_sink: Arc::new(Mutex::new(None)),
            device,
            player_events,
            next_sender,
        };

        thread::spawn(move || {
            for _ in next_receiver {
                queue_tx.send(QueueCommand::Next);
            }
        });

        Ok(Box::new(backend))
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
        self.player_events.send(PlayerEvent::StateChanged(state));
    }

    fn play(&self) {
        self.write_state(PlayerState::Play);
        if let Some(sink) = self.current_sink.lock().unwrap().deref_mut() {
            sink.play();
        }
    }

    fn pause(&self) {
        if let Some(sink) = self.current_sink.lock().unwrap().deref_mut() {
            sink.pause();
            self.write_state(PlayerState::Pause);
        }
    }

    fn stop(&self) {
        self.write_state(PlayerState::Stop);
        if let Some(sink) = self.current_sink.lock().unwrap().deref_mut() {
            sink.stop();
        }
    }
}

impl PlayerBackend for RodioBackend {
    fn set_track(&self, track: &Track) -> Result<(), Error> {
        debug!("Selecting {:?}", track);
        {
            self.player_events
                .send(PlayerEvent::TrackChanged(track.clone()))?;
            let source = self.decode_stream(track, self.core.stream_url(track)?)?;
            let sink = rodio::Sink::new(&self.device);
            sink.append(source);
            if self.state() != PlayerState::Play {
                sink.stop();
            }
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

    fn set_state(&self, state: PlayerState) -> Result<(), Error> {
        match state {
            PlayerState::Play => self.play(),
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait RodioPlayerBuilder {
    fn with_rodio(&mut self) -> Result<&mut Self, Error>;
}

impl RodioPlayerBuilder for PlayerBuilder {
    fn with_rodio(&mut self) -> Result<&mut Self, Error> {
        self.with_player(|core, queue_tx, _, event_tx| RodioBackend::new(core, queue_tx, event_tx))
    }
}
