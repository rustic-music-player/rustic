extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate failure;
extern crate toml;
#[macro_use]
extern crate log;
extern crate ctrlc;
extern crate env_logger;

// Core
extern crate rustic_core as rustic;

// Frontends
extern crate rustic_http_frontend as http_frontend;
extern crate rustic_mpd_frontend as mpd_frontend;

// Stores
extern crate rustic_memory_store as memory_store;
extern crate rustic_sqlite_store as sqlite_store;

// Backends
extern crate rustic_gstreamer_backend as gst_backend;

// Provider
extern crate rustic_local_provider as local_provider;
extern crate rustic_pocketcasts_provider as pocketcasts_provider;
extern crate rustic_soundcloud_provider as soundcloud_provider;
extern crate rustic_spotify_provider as spotify_provider;

mod config;

use failure::Error;
use memory_store::MemoryLibrary;
use sqlite_store::SqliteLibrary;
use std::sync::{Arc, Condvar, Mutex, RwLock};

use config::*;

fn main() -> Result<(), Error> {
    env_logger::init();

    let config = read_config();

    let providers = setup_providers(&config);

    let store: Box<rustic::Library> = match config.library.unwrap_or(LibraryConfig::Memory) {
        LibraryConfig::Memory => Box::new(MemoryLibrary::new()),
        LibraryConfig::SQLite { path } => Box::new(SqliteLibrary::new(path)?),
    };

    let app = rustic::Rustic::new(store, providers)?;

    for player in config.players.iter() {
        let backend = match player.backend_type {
            PlayerBackend::GStreamer => gst_backend::GstBackend::new(Arc::clone(&app))?,
            _ => panic!("invalid backend config"),
        };
        app.add_player(player.name.clone(), backend);
        if player.default {
            app.set_default_player(player.name.clone());
        }
    }

    let keep_running = Arc::new((Mutex::new(true), Condvar::new()));

    let interrupt = Arc::clone(&keep_running);

    ctrlc::set_handler(move || {
        info!("Shutting down");
        let &(ref lock, ref cvar) = &*interrupt;
        let mut running = lock.lock().unwrap();
        *running = false;
        cvar.notify_all();
    })?;

    let mut threads = vec![
        rustic::sync::start(Arc::clone(&app), Arc::clone(&keep_running))?,
        rustic::cache::start(Arc::clone(&app), Arc::clone(&keep_running))?,
    ];

    if config.mpd.is_some() {
        let mpd_thread = mpd_frontend::start(config.mpd.clone(), Arc::clone(&app));
        threads.push(mpd_thread);
    }

    if config.http.is_some() {
        let http_thread = http_frontend::start(config.http.clone(), Arc::clone(&app));
        threads.push(http_thread);
    }

    for handle in threads {
        let _ = handle.join();
    }

    Ok(())
}

fn setup_providers(config: &Config) -> rustic::provider::SharedProviders {
    let mut providers: rustic::provider::SharedProviders = vec![];

    if let Some(pocketcasts) = config.pocketcasts.clone() {
        providers.push(Arc::new(RwLock::new(Box::new(pocketcasts))));
    }
    if let Some(soundcloud) = config.soundcloud.clone() {
        providers.push(Arc::new(RwLock::new(Box::new(soundcloud))));
    }
    if let Some(spotify) = config.spotify.clone() {
        providers.push(Arc::new(RwLock::new(Box::new(spotify))));
    }
    if let Some(local) = config.local.clone() {
        providers.push(Arc::new(RwLock::new(Box::new(local))));
    }
    for provider in &providers {
        let mut provider = provider.write().unwrap();
        provider.setup().unwrap_or_else(|err| {
            error!("Can't setup {} provider: {:?}", provider.title(), err)
        });
    }

    providers
}
