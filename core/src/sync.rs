use std::sync::Arc;
use std::time::Duration;

use failure::Error;
use itertools::Itertools;
use rustic_queue::{broadcast, Receiver, Sender};

use crate::{ProviderType, Rustic};

#[derive(Debug, Clone)]
pub enum SyncEvent {
    Synchronizing(Vec<SyncItem>),
    Idle,
}

#[derive(Debug, Clone)]
pub struct SyncItem {
    pub provider: ProviderType,
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
    tx: Sender<SyncEvent>,
}

impl SyncState {
    pub(crate) fn new() -> SyncState {
        let (tx, rx) = broadcast();

        SyncState { events: rx, tx }
    }

    fn next(&self, event: SyncEvent) {
        log::trace!("{:?}", event);
        self.tx.send(event).unwrap();
    }
}

const SYNC_INTERVAL: u64 = 5 * 60;

pub async fn start(app: Arc<Rustic>) -> Result<(), Error> {
    log::info!("Starting Background Sync");
    let mut interval = tokio::time::interval(Duration::from_secs(SYNC_INTERVAL));
    loop {
        interval.tick().await;
        synchronize(&app).await;
    }
}

async fn synchronize(app: &Arc<Rustic>) {
    let providers = app.providers.clone();
    let mut sync_items: Vec<SyncItem> = providers
        .iter()
        .map(|p| SyncItem {
            provider: p.provider_type,
            state: SyncItemState::Idle,
        })
        .collect();
    app.sync.next(SyncEvent::Synchronizing(sync_items.clone()));
    for provider in providers {
        let provider = provider.get().await;
        if !provider.state().is_authenticated() {
            continue;
        }
        log::info!("Syncing {} library", provider.title());
        let (position, _) = sync_items
            .iter()
            .find_position(|i| i.provider == provider.provider())
            .unwrap();
        sync_items.get_mut(position).unwrap().state = SyncItemState::Syncing;
        app.sync.next(SyncEvent::Synchronizing(sync_items.clone()));
        match provider.sync(Arc::clone(&app.library)).await {
            Ok(result) => {
                sync_items.get_mut(position).unwrap().state = SyncItemState::Done;
                log::info!(
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
                log::error!("Error syncing {}: {:?}", provider.title(), err)
            }
        }
        app.sync.next(SyncEvent::Synchronizing(sync_items.clone()));
    }
    if let Err(e) = app.library.flush() {
        log::error!("Flushing of library failed {:?}", e);
    }
    app.sync.next(SyncEvent::Idle);
}
