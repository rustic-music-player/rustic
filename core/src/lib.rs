use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};

use crossbeam_channel as channel;
use failure::format_err;
use log::{debug, info, trace};
use url::Url;

use crate::extension::HostedExtension;
pub use crate::library::{
    Album, Artist, Library, LibraryQueryJoins, MultiQuery, Playlist, QueryJoins, SearchResults,
    SharedLibrary, SingleQuery, SingleQueryIdentifier, Track,
};
pub use crate::player::{PlayerBackend, PlayerEvent, PlayerState};
pub use crate::provider::{Explorer, Provider};
use crate::provider::SharedProvider;

pub mod cache;
pub mod extension;
pub mod library;
pub mod player;
pub mod provider;
pub mod sync;

pub struct Rustic {
    player: Arc<Mutex<HashMap<String, Arc<Box<dyn PlayerBackend>>>>>,
    pub library: library::SharedLibrary,
    pub providers: provider::SharedProviders,
    pub cache: cache::SharedCache,
    pub extensions: Vec<HostedExtension>,
    default_player: Arc<Mutex<Option<String>>>,
    keep_running: Arc<(Mutex<bool>, Condvar)>,
}

impl Rustic {
    pub fn new(
        library: Box<dyn Library>,
        providers: provider::SharedProviders,
        extensions: Vec<HostedExtension>,
    ) -> Result<Arc<Rustic>, failure::Error> {
        let library = Arc::new(library);
        Ok(Arc::new(Rustic {
            player: Arc::new(Mutex::new(HashMap::new())),
            library,
            providers,
            extensions,
            cache: Arc::new(cache::Cache::new()),
            default_player: Arc::new(Mutex::new(None)),
            keep_running: Arc::new((Mutex::new(true), Condvar::new())),
        }))
    }

    pub fn add_player(&self, id: String, backend: Arc<Box<dyn PlayerBackend>>) {
        debug!("Adding player {}: {:?}", id, backend);
        let mut player = self.player.lock().unwrap();
        player.insert(id, backend);
    }

    pub fn get_player(&self, id: String) -> Option<Arc<Box<dyn PlayerBackend>>> {
        let player = self.player.lock().unwrap();
        player.get(&id).map(Arc::clone)
    }

    pub fn get_default_player(&self) -> Option<Arc<Box<dyn PlayerBackend>>> {
        let default_player = self.default_player.lock().unwrap();
        default_player.as_ref().and_then(|id| {
            let player = self.player.lock().unwrap();

            player.get(id).map(Arc::clone)
        })
    }

    pub fn get_default_player_id(&self) -> Option<String> {
        self.default_player.lock().unwrap().clone()
    }

    pub fn set_default_player(&self, id: String) {
        let mut default_player = self.default_player.lock().unwrap();
        *default_player = Some(id);
    }

    pub fn query_track(&self, query: SingleQuery) -> Result<Option<Track>, failure::Error> {
        debug!("Executing track query: {:?}", query);
        let track = self.library.query_track(query.clone())?;
        if let Some(track) = track {
            Ok(Some(track))
        }else {
            if let SingleQueryIdentifier::Uri(ref uri) = query.identifier {
                trace!("Track is not in library, asking provider");
                let provider = self.get_provider(uri)?;
                let track = match provider {
                    Some(provider) => provider.read().unwrap().resolve_track(uri)?,
                    _ => None,
                };
                Ok(track)
            }else {
                // Only library tracks have an id
                Ok(None)
            }
        }
    }

    fn get_provider(&self, uri: &str) -> Result<Option<&SharedProvider>, failure::Error> {
        trace!("get_provider for {}", uri);
        let url = dbg!(Url::parse(uri))?;
        let provider = self
            .providers
            .iter()
            .find(|provider| provider.read().unwrap().uri_scheme() == url.scheme());
        Ok(provider)
    }

    pub fn stream_url(&self, track: &Track) -> Result<String, failure::Error> {
        self.providers
            .iter()
            .find(|provider| provider.read().unwrap().provider() == track.provider)
            .ok_or_else(|| format_err!("provider for track {:?} not found", track))
            .and_then(|provider| provider.read().unwrap().stream_url(track))
    }

    pub fn exit(&self) {
        let interrupt = Arc::clone(&self.keep_running);
        info!("Shutting down");
        let &(ref lock, ref cvar) = &*interrupt;
        let mut running = lock.lock().unwrap();
        *running = false;
        cvar.notify_all();
    }

    pub fn running(&self) -> Arc<(Mutex<bool>, Condvar)> {
        Arc::clone(&self.keep_running)
    }
}
