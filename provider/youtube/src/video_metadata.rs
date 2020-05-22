use crate::meta::META_YOUTUBE_DEFAULT_THUMBNAIL_URL;
use rustic_core::{Artist, ProviderType, Track};
use std::collections::HashMap;
use std::str::FromStr;
use youtube::models;

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
        let mut meta = HashMap::new();
        if let Some(thumbnail) = thumbnail.as_ref() {
            meta.insert(
                META_YOUTUBE_DEFAULT_THUMBNAIL_URL.into(),
                thumbnail.url.clone().into(),
            );
        }
        Track {
            title: video.video_details.title,
            artist: Some(Artist {
                uri: format!("youtube://channel/{}", &video.video_details.channel_id),
                name: video.video_details.author,
                id: None,
                image_url: None,
                meta: HashMap::new(),
            }),
            album: None,
            album_id: None,
            artist_id: None,
            provider: ProviderType::Youtube,
            uri: format!("youtube://video/{}", video.video_details.video_id),
            has_coverart: thumbnail.is_some(),
            id: None,
            duration: u64::from_str(&video.video_details.length_seconds).ok(),
            meta,
        }
    }
}
