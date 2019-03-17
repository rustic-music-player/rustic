use pocketcasts::Episode;
use rustic::library::Track;
use rustic::provider::Provider;

use meta::*;

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
            provider: Provider::Pocketcasts,
            uri: format!("pocketcasts://episode/{}", episode.uuid),
            image_url: None,
            duration: Some(episode.duration),
            meta: hashmap!(
                META_POCKETCASTS_STREAM_URL => episode.url.into()
            )
        }
    }
}

impl From<Episode> for PocketcastTrack {
    fn from(episode: Episode) -> Self {
        PocketcastTrack(episode)
    }
}