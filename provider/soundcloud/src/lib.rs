use std::str::FromStr;

use failure::{ensure, format_err, Error};
use log::{trace, warn};
use serde::Deserialize;

use async_trait::async_trait;
use lazy_static::lazy_static;
use rustic_core::library::{Album, MetaValue, Playlist, SharedLibrary, Track};
use rustic_core::provider;

use crate::playlist::SoundcloudPlaylist;
use crate::track::SoundcloudTrack;

mod error;
mod meta;
mod playlist;
mod track;

// TODO: configurable host
const SOUNDCLOUD_REDIRECT_URI: &str =
    "http://localhost:8080/api/providers/soundcloud/auth/redirect";

lazy_static! {
    static ref SOUNDCLOUD_RESOLVE_REGEX: regex::Regex =
        regex::RegexBuilder::new("^/([a-z]+)/([0-9]+)")
            .case_insensitive(true)
            .build()
            .unwrap();
}

const TRACK_URI_PREFIX: &str = "soundcloud://track/";

#[derive(Debug, Clone, Deserialize)]
pub struct SoundcloudProvider {
    client_id: String,
    auth_token: Option<String>,
}

impl SoundcloudProvider {
    fn client(&self) -> soundcloud::Client {
        let mut client = soundcloud::Client::new(self.client_id.as_str());
        if let Some(token) = self.auth_token.as_ref() {
            client.authenticate_with_token(token.clone());
        }
        client
    }
}

#[async_trait]
impl provider::ProviderInstance for SoundcloudProvider {
    async fn setup(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn title(&self) -> &'static str {
        "Soundcloud"
    }

    fn uri_scheme(&self) -> &'static str {
        "soundcloud"
    }

    fn provider(&self) -> provider::ProviderType {
        provider::ProviderType::Soundcloud
    }

    fn auth_state(&self) -> provider::AuthState {
        if self.auth_token.is_some() {
            provider::AuthState::Authenticated(None)
        } else {
            provider::AuthState::RequiresOAuth(
                format!("https://soundcloud.com/connect?client_id={}&response_type=token&redirect_uri={}/api/auth/soundcloud", self.client_id, SOUNDCLOUD_REDIRECT_URI)
            )
        }
    }

    async fn authenticate(&mut self, auth: provider::Authentication) -> Result<(), Error> {
        use provider::Authentication::*;
        match auth {
            Token(token) => {
                self.auth_token = Some(token);
                Ok(())
            }
            _ => Err(format_err!("Invalid authentication method")),
        }
    }

    async fn sync(&self, library: SharedLibrary) -> Result<provider::SyncResult, Error> {
        let client = self.client();
        let mut playlists: Vec<Playlist> = client
            .user_playlists()
            .await?
            .iter()
            .cloned()
            .map(playlist::SoundcloudPlaylist::from)
            .map(Playlist::from)
            .collect();
        library.sync_playlists(&mut playlists)?;
        Ok(provider::SyncResult {
            tracks: 0,
            albums: 0,
            artists: 0,
            playlists: playlists.len(),
        })
    }
    fn root(&self) -> provider::ProviderFolder {
        provider::ProviderFolder {
            folders: vec!["Likes".to_owned()],
            items: vec![],
        }
    }
    async fn navigate(&self, path: Vec<String>) -> Result<provider::ProviderFolder, Error> {
        match path[0].as_str() {
            "Likes" => {
                let client = self.client();
                let likes = client.likes().await?;
                let items = likes
                    .iter()
                    .cloned()
                    .map(|track| {
                        provider::ProviderItem::from(Track::from(SoundcloudTrack::from(track)))
                    })
                    .collect();
                let folder = provider::ProviderFolder {
                    folders: vec![],
                    items,
                };
                Ok(folder)
            }
            _ => Err(Error::from(provider::NavigationError::PathNotFound)),
        }
    }

    async fn search(&self, query: String) -> Result<Vec<provider::ProviderItem>, Error> {
        trace!("search {}", query);
        let client = self.client();
        let results = client
            .tracks()
            .query(Some(query))
            .get()
            .await?
            .unwrap_or_else(|| vec![])
            .into_iter()
            .filter(|track| {
                let has_url = track.stream_url.is_some();
                if !has_url {
                    warn!("Track {:?} has no stream url", &track);
                }
                has_url
            })
            .map(SoundcloudTrack::from)
            .map(|track| track.into())
            .collect();
        Ok(results)
    }

    async fn resolve_track(&self, uri: &str) -> Result<Option<Track>, Error> {
        ensure!(uri.starts_with(TRACK_URI_PREFIX), "Invalid Uri: {}", uri);
        let id = &uri[TRACK_URI_PREFIX.len()..];
        let id = usize::from_str(id)?;
        let client = self.client();
        let track = client
            .tracks()
            .id(id)
            .get()
            .await
            .ok()
            .map(SoundcloudTrack::from)
            .map(Track::from);
        Ok(track)
    }

    async fn resolve_album(&self, _uri: &str) -> Result<Option<Album>, Error> {
        Ok(None)
    }

    async fn resolve_playlist(&self, uri: &str) -> Result<Option<Playlist>, Error> {
        ensure!(
            uri.starts_with("soundcloud://playlist/"),
            "Invalid Uri: {}",
            uri
        );
        let id = &uri["soundcloud://playlist/".len()..];
        let id = usize::from_str(id)?;
        let client = self.client();
        let playlist = client.playlist(id).get().await?;
        let playlist = SoundcloudPlaylist::from(playlist);
        let playlist = Playlist::from(playlist);

        Ok(Some(playlist))
    }

    async fn stream_url(&self, track: &Track) -> Result<String, Error> {
        if track.provider == provider::ProviderType::Soundcloud {
            if let rustic_core::library::MetaValue::String(stream_url) =
                track.meta.get(meta::META_SOUNDCLOUD_STREAM_URL).unwrap()
            {
                return Ok(format!(
                    "{}?client_id={}",
                    stream_url,
                    self.client_id.clone()
                ));
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
            .get(meta::META_SOUNDCLOUD_COVER_ART_URL)
            .map(|value| match value {
                MetaValue::String(url) => url.clone(),
                _ => unreachable!(),
            })
            .map(|url| url.into());

        Ok(url)
    }

    async fn resolve_share_url(
        &self,
        url: url::Url,
    ) -> Result<Option<provider::InternalUri>, Error> {
        if url.domain() != Some("soundcloud.com") {
            return Ok(None);
        }
        let client = self.client();
        let url = client.resolve(url.as_str()).await?;
        let path = url.path();
        if let Some(captures) = SOUNDCLOUD_RESOLVE_REGEX.captures(path) {
            let entity_type = &captures[1];
            let id = &captures[2];
            let entity = match entity_type {
                "tracks" => Some(provider::InternalUri::Track(format!(
                    "{}{}",
                    TRACK_URI_PREFIX, id
                ))),
                "playlists" => Some(provider::InternalUri::Playlist(format!(
                    "soundcloud://playlist/{}",
                    id
                ))),
                "users" => Some(provider::InternalUri::Artist(format!(
                    "soundcloud://user/{}",
                    id
                ))),
                _ => None,
            };
            Ok(entity)
        } else {
            Ok(None)
        }
    }
}
