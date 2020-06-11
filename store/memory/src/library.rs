use std::sync::{
    atomic::{AtomicUsize, Ordering},
    RwLock,
};

use failure::Error;
use log::trace;

use rustic_core::{
    Album, Artist, Library, MultiQuery, Playlist, SearchResults, SingleQuery,
    SingleQueryIdentifier, Track,
};
use rustic_store_helpers::{join_album, join_albums, join_track};

#[derive(Debug, Default)]
pub struct MemoryLibrary {
    album_id: AtomicUsize,
    artist_id: AtomicUsize,
    track_id: AtomicUsize,
    playlist_id: AtomicUsize,
    albums: RwLock<Vec<Album>>,
    artists: RwLock<Vec<Artist>>,
    tracks: RwLock<Vec<Track>>,
    playlists: RwLock<Vec<Playlist>>,
}

impl MemoryLibrary {
    pub fn new() -> MemoryLibrary {
        MemoryLibrary {
            album_id: AtomicUsize::new(1),
            artist_id: AtomicUsize::new(1),
            track_id: AtomicUsize::new(1),
            playlist_id: AtomicUsize::new(1),
            ..MemoryLibrary::default()
        }
    }
}

impl Library for MemoryLibrary {
    fn query_track(&self, query: SingleQuery) -> Result<Option<Track>, Error> {
        trace!("Query Track {:?}", query);
        let mut tracks = self.tracks.read().unwrap().clone().into_iter();
        let track = match query.identifier {
            SingleQueryIdentifier::Id(id) => tracks.find(|track| track.id == Some(id)),
            SingleQueryIdentifier::Uri(uri) => tracks.find(|track| track.uri == uri),
        };
        let track = if let Some(track) = track {
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
            .unwrap()
            .clone()
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
        let mut albums = self.albums.read().unwrap().clone().into_iter();
        let album = match query.identifier {
            SingleQueryIdentifier::Id(id) => albums.find(|album| album.id == Some(id)),
            SingleQueryIdentifier::Uri(uri) => albums.find(|album| album.uri == uri),
        };
        let album = if let Some(album) = album {
            Some(join_album(self, album, query.joins)?)
        } else {
            None
        };
        Ok(album)
    }

    fn query_albums(&self, query: MultiQuery) -> Result<Vec<Album>, Error> {
        trace!("Query Albums {:?}", query);
        let albums = self.albums.read().unwrap().clone();
        let albums: Vec<_> = albums
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
        let artist = match query.identifier {
            SingleQueryIdentifier::Id(id) => self
                .artists
                .read()
                .unwrap()
                .iter()
                .cloned()
                .find(|artist| artist.id == Some(id)),
            _ => None,
        };
        Ok(artist)
    }

    fn query_artists(&self, query: MultiQuery) -> Result<Vec<Artist>, Error> {
        trace!("Query Artists {:?}", query);
        let artists = self.artists.read().unwrap().clone();
        Ok(artists)
    }

    fn query_playlist(&self, query: SingleQuery) -> Result<Option<Playlist>, Error> {
        trace!("Query Playlist {:?}", query);
        let playlists = self.playlists.read().unwrap();
        let mut playlist_iter = playlists.iter();
        let playlist = match query.identifier {
            SingleQueryIdentifier::Id(id) => playlist_iter.find(|playlist| playlist.id == Some(id)),
            SingleQueryIdentifier::Uri(uri) => playlist_iter.find(|playlist| playlist.uri == uri),
        };
        Ok(playlist.cloned())
    }

    fn query_playlists(&self, query: MultiQuery) -> Result<Vec<Playlist>, Error> {
        trace!("Query Playlists {:?}", query);
        let playlists = self.playlists.read().unwrap().clone();
        let playlists = playlists
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
        // add artist and album
        self.tracks.write().unwrap().push(track.clone());
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
        self.albums.write().unwrap().push(album.clone());

        Ok(())
    }

    fn add_artist(&self, artist: &mut Artist) -> Result<(), Error> {
        artist.id = Some(self.artist_id.fetch_add(1, Ordering::Relaxed));
        self.artists.write().unwrap().push(artist.clone());
        Ok(())
    }

    fn add_playlist(&self, playlist: &mut Playlist) -> Result<(), Error> {
        playlist.id = Some(self.playlist_id.fetch_add(1, Ordering::Relaxed));
        self.playlists.write().unwrap().push(playlist.clone());
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
            let tracks = self.tracks.read().unwrap();
            tracks.iter().find(|a| a.uri == track.uri).map(|a| a.id)
        };

        let id: usize = has_track
            .and_then(|id| id)
            .unwrap_or_else(|| self.track_id.fetch_add(1, Ordering::Relaxed));
        track.id = Some(id);

        if has_track.is_none() {
            self.tracks.write().unwrap().push(track.clone());
        }
        Ok(())
    }

    fn sync_album(&self, album: &mut Album) -> Result<(), Error> {
        let has_album = {
            let albums = self.albums.read().unwrap();
            albums.iter().find(|a| a.uri == album.uri).map(|a| a.id)
        };

        let id: usize = has_album
            .and_then(|id| id)
            .unwrap_or_else(|| self.album_id.fetch_add(1, Ordering::Relaxed));
        album.id = Some(id);

        if has_album.is_none() {
            self.albums.write().unwrap().push(album.clone());
        }
        Ok(())
    }

    fn sync_artist(&self, artist: &mut Artist) -> Result<(), Error> {
        let has_artist = {
            let artists = self.artists.read().unwrap();
            artists.iter().find(|a| a.uri == artist.uri).map(|a| a.id)
        };

        let id: usize = has_artist
            .and_then(|id| id)
            .unwrap_or_else(|| self.artist_id.fetch_add(1, Ordering::Relaxed));
        artist.id = Some(id);

        if has_artist.is_none() {
            self.artists.write().unwrap().push(artist.clone());
        }
        Ok(())
    }

    fn sync_playlist(&self, playlist: &mut Playlist) -> Result<(), Error> {
        let (index, id) = {
            let playlists = { self.playlists.read().unwrap() };
            let index = playlists.iter().position(|a| a.uri == playlist.uri);
            let id: usize = index
                .and_then(|index| playlists[index].id)
                .unwrap_or_else(|| self.playlist_id.fetch_add(1, Ordering::Relaxed));

            (index, id)
        };
        playlist.id = Some(id);

        let mut playlists = self.playlists.write().unwrap();
        if let Some(index) = index {
            let target_playlist = playlists.get_mut(index).unwrap();
            *target_playlist = playlist.clone();
        } else {
            playlists.push(playlist.clone());
        }
        Ok(())
    }

    fn sync_tracks(&self, tracks: &mut Vec<Track>) -> Result<(), Error> {
        tracks
            .iter_mut()
            .filter(|track| {
                let tracks = self.tracks.read().unwrap();
                tracks
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
                let albums = self.albums.read().unwrap();
                albums
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
                let artists = self.artists.read().unwrap();
                artists
                    .iter()
                    .find(|t| t.uri == artist.uri)
                    .map(|_t| false)
                    .unwrap_or(true)
            })
            .map(|mut artist| self.add_artist(&mut artist))
            .collect()
    }

    fn sync_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        let stored_playlists = { self.playlists.read().unwrap().clone() };
        for playlist in playlists {
            if stored_playlists.contains(&playlist) {
                self.sync_playlist(playlist)?;
            } else {
                self.add_playlist(playlist)?;
            }
        }
        Ok(())
    }

    fn search(&self, query: String) -> Result<SearchResults, Error> {
        let tracks = self
            .tracks
            .read()
            .unwrap()
            .iter()
            .cloned()
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
