use std::collections::HashMap;

use rspotify::model::artist::{FullArtist, SimplifiedArtist};
use serde_derive::{Deserialize, Serialize};

use rustic_core::library::Artist;

use crate::util::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotifyFullArtist(FullArtist);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotifySimplifiedArtist(SimplifiedArtist);

impl From<SpotifyFullArtist> for Artist {
    fn from(artist: SpotifyFullArtist) -> Self {
        let artist = artist.0;
        Artist {
            id: None,
            name: artist.name,
            image_url: convert_images(&artist.images),
            uri: format!("spotify://artist/{}", artist.id),
            meta: HashMap::new(),
        }
    }
}

impl From<SpotifySimplifiedArtist> for Artist {
    fn from(artist: SpotifySimplifiedArtist) -> Self {
        let artist = artist.0;
        Artist {
            id: None,
            name: artist.name,
            image_url: None,
            uri: artist
                .id
                .map(|id| format!("spotify://artist/{}", id))
                .unwrap(),
            meta: HashMap::new(),
        }
    }
}

impl From<FullArtist> for SpotifyFullArtist {
    fn from(artist: FullArtist) -> Self {
        SpotifyFullArtist(artist)
    }
}

impl From<SimplifiedArtist> for SpotifySimplifiedArtist {
    fn from(artist: SimplifiedArtist) -> Self {
        SpotifySimplifiedArtist(artist)
    }
}
