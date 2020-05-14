use std::sync::Arc;

use log::{debug, trace};
use rayon::prelude::*;

use async_trait::async_trait;
use rustic_api::client::*;
use rustic_api::models::*;
use rustic_core::{Album, Artist, Rustic, Track};
use rustic_extension_api::ExtensionManager;

mod library;
mod queue;
mod player;
mod stream_util;

#[derive(Clone)]
pub struct RusticNativeClient {
    pub(crate) app: Arc<Rustic>,
    pub(crate) extensions: ExtensionManager,
}

impl RusticNativeClient {
    pub fn new(app: Arc<Rustic>, extensions: ExtensionManager) -> RusticNativeClient {
        RusticNativeClient {
            app,
            extensions,
        }
    }
}

#[async_trait]
impl RusticApiClient for RusticNativeClient {
    async fn search(&self, query: &str, provider_filter: Option<&Vec<ProviderType>>) -> Result<SearchResults> {
        let providers = &self.app.providers;
        trace!("search {}", query);

        let sw = stopwatch::Stopwatch::start_new();
        let results = providers
            .iter()
            .filter(|provider| {
                if let Some(provider_filter) = provider_filter {
                    let p = provider.read().unwrap().provider().into();
                    provider_filter.contains(&p)
                } else {
                    true
                }
            })
            .map(|provider| provider.read().unwrap().search(query.to_string()))
            .collect::<Result<Vec<_>>>()?;
        debug!("Searching took {}ms", sw.elapsed_ms());

        let tracks: Vec<TrackModel> = results
            .par_iter()
            .cloned()
            .flat_map(|items| items)
            .filter(|result| result.is_track())
            .map(Track::from)
            .map(TrackModel::from)
            .collect();

        let albums: Vec<AlbumModel> = results
            .par_iter()
            .cloned()
            .flat_map(|items| items)
            .filter(|result| result.is_album())
            .map(Album::from)
            .map(AlbumModel::from)
            .collect();

        let artists: Vec<ArtistModel> = results
            .par_iter()
            .cloned()
            .flat_map(|items| items)
            .filter(|result| result.is_artist())
            .map(Artist::from)
            .map(ArtistModel::from)
            .collect();

        Ok(SearchResults {
            tracks,
            albums,
            artists,
            playlists: vec![],
        })
    }

    async fn get_extensions(&self) -> Result<Vec<ExtensionModel>> {
        let extensions = self.extensions.get_extensions().into_iter().map(ExtensionModel::from).collect();

        Ok(extensions)
    }

    async fn get_providers(&self) -> Result<Vec<ProviderModel>> {
        let providers = self.app
            .providers
            .iter()
            .filter(|provider| {
                provider
                    .read()
                    .map(|provider| provider.auth_state().is_authenticated())
                    .unwrap_or(false)
            })
            .map(|provider| {
                let provider = provider.read().unwrap();
                let title = provider.title().to_owned();
                let provider_type = provider.provider();
                let explore = provider.root();

                ProviderModel {
                    title,
                    provider: provider_type.into(),
                    explore: explore.into(),
                }
            })
            .collect();

        Ok(providers)
    }

    async fn get_available_providers(&self) -> Result<Vec<AvailableProviderModel>> {
        let providers = self.app
            .providers
            .iter()
            .map(|provider| {
                let provider = provider.read().expect("can't read provider");
                let provider_type = provider.provider();
                let auth_state = provider.auth_state();

                AvailableProviderModel {
                    provider: provider_type.into(),
                    title: provider.title().to_owned(),
                    enabled: true,
                    auth_state: auth_state.into(),
                }
            })
            .collect();

        Ok(providers)
    }
}

impl std::fmt::Debug for RusticNativeClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RusticNativeClient")
            .finish()
    }
}
