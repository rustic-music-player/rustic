extern crate ctrlc;
extern crate env_logger;
extern crate failure;
#[macro_use]
extern crate log;
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
#[cfg(feature = "iced")]
extern crate rustic_iced_frontend as iced_frontend;
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

use crate::config::*;
#[cfg(feature = "google-cast")]
use google_cast_backend::GoogleCastBuilder;
#[cfg(feature = "gstreamer")]
use gst_backend::GstreamerPlayerBuilder;
use crate::memory_store::MemoryLibrary;
#[cfg(feature = "rodio")]
use crate::rodio_backend::RodioPlayerBuilder;
use rustic_core::Rustic;
use rustic_core::extension::HostedExtension;
use rustic_core::player::{queue::MemoryQueueBuilder, PlayerBuilder};
#[cfg(feature = "sled-store")]
use crate::sled_store::SledLibrary;
#[cfg(feature = "sqlite-store")]
use sqlite_store::SqliteLibrary;
#[cfg(feature = "google-cast")]
use google_cast_backend::GoogleCastBackend;

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

    trace!("Config {:?}", config);

    let extensions = load_extensions(&options, &config)?;
    let providers = setup_providers(&config);

    let store: Box<dyn rustic_core::Library> = match config.library {
        LibraryConfig::Memory => Box::new(MemoryLibrary::new()),
        #[cfg(feature = "sqlite-store")]
        LibraryConfig::SQLite { path } => Box::new(SqliteLibrary::new(path)?),
        #[cfg(feature = "sled-store")]
        LibraryConfig::Sled { path } => Box::new(SledLibrary::new(path)?),
    };

    let app = Rustic::new(store, providers, extensions)?;

    setup_interrupt(&app)?;

    for player_config in config.players.iter() {
        if let Err(e) = setup_player(&app, player_config) {
            error!("Error setting up player {:?}", e);
        }
    }

    #[cfg(feature = "google-cast")]
    {
        if config.discovery.google_cast {
            GoogleCastBackend::start_discovery(Arc::clone(&app));
        }
    }

    let mut threads = vec![
        rustic_core::sync::start(Arc::clone(&app))?,
        rustic_core::cache::start(Arc::clone(&app))?,
    ];

    #[cfg(feature = "mpd")]
    {
        if config.frontend.mpd.is_some() {
            let mpd_thread = mpd_frontend::start(config.frontend.mpd.clone(), Arc::clone(&app));
            threads.push(mpd_thread);
        }
    }

    #[cfg(feature = "web-api")]
    {
        if config.frontend.http.is_some() {
            let http_thread = http_frontend::start(config.frontend.http.clone(), Arc::clone(&app));
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

    #[cfg(feature = "iced")]
    {
        iced_frontend::start(Arc::clone(&app));
    }

    for handle in threads {
        let _ = handle.join();
    }

    Ok(())
}

fn setup_player(app: &Arc<Rustic>, player_config: &PlayerBackendConfig) -> Result<(), failure::Error> {
    let name = player_config.name.clone();
    let player = match player_config.backend_type {
        #[cfg(feature = "gstreamer")]
        PlayerBackend::GStreamer => PlayerBuilder::new(Arc::clone(&app))
            .with_name(&name)
            .with_memory_queue()
            .with_gstreamer()?
            .build(),
        #[cfg(feature = "google-cast")]
        PlayerBackend::GoogleCast { ip } => PlayerBuilder::new(Arc::clone(&app))
            .with_name(&name)
            .with_memory_queue()
            .with_google_cast(ip)?
            .build(),
        #[cfg(feature = "rodio")]
        PlayerBackend::Rodio => PlayerBuilder::new(Arc::clone(&app))
            .with_name(&name)
            .with_memory_queue()
            .with_rodio()?
            .build(),
    };
    app.add_player(name.clone(), player);
    if player_config.default {
        app.set_default_player(name);
    }
    Ok(())
}

fn load_extensions(
    options: &options::CliOptions,
    config: &Config,
) -> Result<Vec<HostedExtension>, Error> {
    let mut paths = vec![
        Path::new("target/debug"),
        Path::new("target/release"),
        Path::new("extensions"),
    ];
    if let Some(ref path) = config.extensions.path {
        paths.insert(0, Path::new(path));
    }
    if let Some(ref path) = options.extensions_path {
        paths.insert(0, Path::new(path));
    }
    let path = paths.iter().find(|path| path.exists());
    if let Some(path) = path {
        let extensions = rustic_core::extension::load_extensions(path)?;
        Ok(extensions)
    } else {
        Ok(Vec::new())
    }
}

fn setup_providers(config: &Config) -> rustic_core::provider::SharedProviders {
    let mut providers: rustic_core::provider::SharedProviders = vec![];

    #[cfg(feature = "pocketcasts")]
    {
        if let Some(pocketcasts) = config.provider.pocketcasts.clone() {
            providers.push(Arc::new(RwLock::new(Box::new(pocketcasts))));
        }
    }
    #[cfg(feature = "soundcloud")]
    {
        if let Some(soundcloud) = config.provider.soundcloud.clone() {
            providers.push(Arc::new(RwLock::new(Box::new(soundcloud))));
        }
    }
    #[cfg(feature = "spotify")]
    {
        if let Some(spotify) = config.provider.spotify.clone() {
            providers.push(Arc::new(RwLock::new(Box::new(spotify))));
        }
    }
    #[cfg(feature = "local-files")]
    {
        if let Some(local) = config.provider.local.clone() {
            providers.push(Arc::new(RwLock::new(Box::new(local))));
        }
    }
    #[cfg(feature = "gmusic")]
    {
        if let Some(gmusic) = config.provider.gmusic.clone() {
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

fn setup_interrupt(app: &Arc<Rustic>) -> Result<(), Error> {
    let app = Arc::clone(app);
    ctrlc::set_handler(move || {
        app.exit();
    })?;
    Ok(())
}
