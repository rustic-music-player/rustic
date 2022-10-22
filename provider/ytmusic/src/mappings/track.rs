use rustic_core::library::Lyrics;
use rustic_core::{Artist, ProviderType, Rating, Track};
use super::map_thumbnail;

pub fn map_track(track: ytmusic::BrowseSong) -> Track {
    let thumbnail = map_thumbnail(track.video_details.thumbnail.thumbnails);

    Track {
        id: None,
        provider: ProviderType::YouTubeMusic,
        uri: format!("ytmusic://track/{}", track.video_details.video_id),
        title: track.video_details.title,
        album: None,
        album_id: None,
        artist_id: None,
        artist: Some(Artist {
            id: None,
            name: track.video_details.author,
            description: None,
            provider: ProviderType::YouTubeMusic,
            meta: maplit::hashmap! {},
            uri: Default::default(),
            albums: vec![],
            playlists: vec![],
            image_url: None,
        }),
        duration: None,
        thumbnail,
        explicit: None,
        position: None,
        chapters: vec![],
        comments: None,
        lyrics: Lyrics::None,
        rating: Rating::None,
        share_url: None,
        meta: maplit::hashmap! {},
    }
}
