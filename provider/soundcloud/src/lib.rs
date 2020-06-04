use std::str::FromStr;

use failure::{ensure, format_err, Error};
use log::{trace, warn};
use serde::Deserialize;

use async_trait::async_trait;
use lazy_static::lazy_static;
use rustic_core::library::{Album, Artist, MetaValue, Playlist, SharedLibrary, Track};
use rustic_core::{provider, CredentialStore, Credentials, ProviderType};

use crate::playlist::SoundcloudPlaylist;
use crate::track::SoundcloudTrack;
use crate::user::SoundcloudUser;
use std::collections::HashMap;

mod error;
mod meta;
mod playlist;
mod track;
mod user;

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
const USER_URI_PREFIX: &str = "soundcloud://user/";

#[derive(Debug, Clone, Deserialize)]
pub struct SoundcloudProvider {
    client_id: String,
    auth_token: Option<String>,
}

impl SoundcloudProvider {
    pub fn new() -> Option<Self> {
        let client_id = option_env!("SOUNDCLOUD_CLIENT_ID");

        client_id.map(|client_id| SoundcloudProvider {
            client_id: client_id.into(),
            auth_token: None,
        })
    }
}

impl SoundcloudProvider {
    fn client(&self) -> soundcloud::Client {
        let mut client = soundcloud::Client::new(self.client_id.as_str());
        if let Some(token) = self.auth_token.as_ref() {
            client.authenticate_with_token(token.clone());
        }
        client
    }

    async fn get_playlist(&self, uri: &str) -> Result<Option<SoundcloudPlaylist>, Error> {
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

        Ok(Some(playlist))
    }

    async fn search_tracks(
        &self,
        client: &soundcloud::Client,
        query: &str,
    ) -> Result<Vec<provider::ProviderItem>, Error> {
        let tracks = client
            .tracks()
            .query(Some(query))
            .get()
            .await?
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

        Ok(tracks)
    }

    async fn search_users(
        &self,
        client: &soundcloud::Client,
        query: &str,
    ) -> Result<Vec<provider::ProviderItem>, Error> {
        let users = client
            .users()
            .query(Some(query))
            .get()
            .await?
            .into_iter()
            .map(SoundcloudUser::from)
            .map(provider::ProviderItem::from)
            .collect();

        Ok(users)
    }

    async fn search_playlists(
        &self,
        client: &soundcloud::Client,
        query: &str,
    ) -> Result<Vec<provider::ProviderItem>, Error> {
        let playlists = client
            .playlists()
            .query(query)
            .get()
            .await?
            .into_iter()
            .map(SoundcloudPlaylist::from)
            .map(provider::ProviderItem::from)
            .collect();

        Ok(playlists)
    }
}

#[async_trait]
impl provider::ProviderInstance for SoundcloudProvider {
    async fn setup(&mut self, cred_store: &dyn CredentialStore) -> Result<(), Error> {
        if let Some(Credentials::Token(token)) = cred_store
            .get_credentials(provider::ProviderType::Soundcloud)
            .await?
        {
            self.auth_token = Some(token);
        }
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
                format!("https://soundcloud.com/connect?client_id={}&response_type=token&redirect_uri={}/api/auth/soundcloud", &self.client_id, SOUNDCLOUD_REDIRECT_URI)
            )
        }
    }

    async fn authenticate(
        &mut self,
        auth: provider::Authentication,
        cred_store: &dyn CredentialStore,
    ) -> Result<(), Error> {
        use provider::Authentication::*;
        match auth {
            Token(token) => {
                self.auth_token = Some(token.clone());
                cred_store
                    .store_credentials(
                        provider::ProviderType::Soundcloud,
                        Credentials::Token(token),
                    )
                    .await?;
                Ok(())
            }
            _ => Err(format_err!("Invalid authentication method")),
        }
    }

    async fn sync(&self, library: SharedLibrary) -> Result<provider::SyncResult, Error> {
        let client = self.client();
        let mut playlists: Vec<Playlist> = client
            .my_playlists()
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
        let mut result = self.search_tracks(&client, &query).await?;
        let users = self.search_users(&client, &query).await?;
        let playlists = self.search_playlists(&client, &query).await?;
        result.extend(users);
        result.extend(playlists);
        Ok(result)
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

    async fn resolve_album(&self, uri: &str) -> Result<Option<Album>, Error> {
        let album = self.get_playlist(uri).await?.map(Album::from);

        Ok(album)
    }

    async fn resolve_artist(&self, uri: &str) -> Result<Option<Artist>, Error> {
        ensure!(uri.starts_with(USER_URI_PREFIX), "Invalid Uri: {}", uri);
        let id = &uri[USER_URI_PREFIX.len()..];
        let id = usize::from_str(id)?;
        let client = self.client();
        let user = client.user(id).get().await?;
        let (albums, playlists) = client
            .user(id)
            .playlists()
            .await?
            .into_iter()
            .map(SoundcloudPlaylist::from)
            .partition::<Vec<_>, _>(|p| p.is_album());

        let albums = albums.into_iter().map(Album::from).collect();
        let playlists = playlists.into_iter().map(Playlist::from).collect();

        Ok(Some(Artist {
            id: None,
            name: user.username,
            playlists,
            albums,
            provider: ProviderType::Soundcloud,
            meta: HashMap::new(),
            image_url: Some(user.avatar_url),
            uri: format!("soundcloud://user/{}", user.id),
        }))
    }

    async fn resolve_playlist(&self, uri: &str) -> Result<Option<Playlist>, Error> {
        let playlist = self.get_playlist(uri).await?.map(Playlist::from);

        Ok(playlist)
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
