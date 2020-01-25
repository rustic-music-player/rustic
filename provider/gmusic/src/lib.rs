use std::convert::TryFrom;
use std::sync::Mutex;

use failure::{format_err, Error};
use log::{debug, error};
use serde_derive::Deserialize;

use gmusic::{auth::stdio_login, GoogleMusicApi};
use lazy_static::lazy_static;
use rustic_core::library::MetaValue;
use rustic_core::{provider, Album, Playlist, SharedLibrary, Track};

use crate::album::GmusicAlbum;
use crate::meta::{META_GMUSIC_COVER_ART_URL, META_GMUSIC_STORE_ID};
use crate::playlist::GmusicPlaylist;
use crate::track::GmusicTrack;

mod album;
mod meta;
mod playlist;
mod track;

lazy_static! {
    static ref PLAY_MUSIC_REGEX: regex::Regex = regex::RegexBuilder::new("^/music/m/([0-9a-z]+)")
        .case_insensitive(true)
        .build()
        .unwrap();
    static ref PLAY_MUSIC_PLAYLIST_REGEX: regex::Regex =
        regex::RegexBuilder::new("^/music/playlist/([0-9a-z]+)")
            .case_insensitive(true)
            .build()
            .unwrap();
    static ref STATE_CACHE: Mutex<Option<String>> = Mutex::new(None);
}

// TODO: configurable host
const GMUSIC_REDIRECT_URI: &str = "http://localhost:8080/api/providers/gmusic/auth/redirect";

#[derive(Clone, Deserialize, Debug)]
pub struct GooglePlayMusicProvider {
    client_id: String,
    client_secret: String,
    device_id: String,
    #[serde(skip)]
    client: Option<GoogleMusicApi>,
}

impl GooglePlayMusicProvider {
    fn get_client(&self) -> Result<&GoogleMusicApi, Error> {
        self.client
            .as_ref()
            .ok_or_else(|| format_err!("Provider Google Play Music is not setup yet"))
    }
}

impl provider::ProviderInstance for GooglePlayMusicProvider {
    fn setup(&mut self) -> Result<(), Error> {
        let api = GoogleMusicApi::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            Some(GMUSIC_REDIRECT_URI),
        )?;
        api.load_token();
        self.client = Some(api);

        Ok(())
    }

    fn auth_state(&self) -> provider::AuthState {
        let client = self.client.as_ref().expect("client isn't setup yet");
        if client.has_token() {
            provider::AuthState::Authenticated(None)
        } else {
            let (url, state) = client.get_oauth_url();
            let mut state_cache = STATE_CACHE.lock().unwrap();
            *state_cache = Some(state);
            provider::AuthState::RequiresOAuth(url)
        }
    }

    fn authenticate(&mut self, authenticate: provider::Authentication) -> Result<(), Error> {
        let client = self.client.as_mut().expect("client isn't setup yet");
        use provider::Authentication::*;

        match authenticate {
            Token(token) => {
                let mut state_cache = STATE_CACHE.lock().unwrap();
                let state = state_cache
                    .take()
                    .ok_or_else(|| format_err!("Missing state"))?;
                debug!("State: {}", state);
                client.request_token(token, state)?;
                client.store_token()?;
                Ok(())
            }
            _ => Err(format_err!("Invalid authentication method")),
        }
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

    fn sync(&self, library: SharedLibrary) -> Result<provider::SyncResult, Error> {
        let client = self.get_client()?;
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
        let client = self.get_client()?;
        let results = client.search(&query, None)?;
        let items = results
            .into_iter()
            .flat_map(|cluster| cluster.entries.into_iter().map(GoogleSearchResult::from))
            .map(provider::ProviderItem::try_from)
            .filter_map(|result| result.ok())
            .collect();

        Ok(items)
    }

    fn resolve_track(&self, uri: &str) -> Result<Option<Track>, Error> {
        let client = self.get_client()?;
        let track_id = &uri["gmusic:track:".len()..];
        let track = client.get_store_track(track_id)?;
        let track = GmusicTrack::from(track);
        let track = Track::from(track);
        Ok(Some(track))
    }

    fn resolve_album(&self, uri: &str) -> Result<Option<Album>, Error> {
        let client = self.get_client()?;
        let album_id = &uri["gmusic:album:".len()..];
        let album = client.get_album(album_id)?;
        let album = GmusicAlbum::from(album);
        let album = Album::from(album);
        Ok(Some(album))
    }

    fn resolve_playlist(&self, _uri: &str) -> Result<Option<Playlist>, Error> {
        unimplemented!()
    }

    fn stream_url(&self, track: &Track) -> Result<String, Error> {
        let id = track
            .meta
            .get(META_GMUSIC_STORE_ID)
            .ok_or_else(|| format_err!("missing track id"))?;
        if let MetaValue::String(ref id) = id {
            let client = self.get_client()?;
            let url = client.get_stream_url(&id, &self.device_id)?;
            Ok(url.to_string())
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
                _ => unreachable!(),
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

            let entity = if id.starts_with("T") {
                // Track
                Some(provider::InternalUri::Track(format!("gmusic:track:{}", id)))
            } else if id.starts_with("B") {
                // Album
                Some(provider::InternalUri::Album(format!("gmusic:album:{}", id)))
            } else if id.starts_with("A") {
                // Artist
                Some(provider::InternalUri::Artist(format!(
                    "gmusic:artist:{}",
                    id
                )))
            } else {
                None
            };

            Ok(entity)
        } else if let Some(captures) = PLAY_MUSIC_PLAYLIST_REGEX.captures(url.path()) {
            let id = &captures[1];

            Ok(Some(provider::InternalUri::Playlist(format!(
                "gmusic://playlist/{}",
                id
            ))))
        } else {
            Ok(None)
        }
    }
}

struct GoogleSearchResult(gmusic::SearchResult);

impl From<gmusic::SearchResult> for GoogleSearchResult {
    fn from(result: gmusic::SearchResult) -> Self {
        GoogleSearchResult(result)
    }
}

impl TryFrom<GoogleSearchResult> for provider::ProviderItem {
    type Error = ();

    fn try_from(result: GoogleSearchResult) -> Result<Self, Self::Error> {
        let result = result.0;
        if let Some(track) = result.track {
            let track = GmusicTrack::from(track);
            let track = Track::from(track);
            Ok(provider::ProviderItem::from(track))
        } else if let Some(playlist) = result.playlist {
            let playlist = GmusicPlaylist::from(playlist);
            let playlist = Playlist::from(playlist);
            Ok(provider::ProviderItem::from(playlist))
        } else if let Some(album) = result.album {
            let album = GmusicAlbum::from(album);
            let album = Album::from(album);
            Ok(provider::ProviderItem::from(album))
        } else {
            Err(())
        }
    }
}
