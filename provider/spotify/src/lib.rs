mod album;
mod artist;
mod track;
mod util;

use failure::{err_msg, Error};
use log::{debug, trace};
use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::spotify::util::get_token;
use rustic_core::library::{Album, Artist, SharedLibrary, Track};
use rustic_core::provider;
use serde_derive::Deserialize;

use crate::album::*;
use crate::artist::*;
use crate::track::*;

#[derive(Clone, Deserialize, Debug)]
pub struct SpotifyProvider {
    client_id: String,
    client_secret: String,
    #[serde(skip)]
    client: Option<Spotify>,
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
        let spotify = self.client.clone().unwrap();

        let albums = spotify.current_user_saved_albums(None, None)?.items;

        debug!("{:?}", albums);

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

        Ok(provider::SyncResult {
            tracks: tracks.len(),
            albums: albums_len,
            artists: 0,
            playlists: 0,
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

    fn stream_url(&self, _track: &Track) -> Result<String, Error> {
        unimplemented!()
    }
}
