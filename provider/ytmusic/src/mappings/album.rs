use rustic_core::{Album, Artist, ProviderType};
use super::{map_playlist_item, map_thumbnail};

pub fn map_album(album: ytmusic::BrowseAlbum) -> Album {
    let thumbnail = map_thumbnail(album.thumbnails);

    Album {
        id: None,
        title: album.title,
        provider: ProviderType::YouTubeMusic,
        uri: format!("ytmusic://album/{}", album.id),
        tracks: album.tracks.into_iter()
            .map(map_playlist_item)
            .map(|mut track| {
                track.thumbnail = thumbnail.clone();

                track
            })
            .collect(),
        artist: album.artists.first().map(|artist| Artist {
            id: None,
            name: artist.name.clone(),
            uri: format!("ytmusic://artist/{}", artist.id),
            provider: ProviderType::YouTubeMusic,
            playlists: vec![],
            albums: vec![],
            image_url: None,
            description: None,
            meta: maplit::hashmap! {},
        }),
        artist_id: None,
        description: album.description,
        thumbnail,
        explicit: None,
        meta: maplit::hashmap! {},
    }
}
