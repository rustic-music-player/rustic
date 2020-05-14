use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender, TryRecvError, unbounded};
use failure::Error;
use futures::Stream;
use itertools::Itertools;
use log::{error, info, trace};

use crate::{Provider, Rustic};

#[derive(Debug, Clone)]
pub enum SyncEvent {
    Synchronizing(Vec<SyncItem>),
    Idle,
}

#[derive(Debug, Clone)]
pub struct SyncItem {
    pub provider: Provider,
    pub state: SyncItemState,
}

#[derive(Debug, Clone)]
pub enum SyncItemState {
    Idle,
    Syncing,
    Done,
    Error,
}

#[derive(Debug, Clone)]
pub struct SyncState {
    pub events: Receiver<SyncEvent>,
    tx: Sender<SyncEvent>
}

impl SyncState {
    pub(crate) fn new() -> SyncState {
        let (tx, rx) = unbounded();

        SyncState {
            events: rx,
            tx
        }
    }

    fn next(&self, event: SyncEvent) {
        trace!("{:?}", event);
        self.tx.send(event).unwrap();
    }
}

pub fn start(app: Arc<Rustic>) -> Result<thread::JoinHandle<()>, Error> {
    thread::Builder::new()
        .name("Background Sync".into())
        .spawn(move || {
            info!("Starting Background Sync");
            let &(ref lock, ref cvar) = &*app.running();
            let mut keep_running = lock.lock().unwrap();
            while *keep_running {
                let providers = app.providers.clone();
                let mut sync_items: Vec<SyncItem> = providers.iter().map(|p| SyncItem {
                    provider: p.read().unwrap().provider(),
                    state: SyncItemState::Idle,
                }).collect();
                app.sync.next(SyncEvent::Synchronizing(sync_items.clone()));
                for provider in providers {
                    let provider = provider.read().unwrap();
                    if !provider.auth_state().is_authenticated() {
                        continue;
                    }
                    info!("Syncing {} library", provider.title());
                    let (position, _) = sync_items.iter().find_position(|i| i.provider == provider.provider()).unwrap();
                    sync_items.get_mut(position).unwrap().state = SyncItemState::Syncing;
                    app.sync.next(SyncEvent::Synchronizing(sync_items.clone()));
                    match provider.sync(Arc::clone(&app.library)) {
                        Ok(result) => {
                            sync_items.get_mut(position).unwrap().state = SyncItemState::Done;
                            info!(
                                "Synced {} tracks, {} albums, {} artist and {} playlists from {}",
                                result.tracks,
                                result.albums,
                                result.artists,
                                result.playlists,
                                provider.title()
                            )
                        }
                        Err(err) => {
                            sync_items.get_mut(position).unwrap().state = SyncItemState::Error;
                            error!("Error syncing {}: {:?}", provider.title(), err)
                        }
                    }
                    app.sync.next(SyncEvent::Synchronizing(sync_items.clone()));
                }
                app.sync.next(SyncEvent::Idle);
                let result = cvar
                    .wait_timeout(keep_running, Duration::from_secs(5 * 60))
                    .unwrap();
                keep_running = result.0;
            }
            info!("Background Sync stopped");
        })
        .map_err(Error::from)
}
