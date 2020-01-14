use std::thread;

use failure::{err_msg, Error, format_err};
use log::{debug, trace};
use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::spotify::util::get_token;
use serde_derive::Deserialize;

use rustic_core::library::{Album, Artist, MetaValue, Playlist, SharedLibrary, Track};
use rustic_core::provider;

use crate::album::*;
use crate::artist::*;
use crate::meta::META_SPOTIFY_URI;
use crate::player::SpotifyPlayer;
use crate::playlist::*;
use crate::track::*;

mod album;
mod artist;
mod meta;
mod player;
mod playlist;
mod track;
mod util;

#[derive(Clone, Deserialize, Debug)]
pub struct SpotifyProvider {
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
    #[serde(skip)]
    client: Option<Spotify>,
    #[serde(skip)]
    player: SpotifyPlayer,
}

impl SpotifyProvider {
    fn sync_tracks(&mut self, library: &SharedLibrary) -> Result<(usize, usize), Error> {
        let spotify = self.client.as_ref().unwrap();

        let albums = spotify.current_user_saved_albums(None, None)?.items;

        let albums_len = albums.len();

        let mut tracks = albums
            .into_iter()
            .map(|album| album.album)
            .map(|album| {
                let mut album_entity = Album::from(SpotifyFullAlbum::from(album.clone()));
                library.sync_album(&mut album_entity);
                album
                    .tracks
                    .items
                    .into_iter()
                    .map(SpotifySimplifiedTrack::from)
                    .map(Track::from)
                    .map(|mut track| {
                        track.album_id = album_entity.id;
                        track
                    })
                    .collect()
            })
            .fold(vec![], |mut a, b: Vec<Track>| {
                a.extend(b);
                a
            });

        library.sync_tracks(&mut tracks)?;

        Ok((tracks.len(), albums_len))
    }

    fn sync_playlists(&mut self, library: &SharedLibrary) -> Result<usize, Error> {
        let spotify = self.client.as_ref().unwrap();

        let playlists = spotify.current_user_playlists(None, None)?;
        let mut playlists = playlists
            .items
            .into_iter()
            .map(|playlist| {
                spotify
                    .playlist(&playlist.id, None, None)
                    .map(SpotifyPlaylist::from)
                    .map(Playlist::from)
            })
            .collect::<Result<Vec<Playlist>, Error>>()?;

        let playlist_count = playlists.len();

        library.sync_playlists(&mut playlists)?;

        Ok(playlist_count)
    }
}

impl rustic_core::provider::ProviderInstance for SpotifyProvider {
    fn setup(&mut self) -> Result<(), Error> {
        let mut oauth = SpotifyOAuth::default()
            .client_id(&self.client_id)
            .client_secret(&self.client_secret)
            .scope(
                &[
                    "user-library-read",
                    "playlist-read-private",
                    "user-top-read",
                    "user-read-recently-played",
                    "playlist-read-collaborative",
                ]
                .join(" "),
            )
            .redirect_uri("http://localhost:8888/callback")
            .build();

        let spotify = get_token(&mut oauth)
            .map(|token_info| {
                let client_credential = SpotifyClientCredentials::default()
                    .token_info(token_info)
                    .build();
                Spotify::default()
                    .client_credentials_manager(client_credential)
                    .build()
            })
            .ok_or_else(|| err_msg("Spotify auth failed"))?;

        self.client = Some(spotify);

        self.player.setup(&self.username, &self.password)?;

        Ok(())
    }

    fn title(&self) -> &'static str {
        "Spotify"
    }

    fn uri_scheme(&self) -> &'static str {
        "spotify"
    }

    fn provider(&self) -> provider::Provider {
        provider::Provider::Spotify
    }

    fn sync(&mut self, library: SharedLibrary) -> Result<provider::SyncResult, Error> {
        let (tracks, albums) = self.sync_tracks(&library)?;

        let playlists = self.sync_playlists(&library)?;

        Ok(provider::SyncResult {
            tracks,
            albums,
            artists: 0,
            playlists,
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
        trace!("search {}", query);
        let spotify = self.client.clone().unwrap();

        let albums = spotify.search_album(&query, None, None, None)?;
        let artists = spotify.search_artist(&query, None, None, None)?;
        let tracks = spotify.search_track(&query, None, None, None)?;

        let albums = albums
            .albums
            .items
            .into_iter()
            .map(SpotifySimplifiedAlbum::from)
            .map(Album::from)
            .map(provider::ProviderItem::from);
        let artists = artists
            .artists
            .items
            .into_iter()
            .map(SpotifyFullArtist::from)
            .map(Artist::from)
            .map(provider::ProviderItem::from);
        let tracks = tracks
            .tracks
            .items
            .into_iter()
            .map(SpotifyFullTrack::from)
            .map(Track::from)
            .map(provider::ProviderItem::from);

        Ok(albums.chain(artists).chain(tracks).collect())
    }

    fn resolve_track(&self, _uri: &str) -> Result<Option<Track>, Error> {
        Ok(None)
    }

    fn resolve_album(&self, _uri: &str) -> Result<Option<Album>, Error> {
        Ok(None)
    }

    fn stream_url(&self, track: &Track) -> Result<String, Error> {
        let uri = track
            .meta
            .get(META_SPOTIFY_URI)
            .ok_or_else(|| format_err!("Missing spotify uri"))?;
        if let MetaValue::String(uri) = uri {
            let uri = uri.clone();
            let player = self.player.clone();
            let t = thread::spawn(move || player.get_audio_file(&uri))
                .join()
                .unwrap()
                .unwrap();

            unimplemented!()
        } else {
            unreachable!()
        }
    }

    fn cover_art(&self, track: &Track) -> Result<Option<provider::CoverArt>, Error> {
        let url = track
            .meta
            .get(meta::META_SPOTIFY_COVER_ART_URL)
            .map(|value| match value {
                MetaValue::String(url) => url.clone(),
                _ => unreachable!()
            })
            .map(|url| url.into());

        Ok(url)
    }

    fn resolve_share_url(&self, _url: url::Url) -> Result<Option<provider::InternalUri>, Error> {
        Ok(None)
    }
}
