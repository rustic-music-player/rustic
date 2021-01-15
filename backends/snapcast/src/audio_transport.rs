use failure::Error;
use smol::process::{Child, Command};

use snapcast_api::SnapcastClient;

#[derive(Debug, Clone)]
pub enum SnapcastAudioTransport {
    Tcp { host: String, port: u16 },
    Pipe(String),
}

impl SnapcastAudioTransport {
    pub async fn add_stream(&self, name: &str, client: &SnapcastClient) -> Result<String, Error> {
        let url = match self {
            SnapcastAudioTransport::Tcp { host, port } => format!(
                "tcp://{}:{}?name={}&mode=server&codec=pcm",
                &host, &port, name
            ),
            SnapcastAudioTransport::Pipe(pipe) => {
                unix_named_pipe::create(&pipe, Some(0o644))?;
                format!("pipe://{}?name={}&mode=read&codec=pcm", &pipe, name)
            }
        };
        let stream_id = client.add_stream(url).await?;
        Ok(stream_id)
    }

    pub async fn close(&self) -> Result<(), Error> {
        if let SnapcastAudioTransport::Pipe(pipe) = self {
            smol::fs::remove_file(&pipe).await?;
        }
        Ok(())
    }

    pub fn spawn(&self, path: &str) -> Result<Child, Error> {
        let target = match self {
            SnapcastAudioTransport::Pipe(pipe) => pipe.clone(),
            SnapcastAudioTransport::Tcp { host, port } => format!("tcp://{}:{}", host, port),
        };
        let child = Command::new("ffmpeg")
            .args(&[
                "-y",
                "-i",
                &path,
                "-f",
                "u16le",
                "-acodec",
                "pcm_s16le",
                "-ac",
                "2",
                "-ar",
                "48000",
                &target,
            ])
            .spawn()?;

        Ok(child)
    }
}
