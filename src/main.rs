extern crate ctrlc;
extern crate env_logger;
extern crate failure;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
extern crate toml;

// Core
extern crate rustic_core as rustic;
// Backends
#[cfg(feature = "gstreamer")]
extern crate rustic_gstreamer_backend as gst_backend;
#[cfg(feature = "google-cast")]
extern crate rustic_google_cast_backend as google_cast_backend;
// Frontends
#[cfg(feature = "web-api")]
extern crate rustic_http_frontend as http_frontend;
#[cfg(feature = "mpd")]
extern crate rustic_mpd_frontend as mpd_frontend;
#[cfg(feature = "dbus")]
extern crate rustic_dbus_frontend as dbus_frontend;
#[cfg(feature = "qt")]
extern crate rustic_qt_frontend as qt_frontend;
// Provider
#[cfg(feature = "local-files")]
extern crate rustic_local_provider as local_provider;
#[cfg(feature = "pocketcasts")]
extern crate rustic_pocketcasts_provider as pocketcasts_provider;
#[cfg(feature = "soundcloud")]
extern crate rustic_soundcloud_provider as soundcloud_provider;
#[cfg(feature = "spotify")]
extern crate rustic_spotify_provider as spotify_provider;
// Stores
extern crate rustic_memory_store as memory_store;
#[cfg(feature = "sled-store")]
extern crate rustic_sled_store as sled_store;
#[cfg(feature = "sqlite-store")]
extern crate rustic_sqlite_store as sqlite_store;

use std::sync::{Arc, Condvar, Mutex, RwLock};

use failure::Error;
use log::LevelFilter;
use structopt::StructOpt;

use config::*;
use memory_store::MemoryLibrary;
#[cfg(feature = "sled-store")]
use sled_store::SledLibrary;
#[cfg(feature = "sqlite-store")]
use sqlite_store::SqliteLibrary;

mod config;
mod options;

fn main() -> Result<(), Error> {
    let options = options::CliOptions::from_args();
    let log_level = match options.verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace
    };

    env_logger::Builder::from_default_env()
        .filter(None, log_level)
        .init();

    let config = read_config(options.config)?;

    let providers = setup_providers(&config);

    let store: Box<rustic::Library> = match config.library.unwrap_or(LibraryConfig::Memory) {
        LibraryConfig::Memory => Box::new(MemoryLibrary::new()),
        #[cfg(feature = "sqlite-store")]
        LibraryConfig::SQLite { path } => Box::new(SqliteLibrary::new(path)?),
        #[cfg(feature = "sled-store")]
        LibraryConfig::Sled { path } => Box::new(SledLibrary::new(path)?)
    };

    let app = rustic::Rustic::new(store, providers, Vec::new())?;

    let keep_running = Arc::new((Mutex::new(true), Condvar::new()));

    let interrupt = Arc::clone(&keep_running);

    ctrlc::set_handler(move || {
        info!("Shutting down");
        let &(ref lock, ref cvar) = &*interrupt;
        let mut running = lock.lock().unwrap();
        *running = false;
        cvar.notify_all();
    })?;

    for player in config.players.iter() {
        match player.backend_type {
            #[cfg(feature = "gstreamer")]
            PlayerBackend::GStreamer => {
                let backend = gst_backend::GstBackend::new(Arc::clone(&app))?;
                app.add_player(player.name.clone(), backend);
                if player.default {
                    app.set_default_player(player.name.clone());
                }
            },
            #[cfg(feature = "google-cast")]
            PlayerBackend::GoogleCast => {
                let discovery = google_cast_backend::GoogleCastBackend::start_discovery(Arc::clone(&app), Arc::clone(&keep_running));
            },
            _ => panic!("invalid backend config"),
        }
    }

    let mut threads = vec![
        rustic::sync::start(Arc::clone(&app))?,
        rustic::cache::start(Arc::clone(&app))?,
    ];

    #[cfg(feature = "mpd")]
        {
            if config.mpd.is_some() {
                let mpd_thread = mpd_frontend::start(config.mpd.clone(), Arc::clone(&app));
                threads.push(mpd_thread);
            }
        }

    #[cfg(feature = "web-api")]
        {
            if config.http.is_some() {
                let http_thread = http_frontend::start(config.http.clone(), Arc::clone(&app));
                threads.push(http_thread);
            }
        }

    #[cfg(feature = "dbus")]
        {
            let dbus_thread = dbus_frontend::start(Arc::clone(&app));
            threads.push(dbus_thread);
        }

    #[cfg(feature = "qt")]
        {
            qt_frontend::start(Arc::clone(&app));
        }

    for handle in threads {
        let _ = handle.join();
    }

    Ok(())
}

fn setup_providers(config: &Config) -> rustic::provider::SharedProviders {
    let mut providers: rustic::provider::SharedProviders = vec![];

    #[cfg(feature = "pocketcasts")]
        {
            if let Some(pocketcasts) = config.pocketcasts.clone() {
                providers.push(Arc::new(RwLock::new(Box::new(pocketcasts))));
            }
        }
    #[cfg(feature = "soundcloud")]
        {
            if let Some(soundcloud) = config.soundcloud.clone() {
                providers.push(Arc::new(RwLock::new(Box::new(soundcloud))));
            }
        }
    #[cfg(feature = "spotify")]
        {
            if let Some(spotify) = config.spotify.clone() {
                providers.push(Arc::new(RwLock::new(Box::new(spotify))));
            }
        }
    #[cfg(feature = "local-files")]
        {
            if let Some(local) = config.local.clone() {
                providers.push(Arc::new(RwLock::new(Box::new(local))));
            }
        }
    for provider in &providers {
        let mut provider = provider.write().unwrap();
        provider.setup().unwrap_or_else(|err| {
            error!("Can't setup {} provider: {:?}", provider.title(), err)
        });
    }

    providers
}
