use crate::Rustic;
use failure::Error;
use log::{error, info};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn start(app: Arc<Rustic>) -> Result<thread::JoinHandle<()>, Error> {
    thread::Builder::new()
        .name("Background Sync".into())
        .spawn(move || {
            info!("Starting Background Sync");
            let &(ref lock, ref cvar) = &*app.running();
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
        })
        .map_err(Error::from)
}
