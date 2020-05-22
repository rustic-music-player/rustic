use youtube::models::PlaylistItemResource;

use rustic_core::{Artist, ProviderType, Track};
use std::collections::HashMap;
use crate::meta::META_YOUTUBE_DEFAULT_THUMBNAIL_URL;

#[derive(Clone)]
pub struct YoutubePlaylistItem(PlaylistItemResource);

impl YoutubePlaylistItem {
    fn into_inner(self) -> PlaylistItemResource {
        self.0
    }
}

impl From<PlaylistItemResource> for YoutubePlaylistItem {
    fn from(result: PlaylistItemResource) -> Self {
        YoutubePlaylistItem(result)
    }
}

impl From<YoutubePlaylistItem> for Track {
    fn from(item: YoutubePlaylistItem) -> Self {
        let resource = item.into_inner();
        let thumbnail = resource.snippet.inner.thumbnails.get("high");
        let mut meta = HashMap::new();
        if let Some(thumbnail) = thumbnail.as_ref() {
            meta.insert(
                META_YOUTUBE_DEFAULT_THUMBNAIL_URL.into(),
                thumbnail.url.clone().into(),
            );
        }

        Track {
            id: None,
            provider: ProviderType::Youtube,
            title: resource.snippet.inner.title,
            uri: format!("youtube://video/{}", &resource.snippet.resource_id.into_inner()),
            album_id: None,
            album: None,
            artist_id: None,
            artist: Some(Artist {
                id: None,
                // TODO: this is the playlist owner
                uri: format!("youtube://channel/{}", resource.snippet.inner.channel_id),
                name: resource.snippet.inner.channel_title,
                image_url: None,
                meta: HashMap::new()
            }),
            has_coverart: true,
            duration: None,
            meta
        }
    }
}
