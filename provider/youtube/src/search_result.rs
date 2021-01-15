use rustic_core::library::Lyrics;
use rustic_core::provider::{ProviderItem, ProviderItemType, ThumbnailState};
use rustic_core::{Artist, Playlist, ProviderType, Rating, Track};
use std::collections::HashMap;
use youtube_api::models::{Id, SearchResult};

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
            Id::VideoId { video_id: _ } => ProviderItemType::Track(search_result.into()),
        };

        ProviderItem {
            label: result.snippet.title,
            data,
        }
    }
}

impl From<YoutubeSearchResult> for Track {
    fn from(result: YoutubeSearchResult) -> Self {
        let result = result.into_inner();
        let thumbnail = result.snippet.thumbnails.get("high");
        let thumbnail = if let Some(thumbnail) = thumbnail.as_ref() {
            ThumbnailState::Url(thumbnail.url.clone())
        } else {
            ThumbnailState::None
        };

        let id = result.id.into_inner();
        Track {
            id: None,
            title: result.snippet.title,
            uri: format!("youtube://video/{}", &id),
            duration: None,
            thumbnail,
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
                playlists: vec![],
                description: None,
            }),
            album: None,
            album_id: None,
            meta: HashMap::new(),
            explicit: None,
            rating: Rating::None,
            position: None,
            share_url: Some(format!("https://youtube.com/watch?v={}", &id)),
            comments: Some(result.snippet.description),
            lyrics: Lyrics::None,
            chapters: Vec::new(),
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
            meta: HashMap::new(),
            description: Some(result.snippet.description),
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
            tracks: Vec::new(),
        }
    }
}
