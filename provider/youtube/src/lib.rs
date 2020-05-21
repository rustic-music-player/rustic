use crate::meta::META_YOUTUBE_DEFAULT_THUMBNAIL_URL;
use crate::video_metadata::YoutubeVideoMetadata;
use async_trait::async_trait;
use failure::{ensure, Error};
use lazy_static::lazy_static;
use log::warn;
use rustic_core::library::MetaValue;
use rustic_core::provider::{
    AuthState, Authentication, CoverArt, InternalUri, ProviderFolder, ProviderInstance,
    ProviderItem, SyncResult,
};
use rustic_core::{Album, Playlist, ProviderType, SharedLibrary, Track};
use serde::Deserialize;
use url::Url;
use youtube::{YoutubeApi, YoutubeDl};
use youtube::models::SearchRequestBuilder;
use crate::search_result::YoutubeSearchResult;

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
}

const VIDEO_URI_PREFIX: &str = "youtube://video/";

#[derive(Debug, Clone, Deserialize)]
pub struct YoutubeProvider {
    api_key: String,
    #[serde(skip)]
    client: Option<YoutubeApi>
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
        self.client = Some(YoutubeApi::new(&self.api_key));
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
        AuthState::Authenticated(None)
    }

    async fn authenticate(&mut self, auth: Authentication) -> Result<(), Error> {
        unimplemented!()
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
