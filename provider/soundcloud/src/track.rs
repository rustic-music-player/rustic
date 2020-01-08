use soundcloud;

use rustic::library::{Artist, Track};
use rustic::provider;

use crate::meta::*;

#[derive(Debug, Clone)]
pub struct SoundcloudTrack(soundcloud::Track);

impl From<SoundcloudTrack> for Track {
    fn from(track: SoundcloudTrack) -> Track {
        let track = track.0;
        let stream_url = track.stream_url.unwrap();

        let mut meta = hashmap! {
            META_SOUNDCLOUD_TRACK_ID.into() => track.id.into(),
            META_SOUNDCLOUD_STREAM_URL.into() => stream_url.into()
        };

        if let Some(image_url) = track.artwork_url.as_ref() {
            meta.insert(META_SOUNDCLOUD_COVER_ART_URL.into(), image_url.clone().into());
        }

        Track {
            id: None,
            title: track.title,
            artist: Some(Artist {
                id: None,
                name: track.user.username,
                image_url: Some(track.user.avatar_url),
                uri: format!("soundcloud://user/{}", track.user.id),
                meta: hashmap!(
                    META_SOUNDCLOUD_USER_ID.into() => track.user.id.into()
                ),
            }),
            artist_id: None,
            album: None,
            album_id: None,
            provider: provider::Provider::Soundcloud,
            uri: format!("soundcloud://track/{}", track.id),
            image_url: track.artwork_url,
            duration: Some(track.duration / 1000),
            meta,
        }
    }
}

impl From<SoundcloudTrack> for provider::ProviderItem {
    fn from(track: SoundcloudTrack) -> provider::ProviderItem {
        provider::ProviderItem::from(Track::from(track))
    }
}

impl From<soundcloud::Track> for SoundcloudTrack {
    fn from(track: soundcloud::Track) -> Self {
        SoundcloudTrack(track)
    }
}
