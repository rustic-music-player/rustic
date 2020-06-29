use std::fs;
use std::io::BufReader;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

use failure::Error;
use log::trace;
use pinboard::NonEmptyPinboard;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use rustic_core::library::Identifiable;
use rustic_core::{
    Album, Artist, Library, MultiQuery, Playlist, SearchResults, SingleQuery,
    SingleQueryIdentifier, Track,
};
use rustic_store_helpers::{join_album, join_albums, join_track};

#[derive(Debug, Serialize, Deserialize)]
struct LibrarySnapshot {
    album_id: usize,
    artist_id: usize,
    track_id: usize,
    playlist_id: usize,
    albums: Vec<Album>,
    artists: Vec<Artist>,
    tracks: Vec<Track>,
    playlists: Vec<Playlist>,
}

impl From<LibrarySnapshot> for MemoryLibrary {
    fn from(snapshot: LibrarySnapshot) -> Self {
        MemoryLibrary {
            persist: true,
            album_id: AtomicUsize::new(snapshot.album_id),
            artist_id: AtomicUsize::new(snapshot.artist_id),
            track_id: AtomicUsize::new(snapshot.track_id),
            playlist_id: AtomicUsize::new(snapshot.playlist_id),
            albums: NonEmptyPinboard::new(snapshot.albums),
            artists: NonEmptyPinboard::new(snapshot.artists),
            tracks: NonEmptyPinboard::new(snapshot.tracks),
            playlists: NonEmptyPinboard::new(snapshot.playlists),
        }
    }
}

#[derive(Debug)]
pub struct MemoryLibrary {
    persist: bool,
    album_id: AtomicUsize,
    artist_id: AtomicUsize,
    track_id: AtomicUsize,
    playlist_id: AtomicUsize,
    albums: NonEmptyPinboard<Vec<Album>>,
    artists: NonEmptyPinboard<Vec<Artist>>,
    tracks: NonEmptyPinboard<Vec<Track>>,
    playlists: NonEmptyPinboard<Vec<Playlist>>,
}

impl Default for MemoryLibrary {
    fn default() -> Self {
        MemoryLibrary {
            persist: false,
            album_id: AtomicUsize::new(1),
            artist_id: AtomicUsize::new(1),
            track_id: AtomicUsize::new(1),
            playlist_id: AtomicUsize::new(1),
            albums: NonEmptyPinboard::new(Vec::new()),
            artists: NonEmptyPinboard::new(Vec::new()),
            tracks: NonEmptyPinboard::new(Vec::new()),
            playlists: NonEmptyPinboard::new(Vec::new()),
        }
    }
}

impl MemoryLibrary {
    pub fn new(persist: bool) -> MemoryLibrary {
        let library = match MemoryLibrary::try_load() {
            Ok(lib) => lib,
            Err(e) => {
                log::error!("Failed to load previous library {}", e);
                None
            }
        };
        let mut library = library.unwrap_or_default();
        library.persist = persist;
        library
    }

    fn try_load() -> Result<Option<Self>, Error> {
        let path = Path::new(".store.json");
        if !path.exists() {
            Ok(None)
        } else {
            let file = fs::File::open(&path)?;
            let reader = BufReader::new(file);
            let library: LibrarySnapshot = from_reader(reader)?;
            Ok(Some(library.into()))
        }
    }

    fn snapshot(&self) -> LibrarySnapshot {
        LibrarySnapshot {
            album_id: self.album_id.load(Ordering::Relaxed),
            artist_id: self.artist_id.load(Ordering::Relaxed),
            track_id: self.track_id.load(Ordering::Relaxed),
            playlist_id: self.playlist_id.load(Ordering::Relaxed),
            albums: self.albums.read(),
            artists: self.artists.read(),
            tracks: self.tracks.read(),
            playlists: self.playlists.read(),
        }
    }

    fn persist(&self) {
        if !self.persist {
            return;
        }
        // TODO: this should happen on an interval on a background thread
        let snapshot = self.snapshot();
        tokio::spawn(async {
            if let Err(e) = MemoryLibrary::store(snapshot).await {
                log::error!("Storing memory library failed {}", e);
            }
        });
    }

    async fn store(snapshot: LibrarySnapshot) -> Result<(), failure::Error> {
        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(".store.json")
            .await?;
        let content = serde_json::to_string(&snapshot)?;
        file.write(content.as_bytes()).await?;

        Ok(())
    }

    fn find<I, T>(&self, iter: &mut I, query: &SingleQuery) -> Option<T>
    where
        I: Iterator<Item = T>,
        T: Identifiable,
    {
        match query.identifier {
            SingleQueryIdentifier::Id(id) => iter.find(|entity| entity.get_id() == Some(id)),
            SingleQueryIdentifier::Uri(ref uri) => iter.find(|entity| &entity.get_uri() == uri),
        }
    }
}

impl Library for MemoryLibrary {
    fn query_track(&self, query: SingleQuery) -> Result<Option<Track>, Error> {
        trace!("Query Track {:?}", query);
        let mut tracks = self.tracks.read().into_iter();
        let track = if let Some(track) = self.find(&mut tracks, &query) {
            Some(join_track(self, track, query.joins)?)
        } else {
            None
        };
        Ok(track)
    }

    fn query_tracks(&self, query: MultiQuery) -> Result<Vec<Track>, Error> {
        trace!("Query Tracks {:?}", query);
        self.tracks
            .read()
            .into_iter()
            .filter(|track| {
                if query.providers.is_empty() {
                    true
                } else {
                    query.providers.contains(&track.provider)
                }
            })
            .map(|track| join_track(self, track, query.joins))
            .collect()
    }

    fn query_album(&self, query: SingleQuery) -> Result<Option<Album>, Error> {
        trace!("Query Album {:?}", query);
        let mut albums = self.albums.read().into_iter();
        let album = if let Some(album) = self.find(&mut albums, &query) {
            Some(join_album(self, album, query.joins)?)
        } else {
            None
        };
        Ok(album)
    }

    fn query_albums(&self, query: MultiQuery) -> Result<Vec<Album>, Error> {
        trace!("Query Albums {:?}", query);
        let albums: Vec<_> = self
            .albums
            .read()
            .into_iter()
            .filter(|album| {
                if query.providers.is_empty() {
                    true
                } else {
                    query.providers.contains(&album.provider)
                }
            })
            .collect();
        join_albums(self, &albums, query.joins)
    }

    fn query_artist(&self, query: SingleQuery) -> Result<Option<Artist>, Error> {
        trace!("Query Artist {:?}", query);
        let mut artists = self.artists.read().into_iter();
        let artist = self.find(&mut artists, &query);
        Ok(artist)
    }

    fn query_artists(&self, query: MultiQuery) -> Result<Vec<Artist>, Error> {
        trace!("Query Artists {:?}", query);
        Ok(self.artists.read())
    }

    fn query_playlist(&self, query: SingleQuery) -> Result<Option<Playlist>, Error> {
        trace!("Query Playlist {:?}", query);
        let mut playlists = self.playlists.read().into_iter();
        let playlist = self.find(&mut playlists, &query);
        Ok(playlist)
    }

    fn query_playlists(&self, query: MultiQuery) -> Result<Vec<Playlist>, Error> {
        trace!("Query Playlists {:?}", query);
        let playlists = self
            .playlists
            .read()
            .into_iter()
            .filter(|playlist| {
                if query.providers.is_empty() {
                    true
                } else {
                    query.providers.contains(&playlist.provider)
                }
            })
            .collect();
        Ok(playlists)
    }

    fn add_track(&self, track: &mut Track) -> Result<(), Error> {
        track.id = Some(self.track_id.fetch_add(1, Ordering::Relaxed));
        // TODO: add artist and album
        let mut tracks = self.tracks.read();
        tracks.push(track.clone());
        self.tracks.set(tracks);
        self.persist();
        Ok(())
    }

    fn add_album(&self, album: &mut Album) -> Result<(), Error> {
        album.id = Some(self.album_id.fetch_add(1, Ordering::Relaxed));
        if let Some(artist) = album.artist.as_mut() {
            if artist.id.is_none() {
                self.add_artist(artist)?;
                album.artist_id = artist.id;
            }
        }
        for track in album.tracks.as_mut_slice() {
            if track.id.is_none() {
                track.album_id = album.id;
                if track.artist.is_none() {
                    track.artist_id = album.artist_id;
                };
                self.add_track(track)?;
            }
        }
        let mut albums = self.albums.read();
        albums.push(album.clone());
        self.albums.set(albums);
        self.persist();

        Ok(())
    }

    fn add_artist(&self, artist: &mut Artist) -> Result<(), Error> {
        trace!("Adding artist {}", artist.name);
        if let Some(a) = self.query_artist(SingleQuery::uri(artist.uri.clone()))? {
            artist.id = a.id;
        } else {
            artist.id = Some(self.artist_id.fetch_add(1, Ordering::Relaxed));
            let mut artists = self.artists.read();
            artists.push(artist.clone());
            self.artists.set(artists);
            self.persist();
        }
        Ok(())
    }

    fn add_playlist(&self, playlist: &mut Playlist) -> Result<(), Error> {
        playlist.id = Some(self.playlist_id.fetch_add(1, Ordering::Relaxed));
        let mut playlists = self.playlists.read();
        playlists.push(playlist.clone());
        self.playlists.set(playlists);
        self.persist();
        Ok(())
    }

    fn add_tracks(&self, tracks: &mut Vec<Track>) -> Result<(), Error> {
        for track in tracks {
            self.add_track(track)?;
        }
        Ok(())
    }

    fn add_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error> {
        for album in albums {
            self.add_album(album)?;
        }
        Ok(())
    }

    fn add_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error> {
        for artist in artists {
            self.add_artist(artist)?;
        }
        Ok(())
    }

    fn add_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        for playlist in playlists {
            self.add_playlist(playlist)?;
        }
        Ok(())
    }

    fn sync_track(&self, track: &mut Track) -> Result<(), Error> {
        let has_track = {
            let tracks = self.tracks.read();
            tracks.iter().find(|a| a.uri == track.uri).map(|a| a.id)
        };

        let id: usize = has_track
            .and_then(|id| id)
            .unwrap_or_else(|| self.track_id.fetch_add(1, Ordering::Relaxed));
        track.id = Some(id);

        if has_track.is_none() {
            let mut tracks = self.tracks.read();
            tracks.push(track.clone());
            self.tracks.set(tracks);
        }
        self.persist();
        Ok(())
    }

    fn sync_album(&self, album: &mut Album) -> Result<(), Error> {
        let has_album = {
            let albums = self.albums.read();
            albums.iter().find(|a| a.uri == album.uri).map(|a| a.id)
        };

        let id: usize = has_album
            .and_then(|id| id)
            .unwrap_or_else(|| self.album_id.fetch_add(1, Ordering::Relaxed));
        album.id = Some(id);

        if has_album.is_none() {
            let mut albums = self.albums.read();
            albums.push(album.clone());
            self.albums.set(albums);
        }
        self.persist();
        Ok(())
    }

    fn sync_artist(&self, artist: &mut Artist) -> Result<(), Error> {
        let has_artist = {
            let artists = self.artists.read();
            artists.iter().find(|a| a.uri == artist.uri).map(|a| a.id)
        };

        let id: usize = has_artist
            .and_then(|id| id)
            .unwrap_or_else(|| self.artist_id.fetch_add(1, Ordering::Relaxed));
        artist.id = Some(id);

        if has_artist.is_none() {
            let mut artists = self.artists.read();
            artists.push(artist.clone());
            self.artists.set(artists);
        }
        self.persist();
        Ok(())
    }

    fn sync_playlist(&self, playlist: &mut Playlist) -> Result<(), Error> {
        let (index, id) = {
            let playlists = self.playlists.read();
            let index = playlists.iter().position(|a| a.uri == playlist.uri);
            let id: usize = index
                .and_then(|index| playlists[index].id)
                .unwrap_or_else(|| self.playlist_id.fetch_add(1, Ordering::Relaxed));

            (index, id)
        };
        playlist.id = Some(id);

        let mut playlists = self.playlists.read();
        if let Some(index) = index {
            let target_playlist = playlists.get_mut(index).unwrap();
            *target_playlist = playlist.clone();
        } else {
            playlists.push(playlist.clone());
        }
        self.playlists.set(playlists);
        self.persist();
        Ok(())
    }

    fn sync_tracks(&self, tracks: &mut Vec<Track>) -> Result<(), Error> {
        tracks
            .iter_mut()
            .filter(|track| {
                self.tracks
                    .read()
                    .iter()
                    .find(|t| t.uri == track.uri)
                    .map(|_t| false)
                    .unwrap_or(true)
            })
            .map(|mut track| self.add_track(&mut track))
            .collect()
    }

    fn sync_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error> {
        albums
            .iter_mut()
            .filter(|album| {
                self.albums
                    .read()
                    .iter()
                    .find(|t| t.uri == album.uri)
                    .map(|_t| false)
                    .unwrap_or(true)
            })
            .map(|mut album| self.add_album(&mut album))
            .collect()
    }

    fn sync_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error> {
        artists
            .iter_mut()
            .filter(|artist| {
                self.artists
                    .read()
                    .iter()
                    .find(|t| t.uri == artist.uri)
                    .map(|_t| false)
                    .unwrap_or(true)
            })
            .map(|mut artist| self.add_artist(&mut artist))
            .collect()
    }

    fn sync_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        let stored_playlists = self.playlists.read();
        for playlist in playlists {
            if stored_playlists.contains(&playlist) {
                self.sync_playlist(playlist)?;
            } else {
                self.add_playlist(playlist)?;
            }
        }
        Ok(())
    }

    fn remove_track(&self, track: &Track) -> Result<(), Error> {
        let mut tracks = self.tracks.read();
        if let Some(position) = tracks.iter().position(|t| t.id == track.id) {
            tracks.remove(position);
            self.tracks.set(tracks);
            Ok(())
        } else {
            Ok(())
        }
    }

    fn remove_album(&self, album: &Album) -> Result<(), Error> {
        let mut albums = self.albums.read();
        if let Some(position) = albums.iter().position(|t| t.id == album.id) {
            albums.remove(position);
            self.albums.set(albums);
            Ok(())
        } else {
            Ok(())
        }
    }

    fn remove_artist(&self, artist: &Artist) -> Result<(), Error> {
        let mut artists = self.artists.read();
        if let Some(position) = artists.iter().position(|t| t.id == artist.id) {
            artists.remove(position);
            self.artists.set(artists);
            Ok(())
        } else {
            Ok(())
        }
    }

    fn remove_playlist(&self, playlist: &Playlist) -> Result<(), Error> {
        let mut playlists = self.playlists.read();
        if let Some(position) = playlists.iter().position(|t| t.id == playlist.id) {
            playlists.remove(position);
            self.playlists.set(playlists);
            Ok(())
        } else {
            Ok(())
        }
    }

    fn search(&self, query: String) -> Result<SearchResults, Error> {
        let tracks = self
            .tracks
            .read()
            .into_iter()
            .filter(|track| track.title.contains(query.as_str()))
            .collect();

        Ok(SearchResults {
            tracks,
            albums: vec![],
            artists: vec![],
            playlists: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rustic_core::{Artist, Library, ProviderType};

    use crate::MemoryLibrary;

    #[test]
    fn adding_the_same_artist_twice_should_only_store_it_once() {
        let mut artist = Artist {
            id: None,
            name: "Test Artist".into(),
            uri: "test:artist".into(),
            image_url: None,
            meta: HashMap::new(),
            provider: ProviderType::Internal,
            albums: Vec::new(),
            playlists: Vec::new(),
        };
        let mut second = artist.clone();
        let store = MemoryLibrary::default();
        store.add_artist(&mut artist).unwrap();

        store.add_artist(&mut second).unwrap();

        assert_eq!(second.id, artist.id);
    }
}
