use youtube_api::models::{SearchResult, Id};
use rustic_core::provider::{ProviderItem, ProviderItemType};
use rustic_core::{Track, ProviderType, Artist, Playlist};
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
        let search_result = result.clone();
        let result = result.into_inner();

        let data = match result.id {
            Id::PlaylistId { playlist_id: _ } => ProviderItemType::Playlist(search_result.into()),
            Id::ChannelId { channel_id: _ } => ProviderItemType::Artist(search_result.into()),
            Id::VideoId { video_id: _ } => ProviderItemType::Track(search_result.into())
        };

        ProviderItem {
            label: result.snippet.title,
            data
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
                albums: vec![],
                playlists: vec![]
            }),
            album: None,
            album_id: None,
            meta
        }
    }
}

impl From<YoutubeSearchResult> for Artist {
    fn from(result: YoutubeSearchResult) -> Self {
        let result = result.into_inner();
        let thumbnail = result.snippet.thumbnails.get("high");

        Artist {
            id: None,
            name: result.snippet.title,
            uri: format!("youtube://author/{}", result.id.into_inner()),
            provider: ProviderType::Youtube,
            playlists: Vec::new(),
            albums: Vec::new(),
            image_url: thumbnail.map(|thumbnail| thumbnail.url.clone()),
            meta: HashMap::new()
        }
    }
}

impl From<YoutubeSearchResult> for Playlist {
    fn from(result: YoutubeSearchResult) -> Self {
        let result = result.into_inner();

        Playlist {
            id: None,
            title: result.snippet.title,
            uri: format!("youtube://playlist/{}", result.id.into_inner()),
            provider: ProviderType::Youtube,
            tracks: Vec::new()
        }
    }
}
