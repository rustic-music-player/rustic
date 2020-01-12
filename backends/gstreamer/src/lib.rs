extern crate crossbeam_channel as channel;
#[macro_use]
extern crate failure;
extern crate gstreamer as gst;
#[macro_use]
extern crate log;
extern crate pinboard;
extern crate rustic_core as core;

use std::any::Any;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use channel::{Receiver, Sender};
use failure::{err_msg, Error};
use gst::{MessageView, prelude::*};
use pinboard::NonEmptyPinboard;

use core::{player::MemoryQueue, PlayerBackend, PlayerEvent, PlayerState, Track};

pub struct GstBackend {
    core: Arc<core::Rustic>,
    queue: MemoryQueue,
    current_track: NonEmptyPinboard<Option<Track>>,
    current_volume: f32,
    state: NonEmptyPinboard<PlayerState>,
    blend_time: Duration,
    pipeline: gst::Pipeline,
    decoder: gst::Element,
    volume: gst::Element,
    sink: gst::Element,
    tx: Sender<PlayerEvent>,
    rx: Receiver<PlayerEvent>,
}

impl std::fmt::Debug for GstBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "GstBackend {{ queue: {:?}, volume: {}, state: {:?}, blend_time: {:?} }}",
            self.queue, self.current_volume, self.state, self.blend_time
        )
    }
}

impl GstBackend {
    pub fn new(core: Arc<core::Rustic>) -> Result<Arc<Box<dyn PlayerBackend>>, Error> {
        gst::init()?;
        let pipeline = gst::Pipeline::new(None);
        let decoder = gst::ElementFactory::make("uridecodebin", None)?;
        let volume = gst::ElementFactory::make("volume", None)?;
        let sink = gst::ElementFactory::make("autoaudiosink", None)?;
        let (tx, rx) = channel::unbounded();
        let backend = GstBackend {
            core,
            queue: MemoryQueue::new(),
            current_track: NonEmptyPinboard::new(None),
            blend_time: Duration::default(),
            current_volume: 1.0,
            state: NonEmptyPinboard::new(PlayerState::Stop),
            pipeline,
            decoder,
            volume,
            sink,
            tx,
            rx,
        };

        backend.pipeline.add(&backend.decoder)?;
        backend.pipeline.add(&backend.volume)?;
        backend.pipeline.add(&backend.sink)?;

        backend.volume.link(&backend.sink)?;

        let sink_pad = backend
            .volume
            .get_static_pad("sink")
            .ok_or_else(|| err_msg("missing sink pad on volume element"))?;
        backend
            .decoder
            .connect_pad_added(move |_el: &gst::Element, pad: &gst::Pad| {
                pad.link(&sink_pad);
            });

        let backend: Arc<Box<dyn PlayerBackend>> = Arc::new(Box::new(backend));

        {
            let gst_backend = Arc::clone(&backend);
            let backend = Arc::clone(&backend);
            thread::spawn(move || {
                let gst_backend: &GstBackend =
                    match gst_backend.as_any().downcast_ref::<GstBackend>() {
                        Some(b) => b,
                        None => panic!("Not a GstBackend"),
                    };
                if let Some(bus) = gst_backend.pipeline.get_bus() {
                    loop {
                        let _res: Result<(), Error> = match bus.pop() {
                            None => Ok(()),
                            Some(msg) => match msg.view() {
                                MessageView::Eos(..) => {
                                    println!("eos");
                                    backend.next()?;
                                    Ok(())
                                }
                                MessageView::Error(err) => {
                                    error!(
                                        "Error from {}: {} ({:?})",
                                        msg.get_src().unwrap().get_path_string(),
                                        err.get_error(),
                                        err.get_debug()
                                    );
                                    bail!(
                                        "Error from {}: {} ({:?})",
                                        msg.get_src().unwrap().get_path_string(),
                                        err.get_error(),
                                        err.get_debug()
                                    );
                                }
                                MessageView::Buffering(buffering) => {
                                    debug!("buffering {}", buffering.get_percent());
                                    Ok(())
                                }
                                MessageView::Warning(warning) => {
                                    warn!("gst warning {:?}", warning.get_debug());
                                    Ok(())
                                }
                                MessageView::Info(info) => {
                                    info!("gst info {:?}", info.get_debug());
                                    Ok(())
                                }
                                _ => Ok(()),
                            },
                        };
                    }
                }
                Ok(())
            });
        }

        Ok(backend)
    }

    fn set_track(&self, track: &Track) -> Result<(), Error> {
        debug!("Selecting {:?}", track);
        self.pipeline.set_state(gst::State::Null)?;

        let stream_url = self.core.stream_url(track)?;

        self.decoder
            .set_property_from_str("uri", stream_url.as_str());

        let state = match self.state.read() {
            PlayerState::Play => gst::State::Playing,
            PlayerState::Pause => gst::State::Paused,
            PlayerState::Stop => gst::State::Null,
        };

        self.pipeline.set_state(state)?;

        self.tx.send(PlayerEvent::TrackChanged(track.clone()))?;
        self.current_track.set(Some(track.clone()));

        Ok(())
    }

    fn write_state(&self, state: PlayerState) {
        self.state.set(state);
        self.tx.send(PlayerEvent::StateChanged(state));
    }
}

impl PlayerBackend for GstBackend {
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
        self.current_track.read()
    }

    fn prev(&self) -> Result<Option<()>, Error> {
        let track = self.queue.prev();
        if let Some(track) = track {
            self.set_track(&track)?;
            Ok(Some(()))
        } else {
            self.set_state(PlayerState::Stop)?;
            Ok(None)
        }
    }

    fn next(&self) -> Result<Option<()>, Error> {
        let track = self.queue.next();
        if let Some(track) = track {
            self.set_track(&track)?;
            Ok(Some(()))
        } else {
            self.set_state(PlayerState::Stop)?;
            Ok(None)
        }
    }

    fn set_state(&self, new_state: PlayerState) -> Result<(), Error> {
        debug!("set_state, {:?}", &new_state);
        match new_state {
            PlayerState::Play => {
                if let Some(track) = self.queue.get_current_track() {
                    self.write_state(new_state);
                    self.set_track(&track)
                } else {
                    self.write_state(PlayerState::Stop);
                    Ok(())
                }
            }
            PlayerState::Pause => {
                self.pipeline.set_state(gst::State::Paused)?;
                self.write_state(new_state);
                Ok(())
            }
            PlayerState::Stop => {
                self.pipeline.set_state(gst::State::Null)?;
                self.write_state(new_state);
                Ok(())
            }
        }
    }

    fn state(&self) -> PlayerState {
        self.state.read()
    }

    fn set_volume(&self, _volume: f32) -> Result<(), Error> {
        unimplemented!()
    }

    fn volume(&self) -> f32 {
        self.current_volume
    }

    fn set_blend_time(&self, _duration: Duration) -> Result<(), Error> {
        unimplemented!()
    }

    fn blend_time(&self) -> Duration {
        self.blend_time
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
