use std::collections::HashMap;

use log::warn;
use maplit::hashmap;
use rspotify::model::track::{FullTrack, SimplifiedTrack};
use serde_derive::{Deserialize, Serialize};

use rustic_core::library::{Album, MetaValue, Track};
use rustic_core::provider;

use crate::meta::*;
use crate::util::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotifyFullTrack(FullTrack);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotifySimplifiedTrack(SimplifiedTrack);

impl From<SpotifyFullTrack> for Track {
    fn from(track: SpotifyFullTrack) -> Self {
        let track = track.0;
        let debug_track = track.clone();
        let artist = artists_to_artist(track.artists);

        let album = track.album.clone();

        let mut meta: HashMap<String, MetaValue> = hashmap!(
            META_SPOTIFY_URI.into() => track.uri.clone().into(),
        );

        if let Some(ref id) = track.id {
            meta.insert(META_SPOTIFY_ID.into(), id.clone().into());
        }

        if let Some(image) = convert_images(&track.album.images) {
            meta.insert(META_SPOTIFY_COVER_ART_URL.into(), image.into());
        }

        Track {
            id: None,
            title: track.name,
            artist_id: None,
            artist: artist.clone(),
            album_id: None,
            album: Some(Album {
                id: None,
                title: track.album.name,
                artist_id: None,
                artist,
                tracks: vec![],
                provider: provider::ProviderType::Spotify,
                image_url: convert_images(&track.album.images),
                uri: track
                    .album
                    .id
                    .map(|id| format!("spotify://album/{}", id))
                    .unwrap_or_else(|| {
                        warn!("album {:?} is missing album id", album);
                        format!("spotify://album/{}", &album.name)
                    }),
                meta: HashMap::new(),
            }),
            provider: provider::ProviderType::Spotify,
            has_coverart: !track.album.images.is_empty(),
            uri: track
                .id
                .map(|id| format!("spotify://track/{}", id))
                .unwrap_or_else(|| {
                    warn!("track {:?} is missing track id", debug_track);
                    format!("spotify://track/{}", &debug_track.uri)
                }),
            duration: Some(u64::from(track.duration_ms / 1000)),
            meta,
        }
    }
}

impl From<SpotifySimplifiedTrack> for Track {
    fn from(track: SpotifySimplifiedTrack) -> Self {
        let track = track.0;
        let debug_track = track.clone();
        let artist = artists_to_artist(track.artists);

        let mut meta: HashMap<String, MetaValue> = hashmap!(
            META_SPOTIFY_URI.into() => track.uri.clone().into(),
        );

        if let Some(ref id) = track.id {
            meta.insert(META_SPOTIFY_ID.into(), id.clone().into());
        }

        Track {
            id: None,
            title: track.name,
            artist_id: None,
            artist,
            album_id: None,
            album: None,
            provider: provider::ProviderType::Spotify,
            has_coverart: false,
            uri: track
                .id
                .map(|id| format!("spotify://track/{}", id))
                .unwrap_or_else(|| {
                    warn!("track {:?} is missing track id", debug_track);
                    format!("spotify://track/{}", &debug_track.uri)
                }),
            duration: Some(u64::from(track.duration_ms / 1000)),
            meta,
        }
    }
}

impl From<FullTrack> for SpotifyFullTrack {
    fn from(track: FullTrack) -> Self {
        SpotifyFullTrack(track)
    }
}

impl From<SimplifiedTrack> for SpotifySimplifiedTrack {
    fn from(track: SimplifiedTrack) -> Self {
        SpotifySimplifiedTrack(track)
    }
}
