use log::{info, error, trace, debug};
use failure::Error;
use image;
use image::FilterType;
use md5;
use reqwest::get;
use std::collections::HashMap;
use std::fs::{create_dir_all, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use crate::Rustic;

const THUMBNAIL_SIZE: u32 = 512;
const SERVICE_INTERVAL: u64 = 30;

#[derive(Debug)]
struct CachedEntry {
    uri: String,
    filename: String,
}

#[derive(Debug, Default)]
pub struct Cache {
    pub coverart: Arc<RwLock<HashMap<String, String>>>,
}

pub type SharedCache = Arc<Cache>;

pub fn start(
    app: Arc<Rustic>,
    running: Arc<(Mutex<bool>, Condvar)>,
) -> Result<thread::JoinHandle<()>, Error> {
    create_dir_all(".cache/coverart")?;

    thread::Builder::new()
        .name("Coverart Cache".into())
        .spawn(move || {
            info!("Starting Coverart Cache");
            let &(ref lock, ref cvar) = &*running;
            let mut keep_running = lock.lock().unwrap();
            while *keep_running {
                info!("Caching Coverart...");
                let result: Result<Vec<CachedEntry>, Error> =
                    app.library.get_tracks().and_then(|tracks| {
                        tracks
                            .iter()
                            .filter(|track| track.image_url.is_some())
                            .filter(|track| {
                                let map = app.cache.coverart.read().unwrap();
                                !map.contains_key(&track.uri)
                            }).map(|track| track.image_url.clone().unwrap())
                            .map(cache_coverart)
                            .collect()
                    });

                match result {
                    Ok(entries) => {
                        info!("Cached {} images", entries.len());
                        let mut map = app.cache.coverart.write().unwrap();
                        for entry in entries {
                            map.insert(entry.uri, entry.filename);
                        }
                    }
                    Err(e) => error!("Error: {:?}", e),
                }

                let result = cvar
                    .wait_timeout(keep_running, Duration::new(SERVICE_INTERVAL, 0))
                    .unwrap();
                keep_running = result.0;
            }
            info!("Coverart Cache stopped");
        }).map_err(Error::from)
}

fn cache_coverart(uri: String) -> Result<CachedEntry, Error> {
    trace!("cache_coverart (uri: {})", &uri);
    let base = ".cache/coverart";
    let hash = md5::compute(&uri);
    let filename = format!("{:x}.png", hash);
    let path = format!("{}/{}", base, filename);
    if Path::new(&path).exists() {
        trace!("file already exists");
        return Ok(CachedEntry { filename, uri });
    }

    debug!("{} -> {}", &uri, &filename);

    let buffer = {
        trace!("fetching image");
        let mut buffer = Vec::new();
        let mut res = get(&uri)?;
        res.read_to_end(&mut buffer)?;
        buffer
    };
    trace!("resizing image");
    let img = image::load_from_memory(&buffer)?;
    let thumb = img.resize_exact(THUMBNAIL_SIZE, THUMBNAIL_SIZE, FilterType::Nearest);
    trace!("storing image");
    let mut file = OpenOptions::new().create(true).write(true).open(&path)?;
    thumb.write_to(&mut file, image::ImageFormat::PNG)?;
    Ok(CachedEntry { filename, uri })
}

impl Cache {
    pub fn new() -> Cache {
        Cache::default()
    }

    pub fn fetch_coverart(&self, uri: String) -> Result<String, Error> {
        trace!("fetch_coverart (uri: {})", &uri);
        {
            let map = self.coverart.read().unwrap();
            if map.contains_key(&uri) {
                return Ok(format!("/cache/coverart/{}", map.get(&uri).unwrap()));
            }
        }
        trace!("coverart not cached yet");
        let entry = cache_coverart(uri)?;
        {
            let mut map = self.coverart.write().unwrap();
            map.insert(entry.uri, entry.filename.clone());
        }
        Ok(format!("/cache/coverart/{}", entry.filename))
    }
}
