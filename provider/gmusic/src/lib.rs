use failure::{Error, format_err};
use serde_derive::Deserialize;

use gmusic::{auth::stdio_login, GoogleMusicApi};
use rustic_core::{Playlist, provider, SharedLibrary, Track};
use rustic_core::library::MetaValue;

use crate::meta::{META_GMUSIC_STORE_ID, META_GMUSIC_TRACK_ID, META_GMUSIC_COVER_ART_URL};
use crate::playlist::GmusicPlaylist;
use crate::track::GmusicTrack;

mod meta;
mod playlist;
mod track;

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
                _ => unreachable!()
            })
            .map(|url| url.into());

        Ok(url)
    }
}
