use failure::{Error, format_err};
use gmusic::{auth::stdio_login, GoogleMusicApi};
use log::error;
use serde_derive::Deserialize;

use lazy_static::lazy_static;
use rustic_core::{Playlist, provider, SharedLibrary, Track};
use rustic_core::library::MetaValue;

use crate::meta::{META_GMUSIC_COVER_ART_URL, META_GMUSIC_STORE_ID, META_GMUSIC_TRACK_ID};
use crate::playlist::GmusicPlaylist;
use crate::track::GmusicTrack;

mod meta;
mod playlist;
mod track;

lazy_static! {
    static ref PLAY_MUSIC_REGEX: regex::Regex = regex::RegexBuilder::new("^/music/m/([0-9a-z]+)")
            .case_insensitive(true)
            .build()
            .unwrap();
    static ref PLAY_MUSIC_PLAYLIST_REGEX: regex::Regex = regex::RegexBuilder::new("^/music/playlist/([0-9a-z]+)")
            .case_insensitive(true)
            .build()
            .unwrap();
}

#[derive(Clone, Deserialize, Debug)]
pub struct GooglePlayMusicProvider {
    client_id: String,
    client_secret: String,
    device_id: String,
    #[serde(skip)]
    client: Option<GoogleMusicApi>,
}

impl provider::ProviderInstance for GooglePlayMusicProvider {
    fn setup(&mut self) -> Result<(), Error> {
        let api = GoogleMusicApi::new(self.client_id.clone(), self.client_secret.clone())?;
        api.load_token()
            .or_else(|_| api.login(stdio_login).and_then(|_| api.store_token()))?;
        self.client = Some(api);

        Ok(())
    }

    fn title(&self) -> &'static str {
        "Google Play Music"
    }

    fn uri_scheme(&self) -> &'static str {
        "gmusic"
    }

    fn provider(&self) -> provider::Provider {
        provider::Provider::GooglePlayMusic
    }

    fn sync(&mut self, library: SharedLibrary) -> Result<provider::SyncResult, Error> {
        let client = self.client.as_ref().unwrap();
        let mut playlists: Vec<Playlist> = client
            .get_all_playlists()?
            .into_iter()
            .map(GmusicPlaylist::from)
            .map(Playlist::from)
            .collect();

        let playlist_entries = client.get_playlist_entries()?;

        for playlist in &mut playlists {
            let playlist_id = &playlist.uri["gmusic:playlist:".len()..];
            playlist.tracks = playlist_entries
                .iter()
                .filter(|entry| entry.playlist_id == playlist_id)
                .filter_map(|entry| entry.track.as_ref())
                .cloned()
                .map(GmusicTrack::from)
                .map(Track::from)
                .collect();
        }

        library.sync_playlists(&mut playlists)?;

        Ok(provider::SyncResult {
            tracks: 0,
            artists: 0,
            albums: 0,
            playlists: playlists.len(),
        })
    }

    fn root(&self) -> provider::ProviderFolder {
        provider::ProviderFolder {
            folders: vec![],
            items: vec![],
        }
    }

    fn navigate(&self, _path: Vec<String>) -> Result<provider::ProviderFolder, Error> {
        Ok(self.root())
    }

    fn search(&self, query: String) -> Result<Vec<provider::ProviderItem>, Error> {
        unimplemented!()
    }

    fn resolve_track(&self, uri: &str) -> Result<Option<Track>, Error> {
        let client = self.client.as_ref().unwrap();
        let track_id = &uri["gmusic:track:".len()..];
        let track = client.get_store_track(track_id)?;
        let track = GmusicTrack::from(track);
        let track = Track::from(track);
        Ok(Some(track))
    }

    fn stream_url(&self, track: &Track) -> Result<String, Error> {
        let id = track
            .meta
            .get(META_GMUSIC_STORE_ID)
            .ok_or_else(|| format_err!("missing track id"))?;
        if let MetaValue::String(ref id) = id {
            let client = self.client.as_ref().unwrap();
            // HACK: sometimes gmusic returns 403, not sure why but retrying fixes it most of the time
            let mut url = None;
            for _ in 0..2 {
                match client.get_stream_url(&id, &self.device_id) {
                    Ok(stream_url) => {
                        url = Some(stream_url);
                        break;
                    }
                    Err(err) => error!("Stream Url {:?}", err)
                }
            }
            url.ok_or_else(|| format_err!("Could not get stream url"))
                .map(|url| url.to_string())
        } else {
            unreachable!()
        }
    }

    fn cover_art(&self, track: &Track) -> Result<Option<provider::CoverArt>, Error> {
        let url = track
            .meta
            .get(META_GMUSIC_COVER_ART_URL)
            .map(|value| match value {
                MetaValue::String(url) => url.clone(),
                _ => unreachable!()
            })
            .map(|url| url.into());

        Ok(url)
    }

    /// # Track
    /// https://play.google.com/music/m/Tq2fjpqmhnx2srvhe5batazevwy?t=Never_Gonna_Give_You_Up_-_Rick_Astley
    /// # Album
    /// https://play.google.com/music/m/B2fhxqwa7avbghszalvxpkqooh4?t=Whenever_You_Need_Somebody_-_Rick_Astley
    /// # Artist
    /// https://play.google.com/music/m/Aubs6vvsgfmxs5v4qnqkyvk37gi?t=Rick_Astley
    /// # Playlist
    /// https://play.google.com/music/playlist/AMaBXymwfjrAK6klYX211rdx5PPNaiqcV1GlfH2OF5DbJMCmnsgFLt5pR6VJ9S8hJiy1vzDdFVFHus05mf0HnZHQI9u8nYyFVw==
    fn resolve_share_url(&self, url: url::Url) -> Result<Option<provider::InternalUri>, Error> {
        if url.domain() != Some("play.google.com") {
            return Ok(None);
        }
        if let Some(captures) = dbg!(PLAY_MUSIC_REGEX.captures(dbg!(url.path()))) {
            let id = &captures[1];

            let entity = if id.starts_with("T") { // Track
                Some(provider::InternalUri::Track(format!("gmusic://track/{}", id)))
            } else if id.starts_with("B") { // Album
                Some(provider::InternalUri::Album(format!("gmusic://album/{}", id)))
            } else if id.starts_with("A") { // Artist
                Some(provider::InternalUri::Artist(format!("gmusic://artist/{}", id)))
            } else { None };

            Ok(entity)
        } else if let Some(captures) = PLAY_MUSIC_PLAYLIST_REGEX.captures(url.path()) {
            let id = &captures[1];

            Ok(Some(provider::InternalUri::Playlist(format!("gmusic://playlist/{}", id))))
        } else {
            Ok(None)
        }
    }
}
