use failure::{Error, format_err};
use maplit::hashmap;
use serde_derive::Deserialize;

use rustic_core::library::{self, SharedLibrary, MetaValue};
use rustic_core::provider::*;

pub mod scanner;

const META_LOCAL_FILE_URL: &str = "LOCAL_FILE_URL";

#[derive(Clone, Deserialize, Debug)]
pub struct LocalProvider {
    path: String,
}

impl ProviderInstance for LocalProvider {
    fn title(&self) -> &'static str {
        "Local"
    }

    fn uri_scheme(&self) -> &'static str {
        "file"
    }

    fn provider(&self) -> Provider {
        Provider::LocalMedia
    }

    fn setup(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn sync(&mut self, library: SharedLibrary) -> Result<SyncResult, Error> {
        let scanner = scanner::Scanner::new(self.path.clone());
        let tracks = scanner.scan()?;
        let albums: Vec<library::Album> = tracks
            .iter()
            .cloned()
            .map(|track| track.into())
            .filter(|album: &Option<library::Album>| album.is_some())
            .map(|album| album.unwrap())
            .fold(Vec::new(), |mut albums, album| {
                if albums.iter().find(|a| a.title == album.title).is_none() {
                    albums.push(album);
                }
                albums
            });
        let albums: Vec<library::Album> = albums
            .into_iter()
            .map(|mut album| -> Result<library::Album, Error> {
                library.add_album(&mut album)?;
                Ok(album)
            })
            .filter(|a| a.is_ok())
            .map(|a| a.unwrap())
            .collect();
        let mut tracks = tracks
            .into_iter()
            .map(library::Track::from)
            .map(|mut t| {
                if let Some(track_album) = &t.album {
                    let album = albums.iter().find(|a| a.title == track_album.title);
                    if let Some(album) = album {
                        t.album_id = album.id;
                    }
                }
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

    fn navigate(&self, _path: Vec<String>) -> Result<ProviderFolder, Error> {
        Ok(self.root())
    }

    fn search(&self, _query: String) -> Result<Vec<ProviderItem>, Error> {
        Ok(vec![])
    }

    fn resolve_track(&self, _uri: &str) -> Result<Option<library::Track>, Error> {
        Ok(None)
    }

    fn stream_url(&self, track: &library::Track) -> Result<String, Error> {
        if track.provider == Provider::LocalMedia {
            return Ok(track.uri.clone());
        }

        Err(format_err!("Invalid provider: {:?}", track.provider))
    }

    fn cover_art(&self, track: &library::Track) -> Result<Option<CoverArt>, Error> {
        if let MetaValue::String(url) = track.meta.get(META_LOCAL_FILE_URL).as_ref().unwrap() {
            let tag = id3::Tag::read_from_path(&url)?;
            let picture = tag.pictures()
                .find(|_| true)
                .map(|picture| CoverArt::Data {
                    data: picture.data.clone(),
                    mime_type: picture.mime_type.clone()
                });

            Ok(picture)
        }else {
            unreachable!()
        }
    }
}

impl From<scanner::Track> for library::Track {
    fn from(track: scanner::Track) -> Self {
        let path = track.path.clone();
        library::Track {
            id: None,
            title: track.title,
            album_id: None,
            album: track.album.map(|name| library::Album {
                id: None,
                title: name,
                artist_id: None,
                artist: None,
                provider: Provider::LocalMedia,
                image_url: None,
                tracks: vec![],
                uri: String::new(),
                meta: hashmap!(
                    META_LOCAL_FILE_URL.into() => path.clone().into()
                ),
            }),
            artist_id: None,
            artist: track.artist.map(|name| library::Artist {
                id: None,
                name,
                uri: String::new(),
                image_url: None,
                meta: hashmap!(
                    META_LOCAL_FILE_URL.into() => path.clone().into()
                ),
            }),
            has_coverart: track.has_coverart,
            provider: Provider::LocalMedia,
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
        track.album.map(|name| library::Album {
            id: None,
            title: name,
            artist_id: None,
            artist: None,
            provider: Provider::LocalMedia,
            image_url: None,
            tracks: vec![],
            uri: String::new(),
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
            uri: String::new(),
            image_url: None,
            meta: hashmap!(
                META_LOCAL_FILE_URL.into() => path.into()
            ),
        })
    }
}
