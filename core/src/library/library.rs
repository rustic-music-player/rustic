use failure::Error;
use crate::library::{Album, Artist, Playlist, Track};
use std::sync::Arc;

pub type SharedLibrary = Arc<Box<dyn Library>>;

pub struct SearchResults {
    pub tracks: Vec<Track>,
    pub albums: Vec<Album>,
    pub artists: Vec<Artist>,
    pub playlists: Vec<Playlist>,
}

pub trait Library: Sync + Send {
    fn get_track(&self, id: usize) -> Result<Option<Track>, Error>;
    fn get_tracks(&self) -> Result<Vec<Track>, Error>;

    fn get_album(&self, id: usize) -> Result<Option<Album>, Error>;
    fn get_albums(&self) -> Result<Vec<Album>, Error>;

    fn get_artist(&self, id: usize) -> Result<Option<Artist>, Error>;
    fn get_artists(&self) -> Result<Vec<Artist>, Error>;

    fn get_playlist(&self, id: usize) -> Result<Option<Playlist>, Error>;
    fn get_playlists(&self) -> Result<Vec<Playlist>, Error>;

    fn add_track(&self, track: &mut Track) -> Result<(), Error>;
    fn add_album(&self, album: &mut Album) -> Result<(), Error>;
    fn add_artist(&self, artist: &mut Artist) -> Result<(), Error>;
    fn add_playlist(&self, playlist: &mut Playlist) -> Result<(), Error>;

    fn add_tracks(&self, tracks: &mut Vec<Track>) -> Result<(), Error>;
    fn add_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error>;
    fn add_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error>;
    fn add_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error>;

    fn sync_track(&self, track: &mut Track) -> Result<(), Error>;
    fn sync_album(&self, album: &mut Album) -> Result<(), Error>;
    fn sync_artist(&self, artist: &mut Artist) -> Result<(), Error>;
    fn sync_playlist(&self, playlist: &mut Playlist) -> Result<(), Error>;

    fn sync_tracks(&self, tracks: &mut Vec<Track>) -> Result<(), Error>;
    fn sync_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error>;
    fn sync_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error>;
    fn sync_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error>;

    fn search(&self, query: String) -> Result<SearchResults, Error>;
}
