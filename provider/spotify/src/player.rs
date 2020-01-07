use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use failure::{Error, format_err};
use librespot::audio::{AudioDecrypt, AudioFile};
use librespot::core::audio_key::AudioKey;
use librespot::core::authentication::{Credentials, get_credentials};
use librespot::core::cache::Cache;
use librespot::core::config::SessionConfig;
use librespot::core::session::Session;
use librespot::core::spotify_id::SpotifyId;
use librespot::metadata::{AudioItem, Metadata, Playlist};
use librespot::protocol::metadata::AudioFile_Format;
use log::trace;
use tokio_core::reactor::Core;

#[derive(Clone)]
pub struct SpotifyPlayer {
    cache: Cache,
    credentials: Option<Credentials>,
}

impl SpotifyPlayer {
    pub fn new() -> SpotifyPlayer {
        let cache = Cache::new(PathBuf::from(".cache/spotify"), true);

        SpotifyPlayer {
            cache,
            credentials: None,
        }
    }

    pub fn setup(&mut self, username: &str, password: &str) -> Result<(), failure::Error> {
        let credentials = {
            let cached_credentials = self.cache.credentials();

            get_credentials(
                Some(username.to_string()),
                Some(password.to_string()),
                cached_credentials,
                |_| password.to_string(),
            )
        }
        .ok_or_else(|| format_err!("Missing spotify credentials"))?;

        self.credentials = Some(credentials);
        Ok(())
    }

    pub fn get_audio_file(&self, uri: &str) -> Result<(), Error> {
        trace!("get_audio_file {}", uri);
        let (session, mut core) = self.connect()?;
        trace!("connected");
        let id =
            SpotifyId::from_uri(uri).map_err(|err| format_err!("spotify id error {:?}", err))?;
        trace!("id {:?}", id);
        let audio = AudioItem::get_audio_item(&session, id);
        let audio: AudioItem = core
            .run(audio)
            .map_err(|err| format_err!("audio file err {:?}", err))?;

        trace!("AudioItem {:#?}", audio);

        let (format, data_rate) = (AudioFile_Format::OGG_VORBIS_320, 40 * 1024);

        let file_id = *audio
            .files
            .get(&format)
            .ok_or_else(|| format_err!("no mp3 available"))?;
        trace!("FileId {:?}", file_id);

        let key = session.audio_key().request(id, file_id);

        let encrypted_file = AudioFile::open(&session, file_id, data_rate, true);
        let encrypted_file: AudioFile = core
            .run(encrypted_file)
            .map_err(|err| format_err!("encrypted file err {:?}", err))?;

        let mut stream_loader_controller = encrypted_file.get_stream_loader_controller();
        stream_loader_controller.set_stream_mode();

        trace!("decrypting file");
        let key: AudioKey = core
            .run(key)
            .map_err(|err| format_err!("key err {:?}", err))?;
        let decrypted_file = AudioDecrypt::new(key, encrypted_file);

        let mut audio_file = Subfile::new(decrypted_file, 0xa7);

        trace!("reading file");
        let mut buffer = Vec::new();
        let bytes = audio_file.read_to_end(&mut buffer)?;
        trace!("read {} bytes", bytes);

        trace!("storing file");
        let mut tmp_file = fs::File::create("/tmp/rustic_spotify_test.ogg")?;
        tmp_file.write(&buffer)?;

        trace!("file is written");

        session.shutdown();

        Ok(())
    }

    fn connect(&self) -> Result<(Session, Core), failure::Error> {
        let credentials = self.credentials.clone().unwrap();

        let session_config = SessionConfig::default();

        let mut core = Core::new()?;
        let session = core.run(Session::connect(
            session_config,
            credentials,
            Some(self.cache.clone()),
            core.handle(),
        ))?;

        Ok((session, core))
    }
}

impl Default for SpotifyPlayer {
    fn default() -> Self {
        SpotifyPlayer::new()
    }
}

impl std::fmt::Debug for SpotifyPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpotifyPlayer").finish()
    }
}

struct Subfile<T: Read + Seek> {
    stream: T,
    offset: u64,
}

impl<T: Read + Seek> Subfile<T> {
    pub fn new(mut stream: T, offset: u64) -> Subfile<T> {
        stream.seek(SeekFrom::Start(offset)).unwrap();
        Subfile { stream, offset }
    }
}

impl<T: Read + Seek> Read for Subfile<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}

impl<T: Read + Seek> Seek for Subfile<T> {
    fn seek(&mut self, mut pos: SeekFrom) -> std::io::Result<u64> {
        pos = match pos {
            SeekFrom::Start(offset) => SeekFrom::Start(offset + self.offset),
            x => x,
        };

        let newpos = self.stream.seek(pos)?;
        if newpos > self.offset {
            Ok(newpos - self.offset)
        } else {
            Ok(0)
        }
    }
}
