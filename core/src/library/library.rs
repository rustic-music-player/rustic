use std::sync::Arc;

use failure::Error;
use futures::stream::BoxStream;

use crate::library::{Album, Artist, LibraryEvent, Playlist, Track};
use crate::{MultiQuery, SingleQuery};

pub type SharedLibrary = Arc<Box<dyn Library>>;

pub struct SearchResults {
    pub tracks: Vec<Track>,
    pub albums: Vec<Album>,
    pub artists: Vec<Artist>,
    pub playlists: Vec<Playlist>,
}

pub trait Library: Sync + Send + std::fmt::Debug {
    /**
     * Fetch a single track
     */
    fn query_track(&self, query: SingleQuery) -> Result<Option<Track>, Error>;

    /**
     * Fetch multiple tracks
     */
    fn query_tracks(&self, query: MultiQuery) -> Result<Vec<Track>, Error>;

    /**
     * Return the album for the given id or None
     */
    fn query_album(&self, query: SingleQuery) -> Result<Option<Album>, Error>;
    /**
     * Return a list of all albums
     */
    fn query_albums(&self, query: MultiQuery) -> Result<Vec<Album>, Error>;

    /**
     * Return the artist for the given id or None
     */
    fn query_artist(&self, query: SingleQuery) -> Result<Option<Artist>, Error>;
    /**
     * Return a list of all artists
     */
    fn query_artists(&self, query: MultiQuery) -> Result<Vec<Artist>, Error>;

    /**
     * Return the playlist for the given id or None
     */
    fn query_playlist(&self, query: SingleQuery) -> Result<Option<Playlist>, Error>;
    /**
     * Return a list of all playlists
     */
    fn query_playlists(&self, query: MultiQuery) -> Result<Vec<Playlist>, Error>;

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
        tracks.iter_mut().try_for_each(|t| self.sync_track(t))
    }
    fn sync_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error> {
        albums.iter_mut().try_for_each(|a| self.sync_album(a))
    }
    fn sync_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error> {
        artists.iter_mut().try_for_each(|a| self.sync_artist(a))
    }
    fn sync_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        playlists.iter_mut().try_for_each(|p| self.sync_playlist(p))
    }

    fn remove_track(&self, track: &Track) -> Result<(), Error>;

    fn remove_album(&self, album: &Album) -> Result<(), Error>;

    fn remove_artist(&self, artist: &Artist) -> Result<(), Error>;

    fn remove_playlist(&self, playlist: &Playlist) -> Result<(), Error>;

    fn search(&self, query: String) -> Result<SearchResults, Error>;

    // TODO: this should happen on an interval on a background thread
    fn flush(&self) -> Result<(), Error>;

    fn observe(&self) -> BoxStream<'static, LibraryEvent>;
}
