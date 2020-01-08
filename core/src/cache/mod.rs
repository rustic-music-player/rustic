use std::collections::HashMap;
use std::fs::{create_dir_all, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use failure::{bail, Error};
use image;
use image::FilterType;
use log::{debug, error, info, trace};
use md5;
use pinboard::NonEmptyPinboard;
use reqwest::{Client, get};

use crate::{MultiQuery, Rustic, Track};

const THUMBNAIL_SIZE: u32 = 512;
const SERVICE_INTERVAL: u64 = 30;

#[derive(Debug)]
struct CachedEntry {
    uri: String,
    filename: String,
}

#[derive(Debug)]
pub struct Cache {
    pub coverart: Arc<RwLock<HashMap<String, String>>>,
    pub tracks: Arc<NonEmptyPinboard<HashMap<String, String>>>,
}

pub type SharedCache = Arc<Cache>;

fn coverart_cache(app: &Arc<Rustic>) -> Result<Vec<Result<CachedEntry, Error>>, Error> {
    let mut tracks: Vec<Track> = app.library.query_tracks(MultiQuery::new())?;
    let mut playlist_tracks: Vec<Track> = app
        .library
        .query_playlists(MultiQuery::new())?
        .iter()
        .flat_map(|playlist| playlist.tracks.clone())
        .collect();

    tracks.append(&mut playlist_tracks);

    unimplemented!("TODO: implement new cache")
}

pub fn start(app: Arc<Rustic>) -> Result<thread::JoinHandle<()>, Error> {
    create_dir_all(".cache/music")?;
    create_dir_all(".cache/coverart")?;

    Ok(thread::spawn(|| {}))
}

fn _start_cache_thread(app: Arc<Rustic>) -> Result<thread::JoinHandle<()>, Error> {
    thread::Builder::new()
        .name("Coverart Cache".into())
        .spawn(move || {
            info!("Starting Coverart Cache");
            let &(ref lock, ref cvar) = &*app.running();
            let mut keep_running = lock.lock().unwrap();
            while *keep_running {
                info!("Caching Coverart...");
                match coverart_cache(&app) {
                    Ok(result) => {
                        let entries: Vec<&CachedEntry> = result
                            .iter()
                            .filter_map(|entry| entry.as_ref().ok())
                            .collect();
                        let errors: Vec<&Error> = result
                            .iter()
                            .filter_map(|entry| entry.as_ref().err())
                            .collect();
                        info!("Cached {} images", entries.len());
                        let mut map = app.cache.coverart.write().unwrap();
                        for entry in entries {
                            map.insert(entry.uri.clone(), entry.filename.clone());
                        }

                        if errors.len() > 0 {
                            error!("{} Errors while caching images", errors.len());
                            for error in errors {
                                error!("{}", error)
                            }
                        }
                    }
                    Err(err) => error!("{}", err),
                }

                let result = cvar
                    .wait_timeout(keep_running, Duration::new(SERVICE_INTERVAL, 0))
                    .unwrap();
                keep_running = result.0;
            }
            info!("Coverart Cache stopped");
        })
        .map_err(Error::from)
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            coverart: Default::default(),
            tracks: Arc::new(NonEmptyPinboard::new(HashMap::new())),
        }
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
        bail!("skipping for now");
        let entry = cache_coverart(uri)?;
        {
            let mut map = self.coverart.write().unwrap();
            map.insert(entry.uri, entry.filename.clone());
        }
        Ok(format!("/cache/coverart/{}", entry.filename))
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
        let client = Client::builder().timeout(Duration::from_secs(5)).build()?;
        let mut res = client.get(&uri).send()?;
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
