use async_trait::async_trait;
use failure::{Error, format_err};
use log::trace;
use rspotify::{AuthCodeSpotify, OAuth, Token};
use rspotify::model::{AlbumId, ArtistId, PlaylistId, SearchResult, SearchType, TrackId};
use rspotify::prelude::*;
use serde_derive::Deserialize;

use rustic_core::{CredentialStore, provider};
use rustic_core::library::{Album, Artist, MetaValue, Playlist, SharedLibrary, Track};

use crate::album::*;
use crate::artist::*;
use crate::meta::META_SPOTIFY_URI;
use crate::playlist::*;
use crate::track::*;

mod album;
mod artist;
mod meta;
mod player;
mod playlist;
mod track;
mod util;

// TODO: configurable host
const SPOTIFY_REDIRECT_URI: &str = "http://localhost:8080/api/providers/spotify/auth/redirect";

#[derive(Clone, Deserialize, Debug)]
pub struct SpotifyProvider {
    client_id: String,
    client_secret: String,
    // username: String,
    // password: String,
    #[serde(skip)]
    client: Option<AuthCodeSpotify>,
    // #[serde(skip)]
    // player: SpotifyPlayer,
}

impl SpotifyProvider {
    pub fn new() -> Option<Self> {
        let client_id = option_env!("SPOTIFY_CLIENT_ID").map(String::from);
        let client_secret = option_env!("SPOTIFY_CLIENT_SECRET").map(String::from);

        client_id
            .zip(client_secret)
            .map(|(client_id, client_secret)| SpotifyProvider {
                client_id,
                client_secret,
                client: None,
            })
    }

    async fn sync_tracks(&self, library: &SharedLibrary) -> Result<(usize, usize), Error> {
        let spotify = self.client.as_ref().unwrap();

        let albums = spotify.current_user_saved_albums_manual(None, None).await?.items;

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

    async fn sync_playlists(&self, library: &SharedLibrary) -> Result<usize, Error> {
        let spotify = self.client.as_ref().unwrap();

        let user_playlists = spotify.current_user_playlists_manual(None, None).await?.items;
        let mut playlists = Vec::with_capacity(user_playlists.len());
        for playlist in user_playlists {
            let id = PlaylistId::from_id(&playlist.id)?;
            // TODO: this should await all playlists at once
            let p = spotify
                .playlist(&id, None, None)
                .await
                .map(SpotifyPlaylist::from)
                .map(Playlist::from)?;
            playlists.push(p);
        }

        let playlist_count = playlists.len();

        library.sync_playlists(&mut playlists)?;

        Ok(playlist_count)
    }

    fn get_oauth_client(&self) -> Result<AuthCodeSpotify, Error> {
        let oauth = OAuth {
            redirect_uri: SPOTIFY_REDIRECT_URI.to_string(),
            scopes: rspotify::scopes!(
                "user-library-read",
                "playlist-read-private",
                "user-top-read",
                "user-read-recently-played",
                "playlist-read-collaborative"
            ),
            ..Default::default()
        };
        let credentials = rspotify::Credentials::new(&self.client_id, &self.client_secret);

        let mut spotify = AuthCodeSpotify::new(credentials, oauth);
        spotify.config.token_cached = true;
        spotify.token = match Token::from_cache(&spotify.config.cache_path) {
            Ok(token) => {
                Some(token)
            }
            Err(err) => {
                log::warn!("Loading spotify token failed {:?}", err);
                None
            }
        };
        Ok(spotify)
    }
}

#[async_trait]
impl rustic_core::provider::ProviderInstance for SpotifyProvider {
    async fn setup(&mut self, cred_store: &dyn CredentialStore) -> Result<(), Error> {
        let client = self.get_oauth_client()?;
        self.client = Some(client);

        Ok(())
    }

    fn state(&self) -> provider::ProviderState {
        if let Some(_) = self.client.as_ref().and_then(|client| client.get_token()) {
            provider::ProviderState::Authenticated(None)
        } else if let Some(ref client) = self.client {
            let auth_url = client.get_authorize_url(false).unwrap();
            provider::ProviderState::RequiresOAuth(auth_url)
        } else {
            provider::ProviderState::InvalidConfiguration(None)
        }
    }

    async fn authenticate(
        &mut self,
        auth: provider::Authentication,
        cred_store: &dyn CredentialStore,
    ) -> Result<(), Error> {
        use provider::Authentication::*;
        match auth {
            Token(token) => {
                if let Some(ref mut client) = self.client {
                    client.request_token(&token).await?;
                    Ok(())
                } else {
                    unreachable!()
                }
            }
            TokenWithState(token, state) => {
                if let Some(ref mut client) = self.client {
                    if client.oauth.state != state {
                        return Err(format_err!("Invalid token state"));
                    }
                    client.request_token(&token).await?;
                    Ok(())
                } else {
                    unreachable!()
                }
            }
            _ => Err(format_err!("Invalid authentication method")),
        }
    }

    fn title(&self) -> &'static str {
        "Spotify"
    }

    fn uri_scheme(&self) -> &'static str {
        "spotify"
    }

    fn provider(&self) -> provider::ProviderType {
        provider::ProviderType::Spotify
    }

    async fn sync(&self, library: SharedLibrary) -> Result<provider::SyncResult, Error> {
        let (tracks, albums) = self.sync_tracks(&library).await?;

        let playlists = self.sync_playlists(&library).await?;

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

    async fn navigate(&self, _path: Vec<String>) -> Result<provider::ProviderFolder, Error> {
        Ok(self.root())
    }

    async fn search(&self, query: String) -> Result<Vec<provider::ProviderItem>, Error> {
        trace!("search {}", query);
        let spotify = self.client.clone().unwrap();

        let albums = if let SearchResult::Albums(albums) = spotify.search(&query, &SearchType::Album, None, None, None, None).await? {
            albums.items
        } else {
            Default::default()
        };
        let artists = if let SearchResult::Artists(artists) = spotify.search(&query, &SearchType::Artist, None, None, None, None).await? {
            artists.items
        } else {
            Default::default()
        };
        let tracks = if let SearchResult::Tracks(tracks) = spotify.search(&query, &SearchType::Track, None, None, None, None).await? {
            tracks.items
        } else {
            Default::default()
        };

        let albums = albums
            .into_iter()
            .map(SpotifySimplifiedAlbum::from)
            .map(Album::from)
            .map(provider::ProviderItem::from);
        let artists = artists
            .into_iter()
            .map(SpotifyFullArtist::from)
            .map(Artist::from)
            .map(provider::ProviderItem::from);
        let tracks = tracks
            .into_iter()
            .map(SpotifyFullTrack::from)
            .map(Track::from)
            .map(provider::ProviderItem::from);

        Ok(albums.chain(artists).chain(tracks).collect())
    }

    async fn resolve_track(&self, uri: &str) -> Result<Option<Track>, Error> {
        let spotify = self.client.as_ref().unwrap();
        let id = &uri["spotify://track/".len()..];
        let track = spotify.track(&TrackId::from_id(id)?).await?;
        let track = SpotifyFullTrack::from(track);
        let track = track.into();

        Ok(Some(track))
    }

    async fn resolve_album(&self, uri: &str) -> Result<Option<Album>, Error> {
        let spotify = self.client.as_ref().unwrap();
        let id = &uri["spotify://album/".len()..];
        let album = spotify.album(&AlbumId::from_id(id)?).await?;
        let album = SpotifyFullAlbum::from(album);
        let album = album.into();

        Ok(Some(album))
    }

    async fn resolve_artist(&self, uri: &str) -> Result<Option<Artist>, Error> {
        let spotify = self.client.as_ref().unwrap();
        let id = &uri["spotify://artist/".len()..];
        let artist = spotify.artist(&ArtistId::from_id(id)?).await?;
        let artist = SpotifyFullArtist::from(artist);
        let artist = artist.into();

        Ok(Some(artist))
    }

    async fn resolve_playlist(&self, uri: &str) -> Result<Option<Playlist>, Error> {
        let spotify = self.client.clone().unwrap();
        let id = &uri["spotify://playlists/".len()..];
        let playlist = spotify.playlist(&PlaylistId::from_id(id)?, None, None).await?;
        let playlist = SpotifyPlaylist::from(playlist);
        let playlist = Playlist::from(playlist);

        Ok(Some(playlist))
    }

    async fn stream_url(&self, track: &Track) -> Result<String, Error> {
        let uri = track
            .meta
            .get(META_SPOTIFY_URI)
            .ok_or_else(|| format_err!("Missing spotify uri"))?;
        if let MetaValue::String(uri) = uri {
            // let player = self.player.clone();
            // let file_path = player.get_audio_file(&uri).await?;

            // Ok(format!("file://{}", file_path))

            unimplemented!()
        } else {
            unreachable!()
        }
    }

    async fn resolve_share_url(
        &self,
        _url: url::Url,
    ) -> Result<Option<provider::InternalUri>, Error> {
        Ok(None)
    }
}
