use std::sync::Arc;
use std::thread::JoinHandle;

use failure::Error;
use log::{error, trace, LevelFilter};
use structopt::StructOpt;

use rustic_api::ApiClient;
use rustic_core::{CredentialStore, Rustic};
#[cfg(feature = "google-cast-backend")]
use rustic_google_cast_backend::GoogleCastBackend;
use rustic_memory_store::MemoryLibrary;

use crate::config::*;
use crate::credential_stores::*;
use crate::setup::*;

mod config;
mod credential_stores;
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
        connect_to_instance(options, config)?;
    } else {
        run_instance(options, config)?;
    }

    Ok(())
}

fn is_remote(options: &options::CliOptions, config: &config::Config) -> bool {
    #[cfg(any(feature = "http-client"))]
    if options.connect.is_some() {
        return true;
    }
    #[cfg(feature = "http-client")]
    if let ClientConfig::Http { url: _ } = config.client {
        return true;
    }

    false
}

async fn setup_instance(
    options: &options::CliOptions,
    config: &config::Config,
) -> Result<(Arc<Rustic>, ApiClient), failure::Error> {
    let extensions = load_extensions(&options, &config)?;
    let credential_store: Box<dyn CredentialStore> = match config.credential_store {
        CredentialStoreConfig::Keychain => Box::new(KeychainCredentialStore),
        CredentialStoreConfig::File { ref path } => {
            Box::new(FileCredentialStore::load(path).await?)
        }
    };
    let providers = setup_providers(&config, credential_store.as_ref()).await?;

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
    let client = setup_client(&app, extensions, credential_store);

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

    Ok((app, client))
}

fn run_instance(
    options: options::CliOptions,
    config: config::Config,
) -> Result<(), failure::Error> {
    let mut rt = tokio::runtime::Runtime::new()?;
    let (app, client) = rt.block_on(setup_instance(&options, &config))?;

    let sync_app = Arc::clone(&app);

    rt.spawn(rustic_core::sync::start(sync_app));

    let mut threads = vec![];

    if let Err(e) = rt.block_on(setup_apis(&config, &app, &client, &mut threads)) {
        error!("frontend setup failed {:?}", e)
    }

    run_frontend(&config, &app, &client)?;

    for handle in threads {
        let _ = handle.join();
    }

    Ok(())
}

fn connect_to_instance(
    options: options::CliOptions,
    config: config::Config,
) -> Result<(), failure::Error> {
    let client = setup_remote_client(&options, &config.client);

    // TOOD: allow support for more frontends when everything is decoupled from app instance
    #[cfg(feature = "iced-frontend")]
    rustic_iced_frontend::start(Arc::clone(&client));

    Ok(())
}

async fn setup_apis(
    config: &config::Config,
    app: &Arc<Rustic>,
    client: &ApiClient,
    threads: &mut Vec<JoinHandle<()>>,
) -> Result<(), failure::Error> {
    #[cfg(feature = "mpd-frontend")]
    {
        if config.frontend.mpd.is_some() {
            let mpd_thread =
                rustic_mpd_frontend::start(config.frontend.mpd.clone(), Arc::clone(&app));
            threads.push(mpd_thread);
        }
    }

    #[cfg(feature = "http-frontend")]
    {
        if config.frontend.http.is_some() {
            let http_thread = rustic_http_frontend::start(
                config.frontend.http.clone(),
                Arc::clone(&app),
                Arc::clone(&client),
            );
            threads.push(http_thread);
        }
    }

    #[cfg(feature = "dbus-frontend")]
    {
        rustic_dbus_frontend::start(Arc::clone(&client)).await?;
    }

    Ok(())
}

#[allow(unused_variables)]
fn run_frontend(
    config: &config::Config,
    app: &Arc<Rustic>,
    client: &ApiClient,
) -> Result<(), failure::Error> {
    #[cfg(feature = "systray-frontend")]
    {
        rustic_systray_frontend::start()?;
    }

    #[cfg(feature = "qt-frontend")]
    {
        rustic_qt_frontend::start(Arc::clone(&client));
    }

    #[cfg(feature = "iced-frontend")]
    if config.frontend.iced.is_some() {
        rustic_iced_frontend::start(Arc::clone(&client));
    }

    Ok(())
}
