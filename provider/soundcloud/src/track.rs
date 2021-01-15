use maplit::hashmap;

use rustic_core::library::{Lyrics, Track};
use rustic_core::{provider, Rating};

use crate::meta::*;
use crate::user::SoundcloudUser;
use rustic_core::provider::ThumbnailState;

#[derive(Debug, Clone)]
pub struct SoundcloudTrack(soundcloud::Track);

impl From<SoundcloudTrack> for Track {
    fn from(track: SoundcloudTrack) -> Track {
        let track = track.0;
        let stream_url = track.stream_url.unwrap();

        let meta = hashmap! {
            META_SOUNDCLOUD_TRACK_ID.into() => track.id.into(),
            META_SOUNDCLOUD_STREAM_URL.into() => stream_url.into()
        };

        Track {
            id: None,
            title: track.title,
            artist: Some(SoundcloudUser::from(track.user).into()),
            artist_id: None,
            album: None,
            album_id: None,
            provider: provider::ProviderType::Soundcloud,
            uri: format!("soundcloud://track/{}", track.id),
            thumbnail: track
                .artwork_url
                .map(|url| url.replace("large", "t500x500"))
                .map(ThumbnailState::Url)
                .unwrap_or_default(),
            duration: Some(track.duration / 1000),
            meta,
            explicit: None,
            rating: Rating::None,
            position: None,
            share_url: Some(track.permalink_url),
            comments: track.description,
            lyrics: Lyrics::None,
            chapters: Vec::new(),
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
