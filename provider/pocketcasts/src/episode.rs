use maplit::hashmap;
use pocketcasts::Episode;
use serde::{Deserialize, Serialize};

use rustic_core::library::Track;
use rustic_core::provider::ProviderType;

use crate::meta::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PocketcastTrack(Episode);

impl From<PocketcastTrack> for Track {
    fn from(episode: PocketcastTrack) -> Track {
        let episode = episode.0;
        Track {
            id: None,
            title: episode.title,
            artist_id: None,
            artist: None,
            album_id: None,
            album: None,
            provider: ProviderType::Pocketcasts,
            uri: format!("pocketcasts://episode/{}", episode.uuid),
            has_coverart: false,
            duration: Some(episode.duration),
            meta: hashmap!(
                META_POCKETCASTS_STREAM_URL.into() => episode.url.into()
            ),
        }
    }
}

impl From<Episode> for PocketcastTrack {
    fn from(episode: Episode) -> Self {
        PocketcastTrack(episode)
    }
}
