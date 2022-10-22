use async_trait::async_trait;
use failure::{ensure, format_err, Error};
use serde::Deserialize;
use url::Url;
use youtube_api::YoutubeDl;

use rustic_core::{Credentials, CredentialStore, InternalUri, Playlist, provider, ProviderType, Rating, TrackPosition};
use rustic_core::library::{Album, Artist, Lyrics, SharedLibrary, Track};
use rustic_core::provider::{Authentication, ProviderFolder, ProviderItem, ProviderState, SyncResult, ThumbnailState};
use ytmusic::YoutubeMusicClient;
use crate::mappings::{map_album, map_artist, map_playlist, map_track};

const ARTIST_URI_PREFIX: &str = "ytmusic://artist/";
const ALBUM_URI_PREFIX: &str = "ytmusic://album/";
const TRACK_URI_PREFIX: &str = "ytmusic://track/";
const PLAYLIST_URI_PREFIX: &str = "ytmusic://playlist/";

mod mappings;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct YouTubeMusicProvider {
    cookies: String,
    #[serde(default)]
    user_id: Option<String>,
    #[serde(skip)]
    client: Option<YoutubeMusicClient>,
}

impl YouTubeMusicProvider {
    pub fn new() -> Option<Self> {
        None
    }

    fn get_youtube_id<'a>(&self, uri: &'a str) -> Result<&'a str, Error> {
        ensure!(uri.starts_with(TRACK_URI_PREFIX), "Invalid Uri: {}", uri);
        let id = &uri[TRACK_URI_PREFIX.len()..];

        Ok(id)
    }

    fn get_album_id<'a>(&self, uri: &'a str) -> Result<&'a str, Error> {
        ensure!(uri.starts_with(ALBUM_URI_PREFIX), "Invalid Uri: {}", uri);
        let id = &uri[ALBUM_URI_PREFIX.len()..];

        Ok(id)
    }

    fn client(&self) -> Result<YoutubeMusicClient, Error> {
        self
            .client
            .clone()
            .ok_or_else(|| format_err!("Provider not setup"))
    }

    async fn get_playlist(&self, id: &str) -> Result<Option<Playlist>, Error> {
        let client = self.client()?;
        let playlist = client.get_playlist(id).await?;

        Ok(playlist.map(map_playlist))
    }

    async fn sync_playlists(&self, client: &YoutubeMusicClient, library: &SharedLibrary) -> Result<usize, Error> {
        let mut playlists = vec![];
        for playlist in client.get_library_playlists(None).await? {
            if let Some(playlist) = self.get_playlist(&playlist.id).await? {
                playlists.push(playlist);
            }
        }

        library.sync_playlists(&mut playlists)?;

        Ok(playlists.len())
    }

    async fn sync_albums(&self, client: &YoutubeMusicClient, library: &SharedLibrary) -> Result<usize, Error> {
        let mut albums = vec![];
        for album in client.get_library_albums(None).await? {
            if let Some(album) = client.get_album(&album.id).await? {
                albums.push(album);
            }
        }
        let mut albums = albums.into_iter()
            .map(map_album)
            .collect();

        library.sync_albums(&mut albums)?;

        Ok(albums.len())
    }

    async fn sync_artists(&self, client: &YoutubeMusicClient, library: &SharedLibrary) -> Result<usize, Error> {
        let mut artists = vec![];
        for artist in client.get_library_artists(None).await? {
            if let Some(artist) = client.get_artist(&artist.id).await? {
                artists.push(artist);
            }
        }
        let mut artists = artists.into_iter()
            .map(map_artist)
            .collect();

        library.sync_artists(&mut artists)?;

        Ok(artists.len())
    }
}

#[async_trait]
impl provider::ProviderInstance for YouTubeMusicProvider {
    async fn setup(&mut self, cred_store: &dyn CredentialStore) -> Result<(), Error> {
        let mut client = YoutubeMusicClient::new(&self.cookies, self.user_id.clone())?;
        client.fetch_visitor_id().await?;
        self.client = Some(client);

        Ok(())
    }

    fn title(&self) -> &'static str {
        "YouTube Music"
    }

    fn uri_scheme(&self) -> &'static str {
        "ytmusic"
    }

    fn provider(&self) -> ProviderType {
        ProviderType::YouTubeMusic
    }

    fn state(&self) -> ProviderState {
        ProviderState::Authenticated(None)
    }

    async fn authenticate(&mut self, auth: Authentication, cred_store: &dyn CredentialStore) -> Result<(), Error> {
        todo!()
    }

    async fn sync(&self, library: SharedLibrary) -> Result<SyncResult, Error> {
        let client = self.client()?;

        let playlists = self.sync_playlists(&client, &library).await?;
        let albums = self.sync_albums(&client, &library).await?;
        let artists = self.sync_artists(&client, &library).await?;

        Ok(SyncResult {
            playlists,
            albums,
            artists,
            tracks: 0,
        })
    }

    fn root(&self) -> ProviderFolder {
        ProviderFolder {
            items: vec![],
            folders: vec![],
        }
    }

    async fn navigate(&self, path: Vec<String>) -> Result<ProviderFolder, Error> {
        todo!()
    }

    async fn search(&self, query: String) -> Result<Vec<ProviderItem>, Error> {
        todo!()
    }

    async fn resolve_track(&self, uri: &str) -> Result<Option<Track>, Error> {
        if !uri.starts_with(TRACK_URI_PREFIX) {
            return Ok(None);
        }
        let id = &uri[TRACK_URI_PREFIX.len()..];
        let client = self.client()?;
        let song = client.get_song(id).await?;

        Ok(song.map(map_track))

    }

    async fn resolve_album(&self, uri: &str) -> Result<Option<Album>, Error> {
        if !uri.starts_with(ALBUM_URI_PREFIX) {
            return Ok(None);
        }
        let id = &uri[ALBUM_URI_PREFIX.len()..];
        let client = self.client()?;
        let album = client.get_album(id).await?;

        Ok(album.map(map_album))
    }

    async fn resolve_artist(&self, uri: &str) -> Result<Option<Artist>, Error> {
        if !uri.starts_with(ARTIST_URI_PREFIX) {
            return Ok(None);
        }
        let id = &uri[ARTIST_URI_PREFIX.len()..];
        let client = self.client()?;
        let artist = client.get_artist(id).await?;

        Ok(artist.map(map_artist))
    }

    async fn resolve_playlist(&self, uri: &str) -> Result<Option<Playlist>, Error> {
        if !uri.starts_with(PLAYLIST_URI_PREFIX) {
            return Ok(None);
        }
        let id = &uri[PLAYLIST_URI_PREFIX.len()..];
        let playlist = self.get_playlist(id).await?;

        Ok(playlist)
    }

    async fn stream_url(&self, track: &Track) -> Result<String, Error> {
        let id = self.get_youtube_id(&track.uri)?;
        let youtube_dl = YoutubeDl::default();
        let url = youtube_dl.get_audio_stream_url(id).await?;

        Ok(url)
    }

    async fn resolve_share_url(&self, url: Url) -> Result<Option<InternalUri>, Error> {
        if url.host_str() != Some("music.youtube.com") {
            return Ok(None);
        }
        if url.path() == "/playlist" {
            let id = url.query_pairs()
                .find(|(key, _)| key == "list")
                .map(|(_, value)| value.to_string());

            if let Some(id) = id {
                return Ok(Some(InternalUri::Playlist(format!("{PLAYLIST_URI_PREFIX}{id}"))));
            }
        }

        Ok(None)
    }
}

