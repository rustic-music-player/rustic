use std::collections::HashMap;
use std::fs::{create_dir_all, OpenOptions};
use std::path::Path;
use std::sync::Arc;

use failure::Error;
use log::{debug, trace};
use md5;
use pinboard::NonEmptyPinboard;
use reqwest::get;

use crate::Track;

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
