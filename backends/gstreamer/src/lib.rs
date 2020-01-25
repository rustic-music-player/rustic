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

use channel::Sender;
use failure::{err_msg, Error};
use gst::{prelude::*, MessageView};
use pinboard::NonEmptyPinboard;

use core::player::{PlayerBackend, PlayerBuilder, PlayerEvent, PlayerState, QueueCommand};
use core::Track;

pub struct GstBackend {
    core: Arc<core::Rustic>,
    current_volume: f32,
    state: NonEmptyPinboard<PlayerState>,
    blend_time: Duration,
    pipeline: gst::Pipeline,
    decoder: gst::Element,
    volume: gst::Element,
    sink: gst::Element,
    player_events: Sender<PlayerEvent>,
}

impl std::fmt::Debug for GstBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "GstBackend {{ volume: {}, state: {:?}, blend_time: {:?} }}",
            self.current_volume, self.state, self.blend_time
        )
    }
}

impl GstBackend {
    pub fn new(
        core: Arc<core::Rustic>,
        queue_tx: Sender<QueueCommand>,
        player_events: Sender<PlayerEvent>,
    ) -> Result<Box<dyn PlayerBackend>, Error> {
        gst::init()?;
        let pipeline = gst::Pipeline::new(None);
        let decoder = gst::ElementFactory::make("uridecodebin", None)?;
        let volume = gst::ElementFactory::make("volume", None)?;
        let sink = gst::ElementFactory::make("autoaudiosink", None)?;
        let backend = GstBackend {
            core,
            blend_time: Duration::default(),
            current_volume: 1.0,
            state: NonEmptyPinboard::new(PlayerState::Stop),
            pipeline,
            decoder,
            volume,
            sink,
            player_events,
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

        let bus = backend
            .pipeline
            .get_bus()
            .ok_or_else(|| format_err!("can't get gst bus"))?;
        thread::spawn(move || loop {
            let _res: Result<(), Error> = match bus.pop() {
                None => Ok(()),
                Some(msg) => match msg.view() {
                    MessageView::Eos(..) => {
                        println!("eos");
                        queue_tx.send(QueueCommand::Next).map_err(|err| err.into())
                    }
                    MessageView::Error(err) => {
                        error!(
                            "Error from {}: {} ({:?})",
                            msg.get_src().unwrap().get_path_string(),
                            err.get_error(),
                            err.get_debug()
                        );
                        Err(format_err!(
                            "Error from {}: {} ({:?})",
                            msg.get_src().unwrap().get_path_string(),
                            err.get_error(),
                            err.get_debug()
                        ))
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
        });

        Ok(Box::new(backend))
    }

    fn write_state(&self, state: PlayerState) {
        self.state.set(state);
        self.player_events.send(PlayerEvent::StateChanged(state));
    }
}

impl PlayerBackend for GstBackend {
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

        self.player_events
            .send(PlayerEvent::TrackChanged(track.clone()))?;

        Ok(())
    }

    fn set_state(&self, new_state: PlayerState) -> Result<(), Error> {
        debug!("set_state, {:?}", &new_state);
        match new_state {
            PlayerState::Play => {
                self.pipeline.set_state(gst::State::Playing)?;
                self.write_state(new_state);
                Ok(())
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait GstreamerPlayerBuilder {
    fn with_gstreamer(&mut self) -> Result<&mut Self, Error>;
}

impl GstreamerPlayerBuilder for PlayerBuilder {
    fn with_gstreamer(&mut self) -> Result<&mut Self, Error> {
        self.with_player(|core, queue_tx, _, event_tx| GstBackend::new(core, queue_tx, event_tx))
    }
}
