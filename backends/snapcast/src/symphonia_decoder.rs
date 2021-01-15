use crate::audio_transport::SnapcastAudioTransport;
use crate::background_job::BackgroundJob;
use failure::Error;
use std::fs::File;
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
use symphonia::core::audio::RawSampleBuffer;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

impl BackgroundJob {
    pub fn _decode_file(&self, path: &str) -> Result<(), Error> {
        log::trace!("Decoding file {}", &path);
        let mut hint = Hint::new();

        let mut target: Box<dyn Write> = match self.transport {
            SnapcastAudioTransport::Pipe(ref pipe) => {
                log::trace!("opening pipe {}", pipe);
                let pipe = unix_named_pipe::open_write(pipe)?;
                Box::new(pipe)
            }
            SnapcastAudioTransport::Tcp { ref host, ref port } => {
                let socket = TcpStream::connect(&format!("{}:{}", host, port))?;
                Box::new(socket)
            }
        };

        let path = Path::new(&path);

        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            log::trace!("file got extension {}", extension);
            hint.with_extension(extension);
        }

        let file = File::open(path)?;
        let stream = MediaSourceStream::new(Box::new(file));

        let format: FormatOptions = Default::default();
        let metadata: MetadataOptions = Default::default();

        log::trace!("probing file");

        let probe = symphonia::default::get_probe().format(&hint, stream, &format, &metadata)?;

        let mut reader = probe.format;
        let stream = reader.default_stream().unwrap();
        let mut decoder =
            symphonia::default::get_codecs().make(&stream.codec_params, &Default::default())?;

        log::trace!("got decoder");
        let mut buffer = loop {
            match decoder.decode(&reader.next_packet()?) {
                Err(symphonia::core::errors::Error::DecodeError(err)) => {
                    log::warn!("decode error: {}", err);
                    continue;
                }
                Err(err) => {
                    log::error!("{:?}", err);
                    decoder.close();
                    return Err(err.into());
                }
                Ok(decoded) => {
                    log::trace!("writing packet");
                    let spec = *decoded.spec();
                    let duration = decoded.capacity() as symphonia::core::units::Duration;

                    log::debug!(
                        "channels: {:?}, bit rate: {}, capacity: {}",
                        spec.channels,
                        spec.rate,
                        duration
                    );
                    let mut buffer = RawSampleBuffer::<f32>::new(duration, spec);
                    buffer.copy_interleaved_ref(decoded);
                    target.write_all(buffer.as_bytes())?;
                    break buffer;
                }
            }
        };
        loop {
            match decoder.decode(&reader.next_packet()?) {
                Err(symphonia::core::errors::Error::DecodeError(err)) => {
                    log::warn!("decode error: {}", err);
                }
                Err(err) => {
                    decoder.close();
                    return Err(err.into());
                }
                Ok(decoded) => {
                    log::trace!("writing packet");
                    buffer.copy_interleaved_ref(decoded);
                    target.write_all(buffer.as_bytes())?;
                }
            }
        }

        Ok(())
    }
}
