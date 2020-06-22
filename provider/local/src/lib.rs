use std::path::PathBuf;

use failure::{Error, format_err};
use maplit::hashmap;
use serde_derive::Deserialize;

use async_trait::async_trait;
use rustic_core::CredentialStore;
use rustic_core::library::{self, SharedLibrary};
use rustic_core::provider::*;

use crate::scanner::Track;

pub mod scanner;

const META_LOCAL_FILE_URL: &str = "LOCAL_FILE_URL";

#[derive(Clone, Deserialize, Debug)]
pub struct LocalProvider {
    path: PathBuf,
}

impl LocalProvider {
    pub fn new() -> Option<Self> {
        dirs::audio_dir().map(|path| LocalProvider { path })
    }
}

#[async_trait]
impl ProviderInstance for LocalProvider {
    fn title(&self) -> &'static str {
        "Local"
    }

    fn uri_scheme(&self) -> &'static str {
        "file"
    }

    fn provider(&self) -> ProviderType {
        ProviderType::LocalMedia
    }

    async fn setup(&mut self, _cred_store: &dyn CredentialStore) -> Result<(), Error> {
        Ok(())
    }

    fn auth_state(&self) -> AuthState {
        AuthState::NoAuthentication
    }

    async fn authenticate(
        &mut self,
        _: Authentication,
        _cred_store: &dyn CredentialStore,
    ) -> Result<(), Error> {
        Ok(())
    }

    async fn sync(&self, library: SharedLibrary) -> Result<SyncResult, Error> {
        let scanner = scanner::Scanner::new(&self.path);
        let tracks = scanner.scan()?;
        let albums = LocalProvider::sync_albums(&library, &tracks);
        let artists = LocalProvider::sync_artists(&library, &tracks);
        let mut tracks = tracks
            .into_iter()
            .map(library::Track::from)
            .map(|mut t| {
                LocalProvider::apply_album_id(&albums, &mut t);
                LocalProvider::apply_artist_id(&artists, &mut t);
                t
            })
            .collect();
        library.sync_tracks(&mut tracks)?;
        Ok(SyncResult {
            tracks: tracks.len(),
            albums: albums.len(),
            artists: 0,
            playlists: 0,
        })
    }

    fn root(&self) -> ProviderFolder {
        ProviderFolder {
            folders: vec![],
            items: vec![],
        }
    }

    async fn navigate(&self, _path: Vec<String>) -> Result<ProviderFolder, Error> {
        Ok(self.root())
    }

    async fn search(&self, _query: String) -> Result<Vec<ProviderItem>, Error> {
        Ok(vec![])
    }

    async fn resolve_track(&self, uri: &str) -> Result<Option<library::Track>, Error> {
        let track = self.get_track(uri)?;

        Ok(Some(track.into()))
    }

    async fn resolve_album(&self, uri: &str) -> Result<Option<library::Album>, Error> {
        let track = self.get_track(uri)?;

        Ok(track.into())
    }

    async fn resolve_artist(&self, _uri: &str) -> Result<Option<library::Artist>, Error> {
        unimplemented!()
    }

    async fn resolve_playlist(&self, _uri: &str) -> Result<Option<library::Playlist>, Error> {
        unimplemented!()
    }

    async fn stream_url(&self, track: &library::Track) -> Result<String, Error> {
        if track.provider == ProviderType::LocalMedia {
            return Ok(track.uri.clone());
        }

        Err(format_err!("Invalid provider: {:?}", track.provider))
    }

    async fn thumbnail(&self, provider_item: &ProviderItemType) -> Result<Option<Thumbnail>, Error> {
        let uri = match provider_item {
            ProviderItemType::Track(track) => {
                if track.thumbnail == ThumbnailState::Data {
                    Some(track.uri.clone())
                }else {
                    None
                }
            },
            ProviderItemType::Album(album) => {
                if album.thumbnail == ThumbnailState::Data {
                    Some(album.uri.clone())
                }else {
                    None
                }
            },
            _ => None
        };
        if let Some(uri) = uri {
            let path = &uri["file://".len()..];
            let tag = id3::Tag::read_from_path(path)?;
            let picture = tag.pictures().find(|_| true).map(|picture| Thumbnail::Data {
                data: picture.data.clone(),
                mime_type: picture.mime_type.clone(),
            });

            Ok(picture)
        }else {
            Ok(None)
        }
    }

    async fn resolve_share_url(&self, _url: url::Url) -> Result<Option<InternalUri>, Error> {
        Ok(None)
    }
}


impl From<scanner::Track> for library::Track {
    fn from(track: scanner::Track) -> Self {
        let path = track.path.clone();
        library::Track {
            id: None,
            title: track.title.clone(),
            album_id: None,
            album: track.clone().into(),
            artist_id: None,
            artist: track.clone().into(),
            thumbnail: if track.has_coverart { ThumbnailState::Data } else { ThumbnailState::None },
            provider: ProviderType::LocalMedia,
            uri: format!("file://{}", track.path),
            duration: None,
            meta: hashmap!(
                META_LOCAL_FILE_URL.into() => path.into()
            ),
        }
    }
}

impl From<scanner::Track> for Option<library::Album> {
    fn from(track: scanner::Track) -> Self {
        let path = track.path.clone();
        let artist = track.clone().into();
        let has_coverart = track.has_coverart;
        track.album.map(|name| library::Album {
            id: None,
            title: name,
            artist_id: None,
            artist,
            provider: ProviderType::LocalMedia,
            thumbnail: if has_coverart { ThumbnailState::Data } else { ThumbnailState::None },
            tracks: vec![],
            uri: format!("file://{}", &path),
            meta: hashmap!(
                META_LOCAL_FILE_URL.into() => path.into()
            ),
        })
    }
}

impl From<scanner::Track> for Option<library::Artist> {
    fn from(track: scanner::Track) -> Self {
        let path = track.path.clone();
        track.artist.map(|name| library::Artist {
            id: None,
            name,
            uri: format!("file://{}", &path),
            image_url: None,
            meta: hashmap!(
                META_LOCAL_FILE_URL.into() => path.into()
            ),
            provider: ProviderType::LocalMedia,
            albums: vec![],
            playlists: vec![],
        })
    }
}

impl LocalProvider {
    fn sync_albums(library: &SharedLibrary, tracks: &[Track]) -> Vec<library::Album> {
        let albums: Vec<library::Album> = tracks
            .iter()
            .cloned()
            .filter_map(Option::<library::Album>::from)
            .fold(Vec::new(), |mut albums, album| {
                if albums.iter().find(|a| a.title == album.title).is_none() {
                    albums.push(album);
                }
                albums
            });
        let albums: Vec<library::Album> = albums
            .into_iter()
            .map(|mut album| -> Result<library::Album, Error> {
                library.sync_album(&mut album)?;
                Ok(album)
            })
            .filter_map(|a| a.ok())
            .collect();
        albums
    }

    fn sync_artists(library: &SharedLibrary, tracks: &[Track]) -> Vec<library::Artist> {
        let artists: Vec<library::Artist> = tracks
            .iter()
            .cloned()
            .filter_map(Option::<library::Artist>::from)
            .fold(Vec::new(), |mut artists, artist| {
                if artists.iter().find(|a| a.name == artist.name).is_none() {
                    artists.push(artist);
                }
                artists
            });
        let artists: Vec<library::Artist> = artists
            .into_iter()
            .map(|mut artist| -> Result<library::Artist, Error> {
                library.sync_artist(&mut artist)?;
                Ok(artist)
            })
            .filter_map(|a| a.ok())
            .collect();
        artists
    }

    fn apply_album_id(albums: &[library::Album], mut t: &mut library::Track) {
        if let Some(track_album) = &t.album {
            let album = albums.iter().find(|a| a.title == track_album.title);
            if let Some(album) = album {
                t.album_id = album.id;
            }
        }
    }

    fn apply_artist_id(artists: &[library::Artist], mut t: &mut library::Track) {
        if let Some(track_artist) = &t.artist {
            let artist = artists.iter().find(|a| a.name == track_artist.name);
            if let Some(artist) = artist {
                t.artist_id = artist.id;
            }
        }
    }

    fn get_track(&self, uri: &str) -> Result<Track, Error> {
        let path = &uri["file://".len()..];
        let tag = id3::Tag::read_from_path(path)?;
        let track = Track {
            path: path.to_string(),
            title: tag.title().map(String::from).unwrap_or_default(),
            artist: tag.artist().map(String::from),
            album: tag.album().map(String::from),
            has_coverart: tag.pictures().any(|_| true),
        };

        Ok(track)
    }
}
