extern crate ctrlc;
extern crate env_logger;
extern crate failure;
#[macro_use]
extern crate log;
// Core
extern crate rustic_core as rustic;
#[cfg(feature = "dbus")]
extern crate rustic_dbus_frontend as dbus_frontend;
#[cfg(feature = "gmusic")]
extern crate rustic_gmusic_provider as gmusic_provider;
// Backends
#[cfg(feature = "google-cast")]
extern crate rustic_google_cast_backend as google_cast_backend;
#[cfg(feature = "gstreamer")]
extern crate rustic_gstreamer_backend as gst_backend;
// Frontends
#[cfg(feature = "web-api")]
extern crate rustic_http_frontend as http_frontend;
// Provider
#[cfg(feature = "local-files")]
extern crate rustic_local_provider as local_provider;
// Stores
extern crate rustic_memory_store as memory_store;
#[cfg(feature = "mpd")]
extern crate rustic_mpd_frontend as mpd_frontend;
#[cfg(feature = "pocketcasts")]
extern crate rustic_pocketcasts_provider as pocketcasts_provider;
#[cfg(feature = "qt")]
extern crate rustic_qt_frontend as qt_frontend;
#[cfg(feature = "rodio")]
extern crate rustic_rodio_backend as rodio_backend;
#[cfg(feature = "sled-store")]
extern crate rustic_sled_store as sled_store;
#[cfg(feature = "soundcloud")]
extern crate rustic_soundcloud_provider as soundcloud_provider;
#[cfg(feature = "spotify")]
extern crate rustic_spotify_provider as spotify_provider;
#[cfg(feature = "sqlite-store")]
extern crate rustic_sqlite_store as sqlite_store;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
extern crate toml;

use std::path::Path;
use std::sync::{Arc, RwLock};

use failure::Error;
use log::LevelFilter;
use structopt::StructOpt;

use config::*;
use memory_store::MemoryLibrary;
#[cfg(feature = "sled-store")]
use sled_store::SledLibrary;
#[cfg(feature = "sqlite-store")]
use sqlite_store::SqliteLibrary;
use rustic::extension::HostedExtension;

mod config;
mod options;

fn main() -> Result<(), Error> {
    let options = options::CliOptions::from_args();
    let log_level = match options.verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    env_logger::Builder::from_default_env()
        .filter(None, log_level)
        .init();

    let config = read_config(&options.config)?;

    let extensions = load_extensions(&options, &config)?;
    let providers = setup_providers(&config);

    let store: Box<dyn rustic::Library> = match config.library.unwrap_or(LibraryConfig::Memory) {
        LibraryConfig::Memory => Box::new(MemoryLibrary::new()),
        #[cfg(feature = "sqlite-store")]
        LibraryConfig::SQLite { path } => Box::new(SqliteLibrary::new(path)?),
        #[cfg(feature = "sled-store")]
        LibraryConfig::Sled { path } => Box::new(SledLibrary::new(path)?),
    };


    let app = rustic::Rustic::new(store, providers, extensions)?;

    setup_interrupt(&app)?;

    for player in config.players.iter() {
        match player.backend_type {
            #[cfg(feature = "gstreamer")]
            PlayerBackend::GStreamer => {
                let backend = gst_backend::GstBackend::new(Arc::clone(&app))?;
                app.add_player(player.name.clone(), backend);
                if player.default {
                    app.set_default_player(player.name.clone());
                }
            }
            #[cfg(feature = "google-cast")]
            PlayerBackend::GoogleCast => {
                let discovery = google_cast_backend::GoogleCastBackend::start_discovery(
                    Arc::clone(&app),
                    Arc::clone(&keep_running),
                );
            }
            #[cfg(feature = "rodio")]
            PlayerBackend::Rodio => {
                let backend = rodio_backend::RodioBackend::new(Arc::clone(&app))?;
                app.add_player(player.name.clone(), backend);
                if player.default {
                    app.set_default_player(player.name.clone());
                }
            }
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

fn load_extensions(options: &options::CliOptions, config: &Config) -> Result<Vec<HostedExtension>, Error> {
    let mut paths = vec![
        Path::new("target/debug"),
        Path::new("target/release"),
        Path::new("extensions")
    ];
    if let Some(ref path) = config.extensions.path {
        paths.insert(0, Path::new(path));
    }
    if let Some(ref path) = options.extensions_path {
        paths.insert(0, Path::new(path));
    }
    let path = paths
        .iter()
        .find(|path| path.exists());
    if let Some(path) = path {
        let extensions = rustic::extension::load_extensions(path)?;
        Ok(extensions)
    } else {
        Ok(Vec::new())
    }
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
    #[cfg(feature = "gmusic")]
    {
        if let Some(gmusic) = config.gmusic.clone() {
            providers.push(Arc::new(RwLock::new(Box::new(gmusic))));
        }
    }
    for provider in &providers {
        let mut provider = provider.write().unwrap();
        provider
            .setup()
            .unwrap_or_else(|err| error!("Can't setup {} provider: {:?}", provider.title(), err));
    }

    providers
}

fn setup_interrupt(app: &Arc<rustic::Rustic>) -> Result<(), Error> {
    let app = Arc::clone(app);
    ctrlc::set_handler(move || {
        app.exit();
    })?;
    Ok(())
}
