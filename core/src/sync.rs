use log::{error, info};
use failure::Error;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use crate::Rustic;

pub fn start(
    app: Arc<Rustic>,
    running: Arc<(Mutex<bool>, Condvar)>,
) -> Result<thread::JoinHandle<()>, Error> {
    thread::Builder::new()
        .name("Background Sync".into())
        .spawn(move || {
            info!("Starting Background Sync");
            let &(ref lock, ref cvar) = &*running;
            let mut keep_running = lock.lock().unwrap();
            while *keep_running {
                let providers = app.providers.clone();
                for provider in providers {
                    let mut provider = provider.write().unwrap();
                    info!("Syncing {} library", provider.title());
                    match provider.sync(Arc::clone(&app.library)) {
                        Ok(result) => info!(
                            "Synced {} tracks, {} albums, {} artist and {} playlists from {}",
                            result.tracks,
                            result.albums,
                            result.artists,
                            result.playlists,
                            provider.title()
                        ),
                        Err(err) => error!("Error syncing {}: {:?}", provider.title(), err),
                    }
                }
                let result = cvar
                    .wait_timeout(keep_running, Duration::from_secs(5 * 60))
                    .unwrap();
                keep_running = result.0;
            }
            info!("Background Sync stopped");
        }).map_err(Error::from)
}
