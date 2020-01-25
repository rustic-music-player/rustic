#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate maplit;
extern crate pocketcasts;
extern crate rayon;
extern crate rustic_core as rustic;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate url;

use failure::Error;
use pocketcasts::{Episode, PocketcastClient, Podcast};
use rayon::prelude::*;

use episode::PocketcastTrack;
use meta::META_POCKETCASTS_COVER_ART_URL;
use podcast::{PocketcastAlbum, PocketcastAlbums, PocketcastSearchResult};
use rustic::library::{Album, Artist, MetaValue, SharedLibrary, Track};
use rustic::{provider, Playlist};

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

impl provider::ProviderInstance for PocketcastsProvider {
    fn setup(&mut self) -> Result<(), Error> {
        let mut client = PocketcastClient::new(self.email.clone(), self.password.clone());
        client.login()?;
        self.client = Some(client);

        Ok(())
    }

    fn title(&self) -> &'static str {
        "Pocketcasts"
    }

    fn uri_scheme(&self) -> &'static str {
        "pocketcasts"
    }

    fn provider(&self) -> provider::Provider {
        provider::Provider::Pocketcasts
    }

    fn auth_state(&self) -> provider::AuthState {
        provider::AuthState::Authenticated(Some(provider::User {
            email: Some(self.email.clone()),
            ..provider::User::default()
        }))
    }

    fn authenticate(&mut self, authentication: provider::Authentication) -> Result<(), Error> {
        use provider::Authentication::*;

        match authentication {
            Password(email, password) => {
                self.email = email;
                self.password = password;
                self.setup()?;
                Ok(())
            }
            _ => Err(format_err!("Invalid authentication method")),
        }
    }

    fn sync(&self, library: SharedLibrary) -> Result<provider::SyncResult, Error> {
        let client = self
            .client
            .clone()
            .ok_or_else(|| format_err!("Pocketcasts not setup"))?;
        let podcasts = client.get_subscriptions()?;
        let albums = podcasts.len();
        let mut episodes: Vec<Track> = podcasts
            .par_iter()
            .cloned()
            .map(|podcast| {
                let episodes = client.get_episodes(&podcast.uuid)?;
                Ok((podcast, episodes))
            })
            .filter(|result: &Result<(Podcast, Vec<Episode>), Error>| {
                debug!("{:?}", result);
                result.is_ok()
            })
            .map(|result| result.unwrap())
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
            .reduce(
                || vec![],
                |mut a, b| {
                    a.extend(b);
                    a
                },
            );
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

    fn navigate(&self, path: Vec<String>) -> Result<provider::ProviderFolder, Error> {
        let client = self.client.clone().unwrap();
        let podcasts = match path[0].as_str() {
            "Subscriptions" => Ok(self.client.clone().unwrap().get_subscriptions()),
            //            "Top Charts" => Ok(PocketcastClient::get_top_charts()),
            //            "Featured" => Ok(PocketcastClient::get_featured()),
            //            "Trending" => Ok(PocketcastClient::get_trending()),
            _ => Err(Error::from(provider::NavigationError::PathNotFound)),
        }?;
        match path.len() {
            1 => podcasts
                .map(PocketcastAlbums::from)
                .map(provider::ProviderFolder::from),
            2 => podcasts.and_then(|podcasts| {
                podcasts
                    .iter()
                    .find(|podcast| podcast.title == path[1])
                    .ok_or(provider::NavigationError::PathNotFound)
                    .map_err(Error::from)
                    .and_then(|podcast| {
                        client
                            .get_episodes(&podcast.uuid)
                            .map_err(|_err| Error::from(provider::NavigationError::FetchError))
                    })
                    .map(|episodes| {
                        episodes
                            .iter()
                            .cloned()
                            .map(PocketcastTrack::from)
                            .map(Track::from)
                            .map(provider::ProviderItem::from)
                            .collect()
                    })
                    // .ok_or(Error::from(provider::NavigationError::FetchError))
                    .map(|items| provider::ProviderFolder {
                        folders: vec![],
                        items,
                    })
            }),
            _ => Err(Error::from(provider::NavigationError::PathNotFound)),
        }
    }

    fn search(&self, query: String) -> Result<Vec<provider::ProviderItem>, Error> {
        let client = self.client.clone().unwrap();
        let podcasts = client.search_podcasts(query)?;
        let podcasts = podcasts
            .into_iter()
            .map(PocketcastSearchResult::from)
            .map(Album::from)
            .map(provider::ProviderItem::from)
            .collect();
        Ok(podcasts)
    }

    fn resolve_track(&self, _uri: &str) -> Result<Option<Track>, Error> {
        unimplemented!()
    }

    fn resolve_album(&self, _uri: &str) -> Result<Option<Album>, Error> {
        unimplemented!()
    }

    fn resolve_playlist(&self, _uri: &str) -> Result<Option<Playlist>, Error> {
        unimplemented!()
    }

    fn stream_url(&self, track: &Track) -> Result<String, Error> {
        if track.provider == provider::Provider::Pocketcasts {
            if let rustic::library::MetaValue::String(stream_url) =
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

    fn cover_art(&self, track: &Track) -> Result<Option<provider::CoverArt>, Error> {
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

    fn resolve_share_url(&self, _: url::Url) -> Result<Option<provider::InternalUri>, Error> {
        Ok(None)
    }
}
