use rustic_core::provider::ThumbnailState;
use rustic_core::{Artist, ProviderType, Track, Rating};
use std::collections::HashMap;
use std::str::FromStr;
use youtube_api::models;
use rustic_core::library::Lyrics;

pub(crate) struct YoutubeVideoMetadata(models::VideoMetadata);

impl YoutubeVideoMetadata {
    fn into_inner(self) -> models::VideoMetadata {
        self.0
    }
}

impl From<models::VideoMetadata> for YoutubeVideoMetadata {
    fn from(content: models::VideoMetadata) -> Self {
        YoutubeVideoMetadata(content)
    }
}

impl From<YoutubeVideoMetadata> for Track {
    fn from(video: YoutubeVideoMetadata) -> Self {
        let video = video.into_inner();
        let thumbnail = video.video_details.thumbnail.thumbnails.last();
        let thumbnail = if let Some(thumbnail) = thumbnail.as_ref() {
            ThumbnailState::Url(thumbnail.url.clone())
        } else {
            ThumbnailState::None
        };
        Track {
            title: video.video_details.title,
            artist: Some(Artist {
                uri: format!("youtube://channel/{}", &video.video_details.channel_id),
                name: video.video_details.author,
                id: None,
                image_url: None,
                meta: HashMap::new(),
                provider: ProviderType::Youtube,
                albums: vec![],
                playlists: vec![],
                description: None,
            }),
            album: None,
            album_id: None,
            artist_id: None,
            provider: ProviderType::Youtube,
            uri: format!("youtube://video/{}", video.video_details.video_id),
            thumbnail,
            id: None,
            duration: u64::from_str(&video.video_details.length_seconds).ok(),
            meta: HashMap::new(),
            explicit: None,
            rating: Rating::None,
            position: None,
            share_url: Some(format!("https://youtube.com/watch?v={}", &video.video_details.video_id)),
            comments: Some(video.video_details.short_description),
            lyrics: Lyrics::None,
            chapters: Vec::new(),
        }
    }
}
