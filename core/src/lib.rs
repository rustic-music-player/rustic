use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};

use failure::format_err;
use log::{debug, info, trace};
use url::Url;

use crate::extension::HostedExtension;
pub use crate::library::{
    Album, Artist, Library, LibraryQueryJoins, MultiQuery, Playlist, QueryJoins, SearchResults,
    SharedLibrary, SingleQuery, SingleQueryIdentifier, Track,
};
use crate::player::Player;
pub use crate::player::{PlayerBackend, PlayerEvent, PlayerState};
use crate::provider::{CoverArt, InternalUri, SharedProvider};
pub use crate::provider::{Explorer, Provider};

pub mod cache;
pub mod extension;
pub mod library;
pub mod player;
pub mod provider;
pub mod sync;

pub struct Rustic {
    player: Arc<Mutex<HashMap<String, Arc<Player>>>>,
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

    pub fn add_player(&self, id: String, player: Arc<Player>) {
        debug!("Adding player {}: {:?}", id, player);
        let mut players = self.player.lock().unwrap();
        players.insert(id, player);
    }

    pub fn get_player(&self, id: String) -> Option<Arc<Player>> {
        let player = self.player.lock().unwrap();
        player.get(&id).map(Arc::clone)
    }

    pub fn get_default_player(&self) -> Option<Arc<Player>> {
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

    pub fn get_players(&self) -> Vec<(String, Arc<Player>)> {
        let players = self.player.lock().unwrap();
        players
            .iter()
            .map(|(id, player)| (id.clone(), Arc::clone(player)))
            .collect()
    }

    pub fn query_track(&self, query: SingleQuery) -> Result<Option<Track>, failure::Error> {
        debug!("Executing track query: {:?}", query);
        let track = self.library.query_track(query.clone())?;
        if let Some(track) = track {
            Ok(Some(track))
        } else {
            if let SingleQueryIdentifier::Uri(ref uri) = query.identifier {
                trace!("Track is not in library, asking provider");
                let provider = self.get_provider_for_url(uri)?;
                let track = match provider {
                    Some(provider) => provider.read().unwrap().resolve_track(uri)?,
                    _ => None,
                };
                Ok(track)
            } else {
                // Only library tracks have an id
                Ok(None)
            }
        }
    }

    pub fn query_album(&self, query: SingleQuery) -> Result<Option<Album>, failure::Error> {
        debug!("Executing album query: {:?}", query);
        let album = self.library.query_album(query.clone())?;
        if let Some(album) = album {
            Ok(Some(album))
        } else {
            if let SingleQueryIdentifier::Uri(ref uri) = query.identifier {
                trace!("Album is not in library, asking provider");
                let provider = self.get_provider_for_url(uri)?;
                let album = match provider {
                    Some(provider) => provider.read().unwrap().resolve_album(uri)?,
                    _ => None,
                };
                Ok(album)
            } else {
                // Only library albums have an id
                Ok(None)
            }
        }
    }

    pub fn query_playlist(&self, query: SingleQuery) -> Result<Option<Playlist>, failure::Error> {
        debug!("Executing playlist query: {:?}", query);
        let playlist = self.library.query_playlist(query.clone())?;
        if let Some(playlist) = playlist {
            Ok(Some(playlist))
        } else {
            if let SingleQueryIdentifier::Uri(ref uri) = query.identifier {
                trace!("Playlist is not in library, asking provider");
                let provider = self.get_provider_for_url(uri)?;
                let playlist = match provider {
                    Some(provider) => provider.read().unwrap().resolve_playlist(uri)?,
                    _ => None,
                };
                Ok(playlist)
            } else {
                // Only library playlists have an id
                Ok(None)
            }
        }
    }

    fn get_provider_for_url(&self, uri: &str) -> Result<Option<&SharedProvider>, failure::Error> {
        trace!("get_provider for {}", uri);
        let url = Url::parse(uri)?;
        let provider = self
            .providers
            .iter()
            .find(|provider| provider.read().unwrap().uri_scheme() == url.scheme());
        Ok(provider)
    }

    pub fn stream_url(&self, track: &Track) -> Result<String, failure::Error> {
        let provider = self.get_provider(track)?;
        let stream_url = provider.read().unwrap().stream_url(track)?;

        Ok(stream_url)
    }

    pub fn cover_art(&self, track: &Track) -> Result<Option<CoverArt>, failure::Error> {
        let provider = self.get_provider(track)?;
        let cover = provider.read().unwrap().cover_art(track)?;

        Ok(cover)
    }

    fn get_provider(&self, track: &Track) -> Result<&SharedProvider, failure::Error> {
        let provider = self
            .providers
            .iter()
            .find(|p| p.read().unwrap().provider() == track.provider)
            .ok_or_else(|| format_err!("provider for track {:?} not found", track))?;

        Ok(provider)
    }

    pub fn resolve_share_url(&self, url: String) -> Result<Option<InternalUri>, failure::Error> {
        trace!("resolving share url {}", url);
        let url = Url::parse(&url)?;
        for provider in self.providers.iter() {
            let url = url.clone();
            let provider = provider.read().unwrap();
            let uri = provider.resolve_share_url(url)?;

            if uri.is_some() {
                return Ok(uri);
            }
        }
        Ok(None)
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
