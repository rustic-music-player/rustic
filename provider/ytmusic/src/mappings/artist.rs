use rustic_core::{Album, Artist, ProviderType};
use super::{map_playlist_item, map_thumbnail};

pub fn map_artist(artist: ytmusic::BrowseArtist) -> Artist {
    Artist {
        id: None,
        name: artist.name,
        provider: ProviderType::YouTubeMusic,
        uri: format!("ytmusic://artist/{}", artist.id),
        playlists: Default::default(),
        albums: Default::default(),
        image_url: artist.thumbnails.into_iter().max_by_key(|thumb| thumb.width).map(|thumb| thumb.url),
        description: None,
        meta: maplit::hashmap! {},
    }
}
