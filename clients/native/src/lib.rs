use std::sync::Arc;

use log::{debug, error, trace};
use futures::future;

use async_trait::async_trait;
use rustic_api::client::*;
use rustic_api::cursor::{Cursor};
use rustic_api::models::*;
use rustic_core::provider::{InternalUri, ProviderItem, ProviderItemType};
use rustic_core::{Album, Artist, CredentialStore, Playlist, Provider, Rustic, SingleQuery, Track};
use rustic_extension_api::{ExtensionManager, ExtensionApi};
use std::convert::TryInto;

mod library;
mod player;
mod playlist;
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
    pub fn new(
        app: Arc<Rustic>,
        extensions: ExtensionManager,
        cred_store: Box<dyn CredentialStore>,
    ) -> RusticNativeClient {
        RusticNativeClient {
            app,
            extensions,
            credential_store: Arc::new(cred_store),
        }
    }

    async fn query_album(&self, query: SingleQuery) -> Result<Option<Album>> {
        let album = self.app.query_album(query).await?;
        let album = if let Some(album) = album {
            Some(self.extensions.resolve_album(album).await?)
        } else {
            None
        };

        Ok(album)
    }

    async fn query_artist(&self, query: SingleQuery) -> Result<Option<Artist>> {
        let artist = self.app.query_artist(query).await?;
        let artist = if let Some(artist) = artist {
            Some(self.extensions.resolve_artist(artist).await?)
        } else {
            None
        };

        Ok(artist)
    }

    async fn query_track(&self, query: SingleQuery) -> Result<Option<Track>> {
        let track = self.app.query_track(query).await?;
        let track = if let Some(track) = track {
            Some(self.extensions.resolve_track(track).await?)
        } else {
            None
        };

        Ok(track)
    }

    async fn query_playlist(&self, query: SingleQuery) -> Result<Option<Playlist>> {
        let playlist = self.app.query_playlist(query).await?;
        let playlist = if let Some(playlist) = playlist {
            Some(self.extensions.resolve_playlist(playlist).await?)
        } else {
            None
        };

        Ok(playlist)
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
            if !provider.state().is_authenticated() {
                continue;
            }
            match provider.search(query.to_string()).await {
                Ok(mut result) => {
                    results.append(&mut result);
                }
                Err(e) => error!(
                    "Searching failed for provider {:?}: {:?}",
                    provider.provider(),
                    e
                ),
            }
        }
        debug!("Searching took {}ms", sw.elapsed_ms());

        let tracks = future::try_join_all(results
            .iter()
            .cloned()
            .filter(|result| result.is_track())
            .map(Track::from)
            .map(|track| self.extensions.resolve_track(track))).await?;
        let tracks: Vec<_> = tracks
            .into_iter()
            .map(TrackModel::from)
            .collect();

        let albums = future::try_join_all(results
            .iter()
            .cloned()
            .filter(|result| result.is_album())
            .map(Album::from)
            .map(|album| self.extensions.resolve_album(album))).await?;
        let albums: Vec<_> = albums
            .into_iter()
            .map(AlbumModel::from)
            .collect();

        let artists = future::try_join_all(results
            .iter()
            .cloned()
            .filter(|result| result.is_artist())
            .map(Artist::from)
            .map(|artist| self.extensions.resolve_artist(artist))).await?;
        let artists: Vec<_> = artists
            .into_iter()
            .map(ArtistModel::from)
            .collect();

        let playlists = future::try_join_all(results
            .iter()
            .cloned()
            .filter(|result| result.is_playlist())
            .map(Playlist::from)
            .map(|playlist| self.extensions.resolve_playlist(playlist))).await?;
        let playlists: Vec<_> = playlists
            .into_iter()
            .map(PlaylistModel::from)
            .collect();

        Ok(SearchResults {
            tracks,
            albums,
            artists,
            playlists,
        })
    }

    async fn aggregated_search(
        &self,
        query: &str,
        providers: Option<Vec<ProviderTypeModel>>,
    ) -> Result<AggregatedSearchResults> {
        let results = self.search(query, providers).await?;

        Ok(aggregate_results(results))
    }

    async fn enable_extension(&self, id: &str) -> Result<()> {
        self.extensions.enable_extension(id).await
    }

    async fn disable_extension(&self, id: &str) -> Result<()> {
        self.extensions.disable_extension(id).await
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
            }
            InternalUri::Album(uri) => {
                let query = SingleQuery::uri(uri);
                let album = self.app.query_album(query).await?;
                album.map(ProviderItemType::Album)
            }
            InternalUri::Artist(uri) => {
                let query = SingleQuery::uri(uri);
                let artist = self.app.query_artist(query).await?;
                artist.map(ProviderItemType::Artist)
            }
            _ => None,
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

fn aggregate_results(results: SearchResults) -> AggregatedSearchResults {
    let tracks: Vec<TrackCollection> = Aggregate::aggregate(results.tracks);
    let albums: Vec<AlbumCollection> = Aggregate::aggregate(results.albums);
    let artists: Vec<ArtistCollection> = Aggregate::aggregate(results.artists);

    AggregatedSearchResults {
        tracks: tracks.into_iter().map(AggregatedTrack::from).collect(),
        albums: albums.into_iter().map(AggregatedAlbum::from).collect(),
        artists: artists.into_iter().map(AggregatedArtist::from).collect(),
        playlists: results.playlists,
    }
}
