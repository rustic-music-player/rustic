use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use failure::format_err;
use log::{debug, trace};
use url::Url;

pub use crate::library::{
    Album, Artist, Library, LibraryQueryJoins, MultiQuery, Playlist, QueryJoins, SearchResults,
    SharedLibrary, SingleQuery, SingleQueryIdentifier, Track,
};
use crate::player::Player;
pub use crate::player::{PlayerBackend, PlayerEvent, PlayerState};
use crate::provider::{CoverArt, InternalUri};
pub use crate::provider::{Explorer, Provider, ProviderType};
pub use crate::cred_store::{CredentialStore, Credentials};

pub mod cache;
mod cred_store;
pub mod library;
pub mod player;
pub mod provider;
pub mod sync;

pub struct Rustic {
    player: Arc<Mutex<HashMap<String, Arc<Player>>>>,
    pub library: library::SharedLibrary,
    pub providers: Vec<Provider>,
    pub cache: cache::SharedCache,
    default_player: Arc<Mutex<Option<String>>>,
    pub sync: sync::SyncState,
}

impl Rustic {
    pub fn new(
        library: Box<dyn Library>,
        providers: Vec<Provider>,
    ) -> Result<Arc<Rustic>, failure::Error> {
        let library = Arc::new(library);
        Ok(Arc::new(Rustic {
            player: Arc::new(Mutex::new(HashMap::new())),
            library,
            providers,
            cache: Arc::new(cache::Cache::new()),
            default_player: Arc::new(Mutex::new(None)),
            sync: sync::SyncState::new(),
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

    pub async fn query_track(&self, query: SingleQuery) -> Result<Option<Track>, failure::Error> {
        debug!("Executing track query: {:?}", query);
        let track = self.library.query_track(query.clone())?;
        if let Some(track) = track {
            Ok(Some(track))
        } else if let SingleQueryIdentifier::Uri(ref uri) = query.identifier {
            trace!("Track is not in library, asking provider");
            let provider = self.get_provider_for_url(uri)?;
            let track = match provider {
                Some(provider) => provider.get().await.resolve_track(uri).await?,
                _ => None,
            };
            Ok(track)
        } else {
            // Only library tracks have an id
            Ok(None)
        }
    }

    pub async fn query_album(&self, query: SingleQuery) -> Result<Option<Album>, failure::Error> {
        debug!("Executing album query: {:?}", query);
        let album = self.library.query_album(query.clone())?;
        if let Some(album) = album {
            Ok(Some(album))
        } else if let SingleQueryIdentifier::Uri(ref uri) = query.identifier {
            trace!("Album is not in library, asking provider");
            let provider = self.get_provider_for_url(uri)?;
            let album = match provider {
                Some(provider) => provider.get().await.resolve_album(uri).await?,
                _ => None,
            };
            Ok(album)
        } else {
            // Only library albums have an id
            Ok(None)
        }
    }

    pub async fn query_artist(&self, query: SingleQuery) -> Result<Option<Artist>, failure::Error> {
        debug!("Executing artist query: {:?}", query);
        let artist = self.library.query_artist(query.clone())?;
        if let Some(artist) = artist {
            Ok(Some(artist))
        } else if let SingleQueryIdentifier::Uri(ref uri) = query.identifier {
            trace!("Artist is not in library, asking provider");
            let provider = self.get_provider_for_url(uri)?;
            let artist = match provider {
                Some(provider) => provider.get().await.resolve_artist(uri).await?,
                _ => None,
            };
            Ok(artist)
        } else {
            // Only library artists have an id
            Ok(None)
        }
    }
    
    pub async fn query_playlist(
        &self,
        query: SingleQuery,
    ) -> Result<Option<Playlist>, failure::Error> {
        debug!("Executing playlist query: {:?}", query);
        let playlist = self.library.query_playlist(query.clone())?;
        if let Some(playlist) = playlist {
            Ok(Some(playlist))
        } else if let SingleQueryIdentifier::Uri(ref uri) = query.identifier {
            trace!("Playlist is not in library, asking provider");
            let provider = self.get_provider_for_url(uri)?;
            let playlist = match provider {
                Some(provider) => provider.get().await.resolve_playlist(uri).await?,
                _ => None,
            };
            Ok(playlist)
        } else {
            // Only library playlists have an id
            Ok(None)
        }
    }

    fn get_provider_for_url(&self, uri: &str) -> Result<Option<&Provider>, failure::Error> {
        trace!("get_provider for {}", uri);
        let url = Url::parse(uri)?;
        let provider = self
            .providers
            .iter()
            .find(|provider| provider.uri_scheme == url.scheme());
        Ok(provider)
    }

    pub fn stream_url(&self, track: &Track) -> Result<String, failure::Error> {
        let provider = self.get_provider(track)?;
        // TODO: we should await instead of blocking
        let mut rt = tokio::runtime::Runtime::new()?;
        let stream_url = rt.block_on(async { provider.get().await.stream_url(track).await })?;
        debug!(
            "getting stream url for track {} => {}",
            track.uri, &stream_url
        );

        Ok(stream_url)
    }

    pub async fn cover_art(&self, track: &Track) -> Result<Option<CoverArt>, failure::Error> {
        let provider = self.get_provider(track)?;
        let cover = provider.get().await.cover_art(track).await?;
        let cover = if let Some(cover) = cover {
            let cached_cover = self.cache.fetch_coverart(&cover).await?;

            if cached_cover.is_some() {
                cached_cover
            }else {
                let cover = self.cache.cache_coverart(&cover).await?;
                Some(cover)
            }
        }else {
            None
        };

        Ok(cover)
    }

    fn get_provider(&self, track: &Track) -> Result<&Provider, failure::Error> {
        let provider = self
            .providers
            .iter()
            .find(|p| p.provider_type == track.provider)
            .ok_or_else(|| format_err!("provider for track {:?} not found", track))?;

        Ok(provider)
    }

    pub async fn resolve_share_url(
        &self,
        url: String,
    ) -> Result<Option<InternalUri>, failure::Error> {
        trace!("resolving share url {}", url);
        let url = Url::parse(&url)?;
        for provider in self.providers.iter() {
            let url = url.clone();
            let provider = provider.get().await;
            let uri = provider.resolve_share_url(url).await?;

            if uri.is_some() {
                return Ok(uri);
            }
        }
        Ok(None)
    }
}
