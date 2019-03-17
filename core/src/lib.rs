use failure::format_err;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crossbeam_channel as channel;
use url::Url;

pub use crate::library::{Album, Artist, Library, Playlist, SearchResults, SharedLibrary, Track};
pub use crate::player::{PlayerBackend, PlayerEvent, PlayerState};
pub use crate::provider::{Explorer, Provider};

pub mod cache;
pub mod library;
pub mod player;
pub mod provider;
pub mod sync;

pub struct Rustic {
    player: Arc<Mutex<HashMap<String, Arc<Box<dyn PlayerBackend>>>>>,
    pub library: library::SharedLibrary,
    pub providers: provider::SharedProviders,
    pub cache: cache::SharedCache,
    default_player: Arc<Mutex<Option<String>>>,
}

impl Rustic {
    pub fn new(
        library: Box<dyn Library>,
        providers: provider::SharedProviders,
    ) -> Result<Arc<Rustic>, failure::Error> {
        let library = Arc::new(library);
        Ok(Arc::new(Rustic {
            player: Arc::new(Mutex::new(HashMap::new())),
            library,
            providers,
            cache: Arc::new(cache::Cache::new()),
            default_player: Arc::new(Mutex::new(None)),
        }))
    }

    pub fn add_player(&self, id: String, backend: Arc<Box<dyn PlayerBackend>>) {
        let mut player = self.player.lock().unwrap();
        player.insert(id, backend);
    }

    pub fn get_player(&self, id: String) -> Option<Arc<Box<dyn PlayerBackend>>> {
        let player = self.player.lock().unwrap();
        player.get(&id).map(Arc::clone)
    }

    pub fn get_default_player(&self) -> Option<Arc<Box<dyn PlayerBackend>>> {
        let default_player = self.default_player.lock().unwrap();
        default_player.clone().and_then(|id| {
            let player = self.player.lock().unwrap();

            player.get(&id).map(Arc::clone)
        })
    }

    pub fn set_default_player(&self, id: String) {
        let mut default_player = self.default_player.lock().unwrap();
        *default_player = Some(id);
    }

    pub fn resolve_track(&self, uri: &str) -> Result<Option<Track>, failure::Error> {
        let track = self
            .library
            .get_tracks()?
            .into_iter()
            .find(|track| track.uri == uri);

        match track {
            Some(track) => Ok(Some(track)),
            None => {
                let url = Url::parse(uri)?;
                let provider = self
                    .providers
                    .iter()
                    .find(|provider| provider.read().unwrap().uri_scheme() == url.scheme());
                let track = match provider {
                    Some(provider) => provider.read().unwrap().resolve_track(uri)?,
                    _ => None,
                };
                Ok(track)
            }
        }
    }

    pub fn stream_url(&self, track: &Track) -> Result<String, failure::Error> {
        self.providers.iter()
            .find(|provider| provider.read().unwrap().provider() == track.provider)
            .ok_or_else(|| format_err!("provider for track {:?} not found", track))
            .and_then(|provider| provider.read().unwrap().stream_url(track))
    }
}
