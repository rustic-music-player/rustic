use failure::{format_err, Error};
use futures::prelude::*;
use pocketcasts::{Episode, PocketcastClient, Podcast};
use serde::Deserialize;

use async_trait::async_trait;
use rustic_core::library::{Album, Artist, SharedLibrary, Track};
use rustic_core::{provider, CredentialStore, Credentials, Playlist};

use crate::episode::PocketcastTrack;
use crate::podcast::{PocketcastAlbum, PocketcastAlbums, PocketcastSearchResult};

mod episode;
mod meta;
mod podcast;

#[derive(Debug, Clone, Deserialize)]
pub struct PocketcastsCredentials {
    email: String,
    password: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct PocketcastsProvider {
    #[serde(flatten)]
    credentials: Option<PocketcastsCredentials>,
    #[serde(skip)]
    client: Option<PocketcastClient>,
}

impl PocketcastsProvider {
    pub fn new() -> Option<Self> {
        Some(Self::default())
    }

    async fn login(&mut self, username: String, password: String) -> Result<(), Error> {
        self.client = Some(PocketcastClient::login(username, password).await?);
        Ok(())
    }
}

#[async_trait]
impl provider::ProviderInstance for PocketcastsProvider {
    async fn setup(&mut self, cred_store: &dyn CredentialStore) -> Result<(), Error> {
        self.client = if let Some(ref credentials) = self.credentials {
            Some(
                PocketcastClient::login(credentials.email.clone(), credentials.password.clone())
                    .await?,
            )
        } else if let Some(Credentials::UserPass { username, password }) = cred_store
            .get_credentials(provider::ProviderType::Pocketcasts)
            .await?
        {
            Some(PocketcastClient::login(username, password).await?)
        } else {
            None
        };

        Ok(())
    }

    fn title(&self) -> &'static str {
        "Pocketcasts"
    }

    fn uri_scheme(&self) -> &'static str {
        "pocketcasts"
    }

    fn provider(&self) -> provider::ProviderType {
        provider::ProviderType::Pocketcasts
    }

    fn state(&self) -> provider::ProviderState {
        if self.client.is_some() {
            provider::ProviderState::Authenticated(None)
        } else {
            provider::ProviderState::RequiresPassword
        }
    }

    async fn authenticate(
        &mut self,
        authentication: provider::Authentication,
        cred_store: &dyn CredentialStore,
    ) -> Result<(), Error> {
        use provider::Authentication::*;

        match authentication {
            Password(email, password) => {
                self.login(email.clone(), password.clone()).await?;
                cred_store
                    .store_credentials(
                        provider::ProviderType::Pocketcasts,
                        Credentials::password(email, password),
                    )
                    .await?;
                Ok(())
            }
            _ => Err(format_err!("Invalid authentication method")),
        }
    }

    async fn sync(&self, library: SharedLibrary) -> Result<provider::SyncResult, Error> {
        let client = self
            .client
            .clone()
            .ok_or_else(|| format_err!("Pocketcasts not setup"))?;
        let podcasts = client.get_subscriptions().await?;
        let albums = podcasts.len();
        let futures: Vec<future::BoxFuture<_>> = podcasts
            .into_iter()
            .map(|p| get_episodes(&client, p))
            .collect();
        let podcast_episodes: Vec<(Podcast, Vec<Episode>)> =
            futures::future::try_join_all(futures).await?;
        let mut episodes: Vec<Track> = podcast_episodes
            .into_iter()
            .map(|(podcast, episodes)| {
                let mut artist = Artist::from(PocketcastAlbum::from(podcast.clone()));
                let mut album = Album::from(PocketcastAlbum::from(podcast));
                library.sync_artist(&mut artist);
                album.artist_id = artist.id;
                library.sync_album(&mut album);
                let tracks: Vec<Track> = episodes
                    .iter()
                    .cloned()
                    .map(PocketcastTrack::from)
                    .map(Track::from)
                    .map(|mut track| {
                        track.album_id = album.id;
                        track.artist_id = artist.id;
                        track.thumbnail = album.thumbnail.clone();
                        track
                    })
                    .collect();
                tracks
            })
            .fold(vec![], |mut a, b| {
                a.extend(b);
                a
            });
        let tracks = episodes.len();
        library.sync_tracks(&mut episodes);
        Ok(provider::SyncResult {
            tracks,
            albums,
            artists: albums,
            playlists: 0,
        })
    }

    fn root(&self) -> provider::ProviderFolder {
        provider::ProviderFolder {
            folders: vec![
                "Subscriptions".to_owned(),
                "Top Charts".to_owned(),
                "Featured".to_owned(),
                "Trending".to_owned(),
            ],
            items: vec![],
        }
    }

    async fn navigate(&self, path: Vec<String>) -> Result<provider::ProviderFolder, Error> {
        let client = self.client.clone().unwrap();
        let podcasts = match path[0].as_str() {
            "Subscriptions" => client.get_subscriptions().await,
            //            "Top Charts" => Ok(PocketcastClient::get_top_charts()),
            //            "Featured" => Ok(PocketcastClient::get_featured()),
            //            "Trending" => Ok(PocketcastClient::get_trending()),
            _ => Err(Error::from(provider::NavigationError::PathNotFound)),
        }?;
        match path.len() {
            1 => Ok(provider::ProviderFolder::from(PocketcastAlbums::from(
                podcasts,
            ))),
            2 => {
                let podcast = podcasts
                    .iter()
                    .find(|podcast| podcast.title == path[1])
                    .ok_or(provider::NavigationError::PathNotFound)?;
                let episodes = client
                    .get_episodes(podcast.uuid)
                    .await
                    .map_err(|_err| Error::from(provider::NavigationError::FetchError))?;
                let items = episodes
                    .iter()
                    .cloned()
                    .map(PocketcastTrack::from)
                    .map(Track::from)
                    .map(provider::ProviderItem::from)
                    .collect();

                Ok(provider::ProviderFolder {
                    folders: vec![],
                    items,
                })
            }
            _ => Err(Error::from(provider::NavigationError::PathNotFound)),
        }
    }

    async fn search(&self, query: String) -> Result<Vec<provider::ProviderItem>, Error> {
        let client = self.client.clone().unwrap();
        let podcasts = client.search_podcasts(query).await?;
        let podcasts = podcasts
            .into_iter()
            .map(PocketcastSearchResult::from)
            .map(Album::from)
            .map(provider::ProviderItem::from)
            .collect();
        Ok(podcasts)
    }

    async fn resolve_track(&self, _uri: &str) -> Result<Option<Track>, Error> {
        unimplemented!()
    }

    async fn resolve_album(&self, _uri: &str) -> Result<Option<Album>, Error> {
        unimplemented!()
    }

    async fn resolve_artist(&self, _uri: &str) -> Result<Option<Artist>, Error> {
        unimplemented!()
    }

    async fn resolve_playlist(&self, _uri: &str) -> Result<Option<Playlist>, Error> {
        unimplemented!()
    }

    async fn stream_url(&self, track: &Track) -> Result<String, Error> {
        if track.provider == provider::ProviderType::Pocketcasts {
            if let rustic_core::library::MetaValue::String(stream_url) =
                track.meta.get(meta::META_POCKETCASTS_STREAM_URL).unwrap()
            {
                return Ok(stream_url.to_string());
            }

            return Err(format_err!(
                "Can't get stream url from track, meta value incompatible"
            ));
        }

        Err(format_err!("Invalid provider: {:?}", track.provider))
    }

    async fn resolve_share_url(&self, _: url::Url) -> Result<Option<provider::InternalUri>, Error> {
        Ok(None)
    }
}

fn get_episodes(
    client: &PocketcastClient,
    podcast: Podcast,
) -> future::BoxFuture<Result<(Podcast, Vec<Episode>), Error>> {
    client
        .get_episodes(podcast.uuid)
        .map(|result| result.map(|episodes| (podcast, episodes)))
        .boxed()
}
