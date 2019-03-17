use failure::Error;
use rustic_core::{Album, Artist, Library, Playlist, SearchResults, Track};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    RwLock,
};

#[derive(Debug, Default)]
pub struct MemoryLibrary {
    album_id: AtomicUsize,
    artist_id: AtomicUsize,
    track_id: AtomicUsize,
    playlist_id: AtomicUsize,
    pub albums: RwLock<Vec<Album>>,
    pub artists: RwLock<Vec<Artist>>,
    pub tracks: RwLock<Vec<Track>>,
    pub playlists: RwLock<Vec<Playlist>>,
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
    fn get_track(&self, id: usize) -> Result<Option<Track>, Error> {
        let track = self
            .tracks
            .read()
            .unwrap()
            .iter()
            .cloned()
            .find(|track| track.id == Some(id));
        Ok(track)
    }

    fn get_tracks(&self) -> Result<Vec<Track>, Error> {
        let tracks = self.tracks.read().unwrap().clone();
        Ok(tracks)
    }

    fn get_album(&self, id: usize) -> Result<Option<Album>, Error> {
        let album = self
            .albums
            .read()
            .unwrap()
            .iter()
            .cloned()
            .find(|album| album.id == Some(id));
        Ok(album)
    }

    fn get_albums(&self) -> Result<Vec<Album>, Error> {
        let albums = self.albums.read().unwrap().clone();
        Ok(albums)
    }

    fn get_artist(&self, id: usize) -> Result<Option<Artist>, Error> {
        let artist = self
            .artists
            .read()
            .unwrap()
            .iter()
            .cloned()
            .find(|artist| artist.id == Some(id));
        Ok(artist)
    }

    fn get_artists(&self) -> Result<Vec<Artist>, Error> {
        let artists = self.artists.read().unwrap().clone();
        Ok(artists)
    }

    fn get_playlist(&self, id: usize) -> Result<Option<Playlist>, Error> {
        let playlist = self
            .playlists
            .read()
            .unwrap()
            .iter()
            .cloned()
            .find(|playlist| playlist.id == Some(id));
        Ok(playlist)
    }

    fn get_playlists(&self) -> Result<Vec<Playlist>, Error> {
        let playlists = self.playlists.read().unwrap().clone();
        Ok(playlists)
    }

    fn add_track(&self, track: &mut Track) -> Result<(), Error> {
        track.id = Some(self.track_id.fetch_add(1, Ordering::Relaxed));
        self.tracks.write().unwrap().push(track.clone());
        Ok(())
    }

    fn add_album(&self, album: &mut Album) -> Result<(), Error> {
        album.id = Some(self.album_id.fetch_add(1, Ordering::Relaxed));
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
        let tracks = tracks.iter().cloned().map(|mut track| {
            track.id = Some(self.track_id.fetch_add(1, Ordering::Relaxed));
            track
        });
        self.tracks.write().unwrap().extend(tracks);
        Ok(())
    }

    fn add_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error> {
        let albums = albums.iter().cloned().map(|mut album| {
            album.id = Some(self.album_id.fetch_add(1, Ordering::Relaxed));
            album
        });
        self.albums.write().unwrap().extend(albums);
        Ok(())
    }

    fn add_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error> {
        let artists = artists.iter().cloned().map(|mut artist| {
            artist.id = Some(self.artist_id.fetch_add(1, Ordering::Relaxed));
            artist
        });
        self.artists.write().unwrap().extend(artists);
        Ok(())
    }

    fn add_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        let playlists = playlists.iter().cloned().map(|mut playlist| {
            playlist.id = Some(self.playlist_id.fetch_add(1, Ordering::Relaxed));
            playlist
        });
        self.playlists.write().unwrap().extend(playlists);
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
        let has_playlist = {
            let playlists = self.playlists.read().unwrap();
            playlists
                .iter()
                .find(|a| a.uri == playlist.uri)
                .map(|a| a.id)
        };

        let id: usize = has_playlist
            .and_then(|id| id)
            .unwrap_or_else(|| self.playlist_id.fetch_add(1, Ordering::Relaxed));
        playlist.id = Some(id);

        if has_playlist.is_none() {
            self.playlists.write().unwrap().push(playlist.clone());
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
            }).map(|mut track| self.add_track(&mut track))
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
            }).map(|mut album| self.add_album(&mut album))
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
            }).map(|mut artist| self.add_artist(&mut artist))
            .collect()
    }

    fn sync_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        playlists
            .iter_mut()
            .filter(|playlist| {
                let playlists = self.playlists.read().unwrap();
                playlists
                    .iter()
                    .find(|p| p.uri == playlist.uri)
                    .map(|_p| false)
                    .unwrap_or(true)
            }).map(|mut p| self.add_playlist(&mut p))
            .collect()
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
