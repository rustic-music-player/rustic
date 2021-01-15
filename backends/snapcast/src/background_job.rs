use crate::audio_transport::SnapcastAudioTransport;
use crate::BackgroundCommand;
use failure::Error;
use rustic_core::{PlayerState, Rustic, Track};
use smol::process::Child;
use std::sync::Arc;
use url::Url;

pub struct BackgroundJob {
    pub core: Arc<Rustic>,
    pub transport: SnapcastAudioTransport,
    pub child: Option<Child>,
}

impl BackgroundJob {
    pub fn new(core: Arc<Rustic>, transport: SnapcastAudioTransport) -> Self {
        BackgroundJob {
            core,
            transport: transport.clone(),
            child: None,
        }
    }

    pub fn has_child_exited(&mut self) -> bool {
        self.child
            .as_mut()
            .and_then(|child| child.try_status().ok())
            .flatten()
            .is_some()
    }

    async fn get_child(&mut self) -> Result<(), Error> {
        match self.child {
            Some(ref mut child) => {
                child.status().await?;

                Ok(())
            }
            None => {
                futures::future::pending::<()>().await;

                Ok(())
            }
        }
    }

    pub fn handle_cmd(&mut self, cmd: BackgroundCommand) -> Result<(), Error> {
        match cmd {
            BackgroundCommand::Play(track, url) => {
                self.decode_stream(&track, url)?;
            }
            BackgroundCommand::SetState(PlayerState::Stop) => {
                if let Some(mut child) = self.child.take() {
                    child.kill()?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn decode_stream(&mut self, track: &Track, stream_url: String) -> Result<(), Error> {
        log::trace!("Decoding stream {} for track {}", &stream_url, track);
        let url = Url::parse(&stream_url)?;
        match url.scheme() {
            "file" => self.play_file(stream_url),
            "http" | "https" => {
                let path = self.core.cache.fetch_track(track, &stream_url)?;
                self.play_file(path)
            }
            scheme => failure::bail!("Invalid scheme: {}", scheme),
        }
    }

    fn play_file(&mut self, mut path: String) -> Result<(), Error> {
        path.replace_range(..7, "");
        log::trace!("Playing file {}", &path);

        if let Some(mut child) = self.child.take() {
            child.kill()?
        }

        self.child = Some(self.transport.spawn(&path)?);

        Ok(())
    }
}
