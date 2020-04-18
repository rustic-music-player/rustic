use std::sync::Arc;

use crossbeam_channel::Receiver;
use log::{debug, error, trace, warn};
use rust_cast::channels::media::{Media, Metadata, MusicTrackMediaMetadata, StreamType};
use rust_cast::channels::receiver::{Application, CastDeviceApp};
use rust_cast::CastDevice;

use rustic_core::PlayerState;

use crate::internal_command::InternalCommand;

pub struct CastCommandTask {
    core: Arc<rustic_core::Rustic>,
    app: Option<Application>,
    receiver: Receiver<InternalCommand>,
}

impl CastCommandTask {
    pub fn new(receiver: Receiver<InternalCommand>, core: Arc<rustic_core::Rustic>) -> Self {
        CastCommandTask {
            core,
            receiver,
            app: None,
        }
    }

    pub fn next(&mut self, device: &CastDevice<'_>) -> Result<(), failure::Error> {
        match self.receiver.recv() {
            Ok(InternalCommand::Play(track)) => {
                if self.app.is_none() {
                    debug!("Launching app...");
                    let app = device
                        .receiver
                        .launch_app(&CastDeviceApp::DefaultMediaReceiver);
                    debug!("Launched app {:?}", &app);
                    self.app = Some(app?);
                    debug!("got app {:?}", self.app);
                }
                if let Some(app) = self.app.as_ref() {
                    trace!("trying to play on application {:?}", app);
                    device.connection.connect(app.transport_id.as_str())?;

                    let media = Media {
                        content_id: self.core.stream_url(&track)?,
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
                    debug!("Loading Media {:?}", media);
                    device.media.load(
                        app.transport_id.as_str(),
                        app.session_id.as_str(),
                        &media,
                    )?;
                }
            }
            Ok(InternalCommand::Volume(volume)) => {
                debug!("Setting volume {}", volume);
                device.receiver.set_volume(volume)?;
            }
            Ok(InternalCommand::SetState(state)) => {
                if let Some(app) = self.app.as_ref() {
                    let status = device.media.get_status(&app.transport_id, None)?;
                    if let Some(media_id) =
                        status.entries.first().map(|status| status.media_session_id)
                    {
                        match state {
                            PlayerState::Play => {
                                debug!("playing media_id: {}", media_id);
                                device.media.play(&app.transport_id, media_id)?;
                            }
                            PlayerState::Pause => {
                                debug!("pausing media_id: {}", media_id);
                                device.media.pause(&app.transport_id, media_id)?;
                            }
                            PlayerState::Stop => {
                                debug!("stopping media_id: {}", media_id);
                                device.media.stop(&app.transport_id, media_id)?;
                            }
                        }
                    }
                } else {
                    warn!("Trying to set state to {:?} while no app is known", state)
                }
            }
            _ => (),
        }
        Ok(())
    }
}
