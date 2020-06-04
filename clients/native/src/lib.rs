use std::sync::Arc;

use log::{debug, trace, error};
use rayon::prelude::*;

use async_trait::async_trait;
use rustic_api::client::*;
use rustic_api::cursor::Cursor;
use rustic_api::models::*;
use rustic_core::provider::{ProviderItem, InternalUri, ProviderItemType};
use rustic_core::{Album, Artist, Provider, Playlist, Rustic, SingleQuery, Track, CredentialStore};
use rustic_extension_api::ExtensionManager;
use std::convert::TryInto;

mod library;
mod player;
mod provider;
mod queue;
mod stream_util;

#[derive(Clone)]
pub struct RusticNativeClient {
    pub(crate) app: Arc<Rustic>,
    pub(crate) extensions: ExtensionManager,
    pub(crate) credential_store: Arc<Box<dyn CredentialStore>>,
}

impl RusticNativeClient {
    pub fn new(app: Arc<Rustic>, extensions: ExtensionManager, cred_store: Box<dyn CredentialStore>) -> RusticNativeClient {
        RusticNativeClient { app, extensions, credential_store: Arc::new(cred_store) }
    }
}

#[async_trait]
impl RusticApiClient for RusticNativeClient {
    async fn search(
        &self,
        query: &str,
        provider_filter: Option<Vec<ProviderTypeModel>>,
    ) -> Result<SearchResults> {
        let providers = &self.app.providers;
        trace!("search {}", query);

        let sw = stopwatch::Stopwatch::start_new();
        let providers: Vec<&Provider> = providers
            .iter()
            .filter(|provider| {
                if let Some(ref provider_filter) = provider_filter {
                    let p = provider.provider_type.into();
                    provider_filter.contains(&p)
                } else {
                    true
                }
            })
            .collect();
        let mut results: Vec<ProviderItem> = Vec::new();
        // TODO: run in parallel
        for provider in providers {
            let provider = provider.get().await;
            match provider.search(query.to_string()).await {
                Ok(mut result) => {
                    results.append(&mut result);
                }
                Err(e) => error!("Searching failed for provider {:?}: {:?}", provider.provider(), e)
            }
        }
        debug!("Searching took {}ms", sw.elapsed_ms());

        let tracks: Vec<TrackModel> = results
            .par_iter()
            .cloned()
            .filter(|result| result.is_track())
            .map(Track::from)
            .map(TrackModel::from)
            .collect();

        let albums: Vec<AlbumModel> = results
            .par_iter()
            .cloned()
            .filter(|result| result.is_album())
            .map(Album::from)
            .map(AlbumModel::from)
            .collect();

        let artists: Vec<ArtistModel> = results
            .par_iter()
            .cloned()
            .filter(|result| result.is_artist())
            .map(Artist::from)
            .map(ArtistModel::from)
            .collect();

        let playlists: Vec<PlaylistModel> = results
            .par_iter()
            .cloned()
            .filter(|result| result.is_playlist())
            .map(Playlist::from)
            .map(PlaylistModel::from)
            .collect();

        Ok(SearchResults {
            tracks,
            albums,
            artists,
            playlists,
        })
    }

    async fn get_extensions(&self) -> Result<Vec<ExtensionModel>> {
        let extensions = self
            .extensions
            .get_extensions()
            .into_iter()
            .map(ExtensionModel::from)
            .collect();

        Ok(extensions)
    }

    async fn open_share_url(&self, url: &str) -> Result<Option<OpenResultModel>> {
        let internal_url = self.app.resolve_share_url(url.to_owned()).await?;
        let result = internal_url.map(OpenResultModel::from);

        Ok(result)
    }

    async fn get_thumbnail(&self, cursor: Cursor) -> Result<Option<CoverArtModel>> {
        let uri = cursor.try_into()?;
        let provider_item = match uri {
            InternalUri::Track(uri) => {
                let query = SingleQuery::uri(uri);
                let track = self.app.query_track(query).await?;
                track.map(ProviderItemType::Track)
            },
            InternalUri::Album(uri) => {
                let query = SingleQuery::uri(uri);
                let album = self.app.query_album(query).await?;
                album.map(ProviderItemType::Album)
            },
            InternalUri::Artist(uri) => {
                let query = SingleQuery::uri(uri);
                let artist = self.app.query_artist(query).await?;
                artist.map(ProviderItemType::Artist)
            },
            _ => None
        };

        if let Some(item) = provider_item {
            let cover_art = self.app.thumbnail(&item).await?;
            let cover_art = cover_art.map(CoverArtModel::from);

            Ok(cover_art)
        } else {
            Ok(None)
        }
    }
}

impl std::fmt::Debug for RusticNativeClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RusticNativeClient").finish()
    }
}
