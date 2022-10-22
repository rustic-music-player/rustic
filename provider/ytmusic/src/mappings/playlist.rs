use rustic_core::library::Lyrics;
use rustic_core::{Album, Artist, Playlist, ProviderType, Rating, Track};
use rustic_core::provider::ThumbnailState;
use super::map_thumbnail;

pub fn map_playlist(playlist: ytmusic::Playlist) -> Playlist {
    Playlist {
        id: None,
        title: playlist.title,
        provider: ProviderType::YouTubeMusic,
        uri: format!("ytmusic://playlist/{}", playlist.id),
        tracks: playlist.tracks.into_iter()
            .map(map_playlist_item)
            .collect(),
    }
}

pub fn map_playlist_item(track: ytmusic::PlaylistItem) -> Track {
    let thumbnail = map_thumbnail(track.thumbnails);

    Track {
        id: None,
        provider: ProviderType::YouTubeMusic,
        uri: format!("ytmusic://track/{}", track.video_id),
        title: track.title,
        album: track.album.map(|album| Album {
            id: None,
            title: album.name,
            provider: ProviderType::YouTubeMusic,
            uri: format!("ytmusic://album/{}", album.id),
            tracks: Default::default(),
            artist: None,
            artist_id: None,
            description: None,
            thumbnail: ThumbnailState::None,
            explicit: None,
            meta: maplit::hashmap! {},
        }),
        album_id: None,
        artist_id: None,
        artist: track.artists.first()
            .map(|artist| Artist {
                id: None,
                name: artist.name.clone(),
                description: None,
                provider: ProviderType::YouTubeMusic,
                meta: maplit::hashmap! {},
                uri: format!("ytmusic://artist/{}", artist.id),
                albums: vec![],
                playlists: vec![],
                image_url: None,
            }),
        duration: track.duration_seconds,
        thumbnail,
        explicit: Some(track.is_explicit),
        position: None,
        chapters: vec![],
        comments: None,
        lyrics: Lyrics::None,
        rating: Rating::None,
        share_url: None,
        meta: maplit::hashmap! {},
    }
}
