use std::collections::HashMap;
use std::fs::{create_dir_all, OpenOptions};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use failure::{format_err, Error};
use log::{debug, trace};
use pinboard::NonEmptyPinboard;
use reqwest::get;

use crate::Track;
use crate::provider::CoverArt;

#[derive(Debug)]
struct CachedEntry {
    uri: String,
    filename: String,
}

#[derive(Debug)]
pub struct Cache {
    pub tracks: Arc<NonEmptyPinboard<HashMap<String, String>>>,
}

pub type SharedCache = Arc<Cache>;

pub fn setup() -> Result<(), Error> {
    create_dir_all(".cache/music")?;
    create_dir_all(".cache/coverart")?;

    Ok(())
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            tracks: Arc::new(NonEmptyPinboard::new(HashMap::new())),
        }
    }

    pub fn fetch_track(&self, track: &Track, stream_url: &str) -> Result<String, Error> {
        trace!("fetch_track (track: {}, stream_url: {})", track, stream_url);
        {
            let map = self.tracks.read();
            if let Some(file) = map.get(&track.uri) {
                return Ok(format!("file://.cache/music/{}", file));
            }
        }
        trace!("track not cached yet");
        let entry = cache_track(track, stream_url)?;
        {
            let mut map = self.tracks.read();
            map.insert(entry.uri.clone(), entry.filename.clone());
            self.tracks.set(map);
        }
        Ok(format!("file://.cache/music/{}", entry.filename))
    }

    pub fn prepare_track(&self, track: &Track, stream_url: &str) -> Result<(), Error> {
        self.fetch_track(track, stream_url)?;
        Ok(())
    }

    pub async fn fetch_coverart(&self, cover: &CoverArt) -> Result<Option<CoverArt>, Error> {
        match cover {
            CoverArt::Url(ref url) => {
                let path = self.get_coverart_path(url);
                if path.exists() {
                    let file = self.read_cache_file(&path).await?;
                    Ok(Some(file))
                }else {
                    Ok(None)
                }
            },
            _ => Ok(None)
        }
    }

    pub async fn cache_coverart(&self, cover: &CoverArt) -> Result<CoverArt, Error> {
        match cover {
            CoverArt::Url(ref url) => self.download_coverart(url).await,
            _ => Err(format_err!(""))
        }
    }

    fn get_coverart_path(&self, url: &str) -> PathBuf {
        let hash = md5::compute(url);
        let path = format!(".cache/coverart/{:x}", hash);

        let mut result = PathBuf::new();
        result.push(&path);

        result
    }

    async fn download_coverart(&self, url: &str) -> Result<CoverArt, Error> {
        use tokio_util::compat::Tokio02AsyncWriteCompatExt;
        use futures::prelude::*;
        use tokio::fs::File;

        let file_path = self.get_coverart_path(url);
        let mut file = File::create(&file_path).await?.compat_write();
        let res = reqwest_10::get(url).await?;
        let stream = res.bytes_stream().map_err(|e|
            futures::io::Error::new(futures::io::ErrorKind::Other, e)
        ).into_async_read();

        futures::io::copy(stream, &mut file).await?;

        self.read_cache_file(&file_path).await
    }

    async fn read_cache_file(&self, path: &Path) -> Result<CoverArt, Error> {
        let data = tokio::fs::read(path).await?;

        Ok(CoverArt::Data {
            mime_type: String::from("image/jpeg"),
            data
        })
    }
}

fn cache_track(track: &Track, stream_url: &str) -> Result<CachedEntry, Error> {
    trace!("cache_track (track: {}, stream_url: {})", track, stream_url);
    let base = ".cache/music";
    let hash = md5::compute(&track.uri);
    let filename = format!("{:x}", hash);
    let path = format!("{}/{}", base, filename);
    if Path::new(&path).exists() {
        trace!("file already exists");
        return Ok(CachedEntry {
            filename,
            uri: track.uri.clone(),
        });
    }

    debug!("{} -> {}", &track.uri, &filename);

    let mut file = OpenOptions::new().create(true).write(true).open(&path)?;
    let mut res = get(stream_url)?;
    res.copy_to(&mut file)?;

    Ok(CachedEntry {
        filename,
        uri: track.uri.clone(),
    })
}
