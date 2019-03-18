extern crate crossbeam_channel as channel;
#[macro_use]
extern crate failure;
extern crate rodio;
extern crate rustic_core as core;
extern crate url;

use channel::{Receiver, Sender};
use core::{PlayerBackend, PlayerEvent, PlayerState, Track};
use failure::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

pub struct RodioBackend {
    queue: Vec<Track>,
    current_index: usize,
    current_track: Option<Track>,
    state: PlayerState,
    blend_time: Duration,
    sink: rodio::Sink,
    tx: Sender<PlayerEvent>,
    rx: Receiver<PlayerEvent>,
}

impl RodioBackend {
    pub fn new() -> Result<Arc<RodioBackend>, Error> {
        let device = rodio::default_output_device().unwrap();
        let sink = rodio::Sink::new(&device);
        let (tx, rx) = channel::unbounded();
        let backend = RodioBackend {
            queue: vec![],
            current_index: 0,
            current_track: None,
            state: PlayerState::Stop,
            blend_time: Duration::default(),
            sink,
            tx,
            rx,
        };

        Ok(Arc::new(backend))
    }

    fn decode_stream(url: String) -> Result<rodio::Decoder<BufReader<File>>, Error> {
        let url = Url::parse(&url)?;
        match url.scheme() {
            "file" => {
                let mut path = url.as_str().to_owned();
                path.replace_range(..7, "");
                let file = File::open(path)?;
                let decoder = rodio::Decoder::new(BufReader::new(file))?;
                Ok(decoder)
            }
            scheme => bail!("Invalid scheme: {}", scheme)
        }
    }
}

impl PlayerBackend for RodioBackend {
    fn enqueue(&mut self, track: &Track) {
        self.queue.push(track.clone());
    }

    fn enqueue_multiple(&mut self, tracks: &[Track]) {
        self.queue.append(&mut tracks.to_vec());
    }

    fn play_next(&mut self, track: &Track) {
        self.queue.insert(self.current_index + 1, track.clone());
    }

    fn queue(&self) -> Vec<Track> {
        self.queue.clone()
    }

    fn clear_queue(&mut self) {
        self.queue.clear();
    }

    fn current(&self) -> Option<Track> {
        self.current_track.cloned()
    }

    fn prev(&mut self) -> Result<Option<()>, Error> {
        unimplemented!()
    }

    fn next(&mut self) -> Result<Option<()>, Error> {
        unimplemented!()
    }

    fn set_state(&mut self, state: PlayerState) -> Result<(), Error> {
        match state {
            PlayerState::Play => {
                let track = self.current().unwrap();
                let source = RodioBackend::decode_stream(track.stream_url)?;
                self.sink.append(source);
                self.sink.play();
            }
            PlayerState::Pause => self.sink.pause(),
            PlayerState::Stop => self.sink.stop()
        }
        self.state = state;
        Ok(())
    }

    fn state(&self) -> PlayerState {
        self.state
    }

    fn set_volume(&mut self, volume: f32) -> Result<(), Error> {
        self.sink.set_volume(volume);
        Ok(())
    }

    fn volume(&self) -> f32 {
        self.sink.volume()
    }

    fn set_blend_time(&mut self, duration: Duration) -> Result<(), Error> {
        unimplemented!()
    }

    fn blend_time(&self) -> Duration {
        unimplemented!()
    }

    fn seek(&mut self, duration: Duration) -> Result<(), Error> {
        unimplemented!()
    }

    fn observe(&self) -> Receiver<PlayerEvent> {
        self.rx.clone()
    }
}