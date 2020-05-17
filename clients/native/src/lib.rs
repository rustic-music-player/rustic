use std::sync::Arc;

use log::{debug, trace};
use rayon::prelude::*;

use async_trait::async_trait;
use rustic_api::client::*;
use rustic_api::cursor::from_cursor;
use rustic_api::models::*;
use rustic_core::{Album, Artist, Rustic, SingleQuery, Track};
use rustic_extension_api::ExtensionManager;

mod library;
mod player;
mod provider;
mod queue;
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
        let mut rt = tokio::runtime::Runtime::new()?;
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
            .map(|provider| {
                let provider = provider.read().unwrap();
                // TODO: we should await all futures instead of blocking each one
                rt.block_on(provider.search(query.to_string()))
            })
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

    async fn open_share_url(&self, url: &str) -> Result<Option<OpenResultModel>> {
        let internal_url = self.app.resolve_share_url(url.to_owned())?;
        let result = internal_url.map(OpenResultModel::from);

        Ok(result)
    }

    async fn get_track_cover_art(&self, cursor: &str) -> Result<Option<CoverArtModel>> {
        let uri = from_cursor(cursor)?;
        let query = SingleQuery::uri(uri);
        let track = self.app.query_track(query)?;

        if let Some(track) = track {
            let cover_art = self.app.cover_art(&track)?;
            let cover_art = cover_art.map(CoverArtModel::from);

            Ok(cover_art)
        }else {
            Ok(None)
        }
    }
}

impl std::fmt::Debug for RusticNativeClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RusticNativeClient")
            .finish()
    }
}
