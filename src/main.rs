use std::path::Path;
use std::sync::{Arc, RwLock};

use failure::Error;
use log::{error, trace, LevelFilter};
use structopt::StructOpt;

use crate::config::*;
use rustic_core::extension::HostedExtension;
use rustic_core::player::{queue::MemoryQueueBuilder, PlayerBuilder};
use rustic_core::Rustic;
#[cfg(feature = "google-cast")]
use rustic_google_cast_backend::GoogleCastBackend;
#[cfg(feature = "google-cast")]
use rustic_google_cast_backend::GoogleCastBuilder;
#[cfg(feature = "gstreamer")]
use rustic_gstreamer_backend::GstreamerPlayerBuilder;
use rustic_memory_store::MemoryLibrary;
#[cfg(feature = "rodio")]
use rustic_rodio_backend::RodioPlayerBuilder;
use rustic_api::RusticApiClient;

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
    let client = rustic_native_client::RusticNativeClient::new(Arc::clone(&app));
    let client: Box<dyn RusticApiClient> = Box::new(client);
    let client = Arc::new(client);

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

    rustic_core::cache::setup()?;

    let mut threads = vec![
        rustic_core::sync::start(Arc::clone(&app))?,
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
            let http_thread =
                rustic_http_frontend::start(config.frontend.http.clone(), Arc::clone(&app), Arc::clone(&client));
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
    if config.frontend.iced.is_some() {
        rustic_iced_frontend::start(Arc::clone(&client));
    }

    for handle in threads {
        let _ = handle.join();
    }

    Ok(())
}

fn setup_player(
    app: &Arc<Rustic>,
    player_config: &PlayerBackendConfig,
) -> Result<(), failure::Error> {
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
