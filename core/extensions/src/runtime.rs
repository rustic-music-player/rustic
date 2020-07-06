use failure::Error;
use rustic_core::{
    Album, Artist, Library, MultiQuery, Playlist, SearchResults, SharedLibrary, SingleQuery, Track,
};

#[derive(Debug, Clone)]
pub struct ExtensionRuntime {
    library: SharedLibrary,
}

impl ExtensionRuntime {
    pub fn new(library: SharedLibrary) -> Self {
        ExtensionRuntime { library }
    }
}

impl Library for ExtensionRuntime {
    fn query_track(&self, query: SingleQuery) -> Result<Option<Track>, Error> {
        self.library.query_track(query)
    }

    fn query_tracks(&self, query: MultiQuery) -> Result<Vec<Track>, Error> {
        self.library.query_tracks(query)
    }

    fn query_album(&self, query: SingleQuery) -> Result<Option<Album>, Error> {
        self.library.query_album(query)
    }

    fn query_albums(&self, query: MultiQuery) -> Result<Vec<Album>, Error> {
        self.library.query_albums(query)
    }

    fn query_artist(&self, query: SingleQuery) -> Result<Option<Artist>, Error> {
        self.library.query_artist(query)
    }

    fn query_artists(&self, query: MultiQuery) -> Result<Vec<Artist>, Error> {
        self.library.query_artists(query)
    }

    fn query_playlist(&self, query: SingleQuery) -> Result<Option<Playlist>, Error> {
        self.library.query_playlist(query)
    }

    fn query_playlists(&self, query: MultiQuery) -> Result<Vec<Playlist>, Error> {
        self.library.query_playlists(query)
    }

    fn add_track(&self, track: &mut Track) -> Result<(), Error> {
        self.library.add_track(track)
    }

    fn add_album(&self, album: &mut Album) -> Result<(), Error> {
        self.library.add_album(album)
    }

    fn add_artist(&self, artist: &mut Artist) -> Result<(), Error> {
        self.library.add_artist(artist)
    }

    fn add_playlist(&self, playlist: &mut Playlist) -> Result<(), Error> {
        self.library.add_playlist(playlist)
    }

    fn add_tracks(&self, tracks: &mut Vec<Track>) -> Result<(), Error> {
        self.library.add_tracks(tracks)
    }

    fn add_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error> {
        self.library.add_albums(albums)
    }

    fn add_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error> {
        self.library.add_artists(artists)
    }

    fn add_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        self.library.add_playlists(playlists)
    }

    fn sync_track(&self, track: &mut Track) -> Result<(), Error> {
        self.library.sync_track(track)
    }

    fn sync_album(&self, album: &mut Album) -> Result<(), Error> {
        self.library.sync_album(album)
    }

    fn sync_artist(&self, artist: &mut Artist) -> Result<(), Error> {
        self.library.sync_artist(artist)
    }

    fn sync_playlist(&self, playlist: &mut Playlist) -> Result<(), Error> {
        self.library.sync_playlist(playlist)
    }

    fn remove_track(&self, track: &Track) -> Result<(), Error> {
        self.library.remove_track(track)
    }

    fn remove_album(&self, album: &Album) -> Result<(), Error> {
        self.library.remove_album(album)
    }

    fn remove_artist(&self, artist: &Artist) -> Result<(), Error> {
        self.library.remove_artist(artist)
    }

    fn remove_playlist(&self, playlist: &Playlist) -> Result<(), Error> {
        self.library.remove_playlist(playlist)
    }

    fn search(&self, query: String) -> Result<SearchResults, Error> {
        self.library.search(query)
    }

    fn flush(&self) -> Result<(), Error> {
        self.library.flush()
    }
}
