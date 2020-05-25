use youtube::models::SearchResult;
use rustic_core::provider::{ProviderItem, ProviderItemType};
use rustic_core::{Track, ProviderType, Artist};
use std::collections::HashMap;
use crate::meta::META_YOUTUBE_DEFAULT_THUMBNAIL_URL;

#[derive(Clone)]
pub struct YoutubeSearchResult(SearchResult);

impl YoutubeSearchResult {
    fn into_inner(self) -> SearchResult {
        self.0
    }
}

impl From<SearchResult> for YoutubeSearchResult {
    fn from(result: SearchResult) -> Self {
        YoutubeSearchResult(result)
    }
}

impl From<YoutubeSearchResult> for ProviderItem {
    fn from(result: YoutubeSearchResult) -> Self {
        let track = result.clone().into();
        let result = result.into_inner();

        ProviderItem {
            label: result.snippet.title.clone(),
            data: ProviderItemType::Track(track)
        }
    }
}

impl From<YoutubeSearchResult> for Track {
    fn from(result: YoutubeSearchResult) -> Self {
        let result = result.into_inner();
        let mut meta = HashMap::new();
        let thumbnail = result.snippet.thumbnails.get("high");
        if let Some(thumbnail) = thumbnail.as_ref() {
            meta.insert(
                META_YOUTUBE_DEFAULT_THUMBNAIL_URL.into(),
                thumbnail.url.clone().into(),
            );
        }

        Track {
            id: None,
            title: result.snippet.title,
            uri: format!("youtube://video/{}", result.id.into_inner()),
            duration: None,
            has_coverart: true,
            provider: ProviderType::Youtube,
            artist_id: None,
            artist: Some(Artist {
                meta: HashMap::new(),
                id: None,
                uri: format!("youtube://author/{}", &result.snippet.channel_id),
                name: result.snippet.channel_title,
                image_url: None,
                provider: ProviderType::Youtube,
            }),
            album: None,
            album_id: None,
            meta
        }
    }
}
