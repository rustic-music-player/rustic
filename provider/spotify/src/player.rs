use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use failure::{format_err, Error};
use librespot::audio::{AudioDecrypt, AudioFile};
use librespot::core::authentication::{Credentials};
use librespot::core::cache::Cache;
use librespot::core::config::SessionConfig;
use librespot::core::session::Session;
use librespot::core::spotify_id::SpotifyId;
use librespot::metadata::AudioItem;
use librespot::protocol::metadata::AudioFile_Format;
use log::trace;

#[derive(Clone)]
pub struct SpotifyPlayer {
    cache: Cache,
    credentials: Option<Credentials>,
}

impl SpotifyPlayer {
    pub fn new() -> SpotifyPlayer {
        let cache = Cache::new(
            Some(PathBuf::from(".cache/spotify/data")),
            Some(PathBuf::from(".cache/spotify/audio")),
            None
        ).unwrap();

        SpotifyPlayer {
            cache,
            credentials: None,
        }
    }

    pub fn setup(&mut self, username: &str, password: &str) {
        let credentials = self.cache.credentials().unwrap_or_else(|| Credentials::with_password(username, password));

        self.credentials = Some(credentials);
    }

    pub async fn get_audio_file(&self, uri: &str) -> Result<&str, Error> {
        trace!("get_audio_file {}", uri);
        let session = self.connect().await?;
        trace!("connected");
        let id =
            SpotifyId::from_uri(uri).map_err(|err| format_err!("spotify id error {:?}", err))?;
        trace!("id {:?}", id);
        let audio = AudioItem::get_audio_item(&session, id).await.map_err(|err| failure::format_err!("fetching audio item failed {:?}", err))?;

        trace!("AudioItem {:#?}", audio);

        let (format, data_rate) = (AudioFile_Format::OGG_VORBIS_320, 40 * 1024);

        let file_id = *audio
            .files
            .get(&format)
            .ok_or_else(|| format_err!("no ogg available"))?;
        trace!("FileId {:?}", file_id);

        let encrypted_file = AudioFile::open(&session, file_id, data_rate, true).await.map_err(|err| failure::format_err!("opening audio file failed {:?}", err))?;

        trace!("is_cached: {}", encrypted_file.is_cached());
        let stream_loader_controller = encrypted_file.get_stream_loader_controller();
        stream_loader_controller.set_stream_mode();
        trace!("file_size: {}", stream_loader_controller.len());

        trace!("fetching file");
        stream_loader_controller.fetch_next_blocking(stream_loader_controller.len());

        let key = session.audio_key().request(id, file_id).await.map_err(|err| failure::format_err!("fetching audio key failed {:?}", err))?;

        trace!("decrypting file");
        let decrypted_file = AudioDecrypt::new(key, encrypted_file);
        let mut audio_file = Subfile::new(decrypted_file, 0xa7);

        trace!("reading file");
        let mut buffer = Vec::new();
        let bytes = audio_file.read_to_end(&mut buffer)?;
        trace!("read {} bytes", bytes);

        trace!("storing file");
        let file_path = "/tmp/rustic_spotify_test.ogg";
        let mut tmp_file = fs::File::create(&file_path)?;
        tmp_file.write(&buffer)?;

        trace!("file is written");

        session.shutdown();

        Ok(file_path)
    }

    async fn connect(&self) -> Result<Session, failure::Error> {
        let credentials = self.credentials.clone().unwrap();

        let session_config = SessionConfig::default();

        let session = Session::connect(
            session_config,
            credentials,
            Some(self.cache.clone()),
        ).await?;

        Ok(session)
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
