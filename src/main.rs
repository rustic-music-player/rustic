use std::sync::Arc;
use std::thread::JoinHandle;

use failure::Error;
use log::{error, LevelFilter, trace};
use structopt::StructOpt;

use rustic_api::ApiClient;
use rustic_core::Rustic;
#[cfg(feature = "google-cast-backend")]
use rustic_google_cast_backend::GoogleCastBackend;
use rustic_memory_store::MemoryLibrary;

use crate::config::*;
use crate::setup::*;

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

    if is_remote(&options, &config) {
        connect_to_instance(options, config)
    } else {
        run_instance(options, config)
    }
}

fn is_remote(options: &options::CliOptions, config: &config::Config) -> bool {
    if options.connect.is_some() {
        true
    } else if let ClientConfig::Http { url: _ } = config.client {
        true
    } else {
        false
    }
}

fn run_instance(options: options::CliOptions, config: config::Config) -> Result<(), failure::Error> {
    let extensions = load_extensions(&options, &config)?;
    let providers = setup_providers(&config)?;

    let store: Box<dyn rustic_core::Library> = match config.library {
        LibraryConfig::Memory => Box::new(MemoryLibrary::new()),
        #[cfg(feature = "sqlite-store")]
        LibraryConfig::SQLite { ref path } => {
            let library = rustic_sqlite_store::SqliteLibrary::new(path.clone())?;
            Box::new(library)
        }
        #[cfg(feature = "sled-store")]
        LibraryConfig::Sled { ref path } => {
            let library = rustic_sled_store::SledLibrary::new(path)?;
            Box::new(library)
        }
    };

    let app = Rustic::new(store, providers)?;
    let client = setup_client(&app, extensions);

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

    setup_frontends(&config, &app, &client, &mut threads);

    for handle in threads {
        let _ = handle.join();
    }

    Ok(())
}

fn connect_to_instance(options: options::CliOptions, config: config::Config) -> Result<(), failure::Error> {
    let client = setup_remote_client(&options, &config.client);

    // TOOD: allow support for more frontends when everything is decoupled from app instance
    #[cfg(feature = "iced-frontend")]
    rustic_iced_frontend::start(Arc::clone(&client));

    Ok(())
}

fn setup_frontends(config: &config::Config, app: &Arc<Rustic>, client: &ApiClient, threads: &mut Vec<JoinHandle<()>>) {
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
}

fn setup_interrupt(app: &Arc<Rustic>) -> Result<(), Error> {
    let app = Arc::clone(app);
    ctrlc::set_handler(move || {
        app.exit();
    })?;
    Ok(())
}
