use std::any::Any;
use std::sync::Arc;
use std::time::Duration;

use failure::Error;
use pinboard::NonEmptyPinboard;

use rustic_core::player::{
    PlayerBackend, PlayerBuilder, PlayerBus, PlayerEvent, PlayerState, QueueCommand,
};
use rustic_core::{Rustic, Track};

pub struct GstBackend {
    core: Arc<Rustic>,
    current_volume: f32,
    state: NonEmptyPinboard<PlayerState>,
    blend_time: Duration,
    player: gstreamer_player::Player,
    bus: PlayerBus,
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
    pub fn new(core: Arc<Rustic>, bus: PlayerBus) -> Result<Box<dyn PlayerBackend>, Error> {
        // TODO: only run once
        std::thread::spawn(|| glib::MainLoop::new(None, false).run());
        gstreamer::init()?;
        let player = gstreamer_player::Player::new(None, None);
        let backend = GstBackend {
            core,
            blend_time: Duration::default(),
            current_volume: 1.0,
            state: NonEmptyPinboard::new(PlayerState::Stop),
            player,
            bus: bus.clone(),
        };

        let eos_bus = bus.clone();
        let error_bus = bus;
        backend.player.connect_end_of_stream(move |_| {
            log::debug!("reached end of stream");
            if let Err(e) = eos_bus.send_queue_msg(QueueCommand::Next) {
                log::error!("Failed loading next track: {:?}", e)
            }
        });
        backend.player.connect_state_changed(|p, state| {
            log::debug!("state changed to {} for player: {:?}", state, p);
        });
        backend.player.connect_error(move |_, err| {
            log::debug!("skipping track because of playback error");
            log::error!("{:?}", err);
            if let Err(e) = error_bus.send_queue_msg(QueueCommand::Next) {
                log::error!("Failed loading next track: {:?}", e)
            }
        });
        backend.player.connect_buffering(|_, p| {
            log::debug!("buffering {}", p);
        });

        Ok(Box::new(backend))
    }

    fn write_state(&self, state: PlayerState) -> Result<(), Error> {
        self.state.set(state);
        self.bus.emit_event(PlayerEvent::StateChanged(state))?;

        Ok(())
    }
}

impl PlayerBackend for GstBackend {
    fn set_track(&self, track: &Track, stream_url: String) -> Result<(), Error> {
        log::debug!("Selecting {:?}", track);

        self.player.set_uri(Some(stream_url.as_str()));

        match self.state.read() {
            PlayerState::Play => self.player.play(),
            PlayerState::Pause => self.player.pause(),
            PlayerState::Stop => self.player.stop(),
        }

        self.bus
            .emit_event(PlayerEvent::TrackChanged(track.clone()))?;

        Ok(())
    }

    fn set_state(&self, new_state: PlayerState) -> Result<(), Error> {
        log::debug!("set_state, {:?}", &new_state);
        match new_state {
            PlayerState::Play => {
                self.player.play();
                self.write_state(new_state)?;
                Ok(())
            }
            PlayerState::Pause => {
                self.player.pause();
                self.write_state(new_state)?;
                Ok(())
            }
            PlayerState::Stop => {
                self.player.stop();
                self.write_state(new_state)?;
                Ok(())
            }
        }
    }

    fn state(&self) -> PlayerState {
        self.state.read()
    }

    fn set_volume(&self, volume: f32) -> Result<(), Error> {
        self.player.set_volume(volume as f64);
        self.bus.emit_event(PlayerEvent::VolumeChanged(volume))?;
        Ok(())
    }

    fn volume(&self) -> f32 {
        self.player.volume() as f32
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
        self.with_player(GstBackend::new)
    }
}
