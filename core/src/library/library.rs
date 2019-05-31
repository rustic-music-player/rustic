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
    /**
     * Return the track for the given id or None
     */
    fn get_track(&self, id: usize) -> Result<Option<Track>, Error>;
    /**
     * Return a list of all tracks
     */
    fn get_tracks(&self) -> Result<Vec<Track>, Error>;

    /**
     * Return the album for the given id or None
     */
    fn get_album(&self, id: usize) -> Result<Option<Album>, Error>;
    /**
     * Return a list of all albums
     */
    fn get_albums(&self) -> Result<Vec<Album>, Error>;

    /**
     * Return the artist for the given id or None
     */
    fn get_artist(&self, id: usize) -> Result<Option<Artist>, Error>;
    /**
     * Return a list of all artists
     */
    fn get_artists(&self) -> Result<Vec<Artist>, Error>;

    /**
     * Return the playlist for the given id or None
     */
    fn get_playlist(&self, id: usize) -> Result<Option<Playlist>, Error>;
    /**
     * Return a list of all playlists
     */
    fn get_playlists(&self) -> Result<Vec<Playlist>, Error>;

    /**
     * Store the given track, setting the id
     */
    fn add_track(&self, track: &mut Track) -> Result<(), Error>;
    /**
     * Store the given album, setting the id
     */
    fn add_album(&self, album: &mut Album) -> Result<(), Error>;
    /**
     * Store the given artist, setting the id
     */
    fn add_artist(&self, artist: &mut Artist) -> Result<(), Error>;
    /**
     * Store the given playlist, setting the id
     */
    fn add_playlist(&self, playlist: &mut Playlist) -> Result<(), Error>;

    /**
     * Store multiple tracks, setting the ids
     */
    fn add_tracks(&self, tracks: &mut Vec<Track>) -> Result<(), Error>;
    /**
     * Store multiple albums, setting the ids
     */
    fn add_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error>;
    /**
     * Store multiple artists, setting the ids
     */
    fn add_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error>;
    /**
     * Store multiple playlists, setting the ids
     */
    fn add_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error>;

    /**
     * Sync the given track by its uri
     * Will set the id when not persisted yet
     */
    fn sync_track(&self, track: &mut Track) -> Result<(), Error>;
    /**
     * Sync the given album by its uri
     * Will set the id when not persisted yet
     */
    fn sync_album(&self, album: &mut Album) -> Result<(), Error>;
    /**
     * Sync the given artist by its uri
     * Will set the id when not persisted yet
     */
    fn sync_artist(&self, artist: &mut Artist) -> Result<(), Error>;
    /**
     * Sync the given playlist by its uri
     * Will set the id when not persisted yet
     */
    fn sync_playlist(&self, playlist: &mut Playlist) -> Result<(), Error>;

    fn sync_tracks(&self, tracks: &mut Vec<Track>) -> Result<(), Error> {
        tracks.iter_mut()
            .map(|t| self.sync_track(t))
            .collect()
    }
    fn sync_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error> {
        albums.iter_mut()
            .map(|a| self.sync_album(a))
            .collect()
    }
    fn sync_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error> {
        artists.iter_mut()
            .map(|a| self.sync_artist(a))
            .collect()
    }
    fn sync_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        playlists.iter_mut()
            .map(|p| self.sync_playlist(p))
            .collect()
    }

    fn search(&self, query: String) -> Result<SearchResults, Error>;
}
