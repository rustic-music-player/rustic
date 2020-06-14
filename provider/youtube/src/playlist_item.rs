use youtube_api::models::PlaylistItemResource;

use rustic_core::{Artist, ProviderType, Track};
use std::collections::HashMap;
use rustic_core::provider::ThumbnailState;

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
        let thumbnail = if let Some(thumbnail) = thumbnail.as_ref() {
            ThumbnailState::Url(thumbnail.url.clone())
        }else {
            ThumbnailState::None
        };

        Track {
            id: None,
            provider: ProviderType::Youtube,
            title: resource.snippet.inner.title,
            uri: format!(
                "youtube://video/{}",
                &resource.snippet.resource_id.into_inner()
            ),
            album_id: None,
            album: None,
            artist_id: None,
            artist: Some(Artist {
                id: None,
                // TODO: this is the playlist owner
                uri: format!("youtube://channel/{}", resource.snippet.inner.channel_id),
                name: resource.snippet.inner.channel_title,
                image_url: None,
                meta: HashMap::new(),
                provider: ProviderType::Youtube,
                albums: vec![],
                playlists: vec![],
            }),
            thumbnail,
            duration: None,
            meta: HashMap::new(),
        }
    }
}
