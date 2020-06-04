use youtube_api::models::PlaylistResource;

use crate::playlist_item::YoutubePlaylistItem;
use rustic_core::{Playlist, ProviderType, Track};

#[derive(Clone)]
pub struct YoutubePlaylist(PlaylistResource);

impl YoutubePlaylist {
    fn into_inner(self) -> PlaylistResource {
        self.0
    }
}

impl From<PlaylistResource> for YoutubePlaylist {
    fn from(result: PlaylistResource) -> Self {
        YoutubePlaylist(result)
    }
}

pub(crate) struct PlaylistWithItems(pub YoutubePlaylist, pub Vec<YoutubePlaylistItem>);

impl From<PlaylistWithItems> for Playlist {
    fn from(playlist: PlaylistWithItems) -> Self {
        let resource = playlist.0.into_inner();
        let tracks = playlist.1.into_iter().map(Track::from).collect();

        Playlist {
            id: None,
            tracks,
            provider: ProviderType::Youtube,
            title: resource.snippet.title,
            uri: format!("youtube://playlist/{}", &resource.id),
        }
    }
}
