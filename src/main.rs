use std::sync::Arc;

use failure::Error;
use log::{error, trace, LevelFilter};
use structopt::StructOpt;

use crate::config::*;
use crate::setup::*;
use rustic_core::Rustic;
#[cfg(feature = "google-cast-backend")]
use rustic_google_cast_backend::GoogleCastBackend;
use rustic_memory_store::MemoryLibrary;

mod config;
mod options;
mod setup;

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
        LibraryConfig::SQLite { path } => {
            let library = rustic_sqlite_store::SqliteLibrary::new(path)?;
            Box::new(library)
        }
        #[cfg(feature = "sled-store")]
        LibraryConfig::Sled { path } => {
            let library = rustic_sled_store::SledLibrary::new(path)?;
            Box::new(library)
        }
    };

    let app = Rustic::new(store, providers, extensions)?;
    let client = setup_client(&app, &config.client);

    setup_interrupt(&app)?;

    for player_config in config.players.iter() {
        if let Err(e) = setup_player(&app, player_config) {
            error!("Error setting up player {:?}", e);
        }
    }

    #[cfg(feature = "google-cast-backend")]
    {
        if config.discovery.google_cast {
            GoogleCastBackend::start_discovery(Arc::clone(&app));
        }
    }

    rustic_core::cache::setup()?;

    let mut threads = vec![
        rustic_core::sync::start(Arc::clone(&app))?,
    ];

    #[cfg(feature = "mpd-frontend")]
    {
        if config.frontend.mpd.is_some() {
            let mpd_thread = mpd_frontend::start(config.frontend.mpd.clone(), Arc::clone(&app));
            threads.push(mpd_thread);
        }
    }

    #[cfg(feature = "http-frontend")]
    {
        if config.frontend.http.is_some() {
            let http_thread =
                rustic_http_frontend::start(config.frontend.http.clone(), Arc::clone(&app), Arc::clone(&client));
            threads.push(http_thread);
        }
    }

    #[cfg(feature = "dbus-frontend")]
    {
        let dbus_thread = dbus_frontend::start(Arc::clone(&app));
        threads.push(dbus_thread);
    }

    #[cfg(feature = "qt-frontend")]
    {
        qt_frontend::start(Arc::clone(&app));
    }

    #[cfg(feature = "iced-frontend")]
    if config.frontend.iced.is_some() {
        rustic_iced_frontend::start(Arc::clone(&client));
    }

    for handle in threads {
        let _ = handle.join();
    }

    Ok(())
}

fn setup_interrupt(app: &Arc<Rustic>) -> Result<(), Error> {
    let app = Arc::clone(app);
    ctrlc::set_handler(move || {
        app.exit();
    })?;
    Ok(())
}
