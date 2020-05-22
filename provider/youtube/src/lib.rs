use failure::{ensure, format_err, Error};
use log::{debug, warn};
use serde::Deserialize;
use url::Url;

use async_trait::async_trait;
use lazy_static::lazy_static;
use rustic_core::{Album, Playlist, ProviderType, SharedLibrary, Track};
use rustic_core::library::MetaValue;
use rustic_core::provider::{
    Authentication, AuthState, CoverArt, InternalUri, ProviderFolder, ProviderInstance,
    ProviderItem, SyncResult,
};
use tokio::sync::Mutex;
use youtube::{YoutubeApi, YoutubeDl};
use youtube::models::SearchRequestBuilder;

use crate::meta::META_YOUTUBE_DEFAULT_THUMBNAIL_URL;
use crate::search_result::YoutubeSearchResult;
use crate::video_metadata::YoutubeVideoMetadata;

mod meta;
mod search_result;
mod video_metadata;

lazy_static! {
    // Source: https://github.com/ritiek/rafy-rs
    static ref YOUTUBE_VIDEO_REGEX: regex::Regex =
        regex::RegexBuilder::new(r"^.*(?:(?:youtu\.be/|v/|vi/|u/w/|embed/)|(?:(?:watch)?\?v(?:i)?=|\&v(?:i)?=))([^#\&\?]*).*")
            .case_insensitive(true)
            .build()
            .unwrap();
    static ref STATE_CACHE: Mutex<Option<String>> = Mutex::new(None);
}

// TODO: configurable host
const YOUTUBE_REDIRECT_URI: &str = "http://localhost:8080/api/providers/youtube/auth/redirect";

const VIDEO_URI_PREFIX: &str = "youtube://video/";

#[derive(Debug, Clone, Deserialize)]
pub struct YoutubeProvider {
    #[serde(default)]
    api_key: Option<String>,
    #[serde(default)]
    client_id: Option<String>,
    #[serde(default)]
    client_secret: Option<String>,
    #[serde(skip)]
    client: Option<YoutubeApi>,
}

impl YoutubeProvider {
    fn get_youtube_id<'a>(&self, uri: &'a str) -> Result<&'a str, failure::Error> {
        ensure!(uri.starts_with(VIDEO_URI_PREFIX), "Invalid Uri: {}", uri);
        let id = &uri[VIDEO_URI_PREFIX.len()..];

        Ok(id)
    }
}

#[async_trait]
impl ProviderInstance for YoutubeProvider {
    async fn setup(&mut self) -> Result<(), Error> {
        self.client = match (self.api_key.as_ref(), self.client_id.as_ref(), self.client_secret.as_ref()) {
            (Some(api_key), None, None) => Some(YoutubeApi::new(api_key)),
            (Some(api_key), Some(client_id), Some(client_secret)) => {
                Some(YoutubeApi::new_with_oauth(api_key, client_id.clone(), client_secret.clone(), Some(YOUTUBE_REDIRECT_URI))?)
            },
            (None, None, None) => None,
            _ => return Err(format_err!("Invalid provider configuration "))
        };
        Ok(())
    }

    fn title(&self) -> &'static str {
        "YouTube"
    }

    fn uri_scheme(&self) -> &'static str {
        "youtube"
    }

    fn provider(&self) -> ProviderType {
        ProviderType::Youtube
    }

    fn auth_state(&self) -> AuthState {
        if let Some(client) = self.client.as_ref() {
            if client.has_token() {
                AuthState::Authenticated(None)
            } else {
                let (url, state) = client.get_oauth_url().unwrap();
                let mut state_cache = STATE_CACHE.try_lock().unwrap();
                *state_cache = Some(state);
                AuthState::RequiresOAuth(url)
            }
        }else {
            AuthState::NoAuthentication
        }
    }

    async fn authenticate(&mut self, auth: Authentication) -> Result<(), Error> {
        let client = self.client.as_mut().expect("client isn't setup yet");
        use rustic_core::provider::Authentication::*;

        match auth {
            Token(token) | TokenWithState(token, _) => {
                let mut state_cache = STATE_CACHE.lock().await;
                let state = state_cache
                    .take()
                    .ok_or_else(|| format_err!("Missing state"))?;
                debug!("State: {}", state);
                client.request_token(token, state).await?;
                client.store_token().await?;
                Ok(())
            }
            _ => Err(format_err!("Invalid authentication method")),
        }
    }

    async fn sync(&self, library: SharedLibrary) -> Result<SyncResult, Error> {
        warn!("sync is not implemented");
        Ok(SyncResult::empty())
    }

    fn root(&self) -> ProviderFolder {
        ProviderFolder {
            folders: vec!["Subscriptions".to_owned(), "Trending".to_owned()],
            items: vec![],
        }
    }

    async fn navigate(&self, path: Vec<String>) -> Result<ProviderFolder, Error> {
        unimplemented!()
    }

    async fn search(&self, query: String) -> Result<Vec<ProviderItem>, Error> {
        let request = SearchRequestBuilder {
            query: Some(query),
            ..SearchRequestBuilder::default()
        };
        let response = self.client.as_ref().unwrap().search(request).await?;
        let result = response.items.into_iter()
            .map(YoutubeSearchResult::from)
            .map(ProviderItem::from)
            .collect();
        Ok(result)
    }

    async fn resolve_track(&self, uri: &str) -> Result<Option<Track>, Error> {
        ensure!(uri.starts_with(VIDEO_URI_PREFIX), "Invalid Uri: {}", uri);
        let id = &uri[VIDEO_URI_PREFIX.len()..];
        let content = YoutubeApi::get_video_info(id).await?;
        let video = YoutubeVideoMetadata::from(content);
        let track = video.into();

        Ok(Some(track))
    }

    async fn resolve_album(&self, uri: &str) -> Result<Option<Album>, Error> {
        unimplemented!()
    }

    async fn resolve_playlist(&self, uri: &str) -> Result<Option<Playlist>, Error> {
        unimplemented!()
    }

    async fn stream_url(&self, track: &Track) -> Result<String, Error> {
        let id = self.get_youtube_id(&track.uri)?;
        let youtube_dl = YoutubeDl::default();
        let url = youtube_dl.get_audio_stream_url(id).await?;

        Ok(url)
    }

    async fn cover_art(&self, track: &Track) -> Result<Option<CoverArt>, Error> {
        if let Some(MetaValue::String(url)) = track.meta.get(META_YOUTUBE_DEFAULT_THUMBNAIL_URL) {
            Ok(Some(CoverArt::Url(url.clone())))
        } else {
            Ok(None)
        }
    }

    async fn resolve_share_url(&self, url: Url) -> Result<Option<InternalUri>, Error> {
        if let Some(captures) = YOUTUBE_VIDEO_REGEX.captures(url.as_str()) {
            let id = &captures[1];
            let internal_uri = format!("youtube://video/{}", id);

            Ok(Some(InternalUri::Track(internal_uri)))
        } else {
            Ok(None)
        }
    }
}
