use std::collections::HashMap;

use rustic_core::{Artist, ProviderType, Album, Playlist, Track};
use crate::album::GmusicAlbum;
use crate::track::GmusicTrack;

#[derive(Debug, Clone)]
pub struct GmusicArtist(gmusic::Artist);

impl From<gmusic::Artist> for GmusicArtist {
    fn from(artist: gmusic::Artist) -> Self {
        GmusicArtist(artist)
    }
}

impl From<GmusicArtist> for Artist {
    fn from(artist: GmusicArtist) -> Self {
        let artist = artist.0;
        Artist {
            id: None,
            name: artist.name,
            uri: format!("gmusic:artist:{}", artist.id),
            image_url: artist.artist_art_ref,
            provider: ProviderType::GooglePlayMusic,
            meta: HashMap::new(),
            albums: artist.albums.into_iter()
                .map(GmusicAlbum::from)
                .map(Album::from)
                .collect(),
            playlists: vec![Playlist {
                title: "Top Tracks".into(),
                tracks: artist.top_tracks.into_iter()
                    .map(GmusicTrack::from)
                    .map(Track::from)
                    .collect(),
                provider: ProviderType::GooglePlayMusic,
                id: None,
                uri: format!("gmusic:artist:top:{}", artist.id)
            }]
        }
    }
}
