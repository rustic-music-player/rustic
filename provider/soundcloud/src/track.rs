use maplit::hashmap;

use rustic_core::library::Track;
use rustic_core::provider;

use crate::meta::*;
use crate::user::SoundcloudUser;

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
            meta.insert(
                META_SOUNDCLOUD_COVER_ART_URL.into(),
                image_url.clone().into(),
            );
        }

        Track {
            id: None,
            title: track.title,
            artist: Some(SoundcloudUser::from(track.user).into()),
            artist_id: None,
            album: None,
            album_id: None,
            provider: provider::ProviderType::Soundcloud,
            uri: format!("soundcloud://track/{}", track.id),
            has_coverart: track.artwork_url.is_some(),
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
