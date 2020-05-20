use failure::{format_err, Error};
use pocketcasts::{Episode, PocketcastClient, Podcast};
use serde::Deserialize;

use async_trait::async_trait;
use rustic_core::library::{Album, Artist, MetaValue, SharedLibrary, Track};
use rustic_core::{provider, Playlist};

use crate::episode::PocketcastTrack;
use crate::meta::META_POCKETCASTS_COVER_ART_URL;
use crate::podcast::{PocketcastAlbum, PocketcastAlbums, PocketcastSearchResult};
use futures::prelude::*;

mod episode;
mod meta;
mod podcast;

#[derive(Debug, Clone, Deserialize)]
pub struct PocketcastsProvider {
    email: String,
    password: String,
    #[serde(skip)]
    client: Option<PocketcastClient>,
}

#[async_trait]
impl provider::ProviderInstance for PocketcastsProvider {
    async fn setup(&mut self) -> Result<(), Error> {
        let client = PocketcastClient::login(self.email.clone(), self.password.clone()).await?;
        self.client = Some(client);

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

    fn auth_state(&self) -> provider::AuthState {
        provider::AuthState::Authenticated(Some(provider::User {
            email: Some(self.email.clone()),
            ..provider::User::default()
        }))
    }

    async fn authenticate(
        &mut self,
        authentication: provider::Authentication,
    ) -> Result<(), Error> {
        use provider::Authentication::*;

        match authentication {
            Password(email, password) => {
                self.email = email;
                self.password = password;
                self.setup().await?;
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
                        track.has_coverart = album.image_url.is_some();
                        if let Some(image_url) = album.image_url.as_ref() {
                            track.meta.insert(
                                META_POCKETCASTS_COVER_ART_URL.into(),
                                image_url.clone().into(),
                            );
                        }
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

    async fn cover_art(&self, track: &Track) -> Result<Option<provider::CoverArt>, Error> {
        let url = track
            .meta
            .get(meta::META_POCKETCASTS_COVER_ART_URL)
            .map(|value| match value {
                MetaValue::String(url) => url.clone(),
                _ => unreachable!(),
            })
            .map(|url| url.into());

        Ok(url)
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
