use std::path::Path;
use std::sync::Arc;

use bincode::{deserialize, serialize};
use failure::Error;
use serde::de::DeserializeOwned;

use rustic_core::{Album, Artist, Playlist, SearchResults, Track, SingleQuery, MultiQuery, SingleQueryIdentifier};

use crate::util::*;

pub struct SledLibrary {
    db: sled::Db,
    artists_tree: Arc<sled::Tree>,
    albums_tree: Arc<sled::Tree>,
    tracks_tree: Arc<sled::Tree>,
    playlists_tree: Arc<sled::Tree>,
}

impl SledLibrary {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<SledLibrary, Error> {
        let db = sled::Db::start_default(path)?;
        let artists_tree = db.open_tree("artists".into())?;
        let albums_tree = db.open_tree("albums".into())?;
        let tracks_tree = db.open_tree("tracks".into())?;
        let playlists_tree = db.open_tree("playlists".into())?;

        Ok(SledLibrary {
            db,
            artists_tree,
            albums_tree,
            tracks_tree,
            playlists_tree,
        })
    }

    fn next_id(&self) -> Result<usize, Error> {
        let id = self.db.generate_id()?;

        Ok(id as usize)
    }

    fn id(&self, id: Option<usize>) -> Result<Vec<u8>, Error> {
        let id = id.unwrap_or_else(|| self.next_id().unwrap());
        serialize_id(id)
    }

    fn serialize_track(&self, track: &Track) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let id = self.id(track.id)?;
        let bytes = serialize(&track)?;
        Ok((id, bytes))
    }

    fn serialize_artist(&self, artist: &Artist) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let id = self.id(artist.id)?;
        let bytes = serialize(&artist)?;
        Ok((id, bytes))
    }

    fn serialize_album(&self, album: &Album) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let id = self.id(album.id)?;
        let bytes = serialize(&album)?;
        Ok((id, bytes))
    }

    fn serialize_playlist(&self, playlist: &Playlist) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let id = self.id(playlist.id)?;
        let bytes = serialize(&playlist)?;
        Ok((id, bytes))
    }

    fn sync_entity<E, M>(&self, tree: &Arc<sled::Tree>, matches: M) -> Option<Result<E, Error>>
        where E: DeserializeOwned,
              M: Fn(&E) -> bool {
        tree
            .iter()
            .map(|item| item.map_err(Error::from).and_then(|(_, bytes)| {
                let entity: E = deserialize(&bytes)?;
                Ok(entity)
            }))
            .find(|item| match item {
                Ok(t) => matches(t),
                _ => false
            })
    }
}

impl rustic_core::Library for SledLibrary {
    fn query_track(&self, query: SingleQuery) -> Result<Option<Track>, Error> {
        match query.identifier {
            SingleQueryIdentifier::Id(id) => fetch_entity(&self.tracks_tree, id),
            _ => Ok(None)
        }
    }

    fn query_tracks(&self, query: MultiQuery) -> Result<Vec<Track>, Error> {
        fetch_entities(&self.tracks_tree)
    }

    fn query_album(&self, query: SingleQuery) -> Result<Option<Album>, Error> {
        match query.identifier {
            SingleQueryIdentifier::Id(id) => fetch_entity(&self.albums_tree, id),
            _ => Ok(None)
        }
    }

    fn query_albums(&self, query: MultiQuery) -> Result<Vec<Album>, Error> {
        fetch_entities(&self.albums_tree)
    }

    fn query_artist(&self, query: SingleQuery) -> Result<Option<Artist>, Error> {
        match query.identifier {
            SingleQueryIdentifier::Id(id) => fetch_entity(&self.artists_tree, id),
            _ => Ok(None)
        }
    }

    fn query_artists(&self, query: MultiQuery) -> Result<Vec<Artist>, Error> {
        fetch_entities(&self.artists_tree)
    }

    fn query_playlist(&self, query: SingleQuery) -> Result<Option<Playlist>, Error> {
        match query.identifier {
            SingleQueryIdentifier::Id(id) => fetch_entity(&self.playlists_tree, id),
            _ => Ok(None)
        }
    }

    fn query_playlists(&self, query: MultiQuery) -> Result<Vec<Playlist>, Error> {
        fetch_entities(&self.playlists_tree)
    }

    fn add_track(&self, track: &mut Track) -> Result<(), Error> {
        let (id, bytes) = self.serialize_track(&track)?;
        self.tracks_tree.set(id, bytes)?;
        Ok(())
    }

    fn add_album(&self, album: &mut Album) -> Result<(), Error> {
        let (id, bytes) = self.serialize_album(&album)?;
        self.albums_tree.set(id, bytes)?;
        Ok(())
    }

    fn add_artist(&self, artist: &mut Artist) -> Result<(), Error> {
        let (id, bytes) = self.serialize_artist(&artist)?;
        self.artists_tree.set(id, bytes)?;
        Ok(())
    }

    fn add_playlist(&self, playlist: &mut Playlist) -> Result<(), Error> {
        let (id, bytes) = self.serialize_playlist(&playlist)?;
        self.playlists_tree.set(id, bytes)?;
        Ok(())
    }

    fn add_tracks(&self, tracks: &mut Vec<Track>) -> Result<(), Error> {
        for mut track in tracks {
            track.id = Some(self.next_id()?);
            self.add_track(&mut track)?
        }
        Ok(())
    }

    fn add_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error> {
        for mut album in albums {
            album.id = Some(self.next_id()?);
            self.add_album(&mut album)?
        }
        Ok(())
    }

    fn add_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error> {
        for mut artist in artists {
            artist.id = Some(self.next_id()?);
            self.add_artist(&mut artist)?
        }
        Ok(())
    }

    fn add_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        for mut playlist in playlists {
            playlist.id = Some(self.next_id()?);
            self.add_playlist(&mut playlist)?
        }
        Ok(())
    }

    fn sync_track(&self, track: &mut Track) -> Result<(), Error> {
        let find_result = self.sync_entity::<Track, _>(&self.tracks_tree, |t| t.uri == track.uri);
        if let Some(found_track) = find_result {
            let id = self.id(found_track?.id)?;
            let track = serialize(track)?;
            self.tracks_tree.set(id, track)?;
        } else {
            self.add_track(track)?;
        }
        Ok(())
    }

    fn sync_album(&self, album: &mut Album) -> Result<(), Error> {
        let find_result = self.sync_entity::<Album, _>(&self.albums_tree, |a| a.uri == album.uri);
        if let Some(found_album) = find_result {
            let id = self.id(found_album?.id)?;
            let album = serialize(album)?;
            self.albums_tree.set(id, album)?;
        } else {
            self.add_album(album)?;
        }
        Ok(())
    }

    fn sync_artist(&self, artist: &mut Artist) -> Result<(), Error> {
        let find_result = self.sync_entity::<Artist, _>(&self.artists_tree, |a| a.uri == artist.uri);
        if let Some(found_artist) = find_result {
            let id = self.id(found_artist?.id)?;
            let artist = serialize(artist)?;
            self.artists_tree.set(id, artist)?;
        } else {
            self.add_artist(artist)?;
        }
        Ok(())
    }

    fn sync_playlist(&self, playlist: &mut Playlist) -> Result<(), Error> {
        let find_result = self.sync_entity::<Playlist, _>(&self.playlists_tree, |p| p.uri == playlist.uri);
        if let Some(found_playlist) = find_result {
            let id = self.id(found_playlist?.id)?;
            let playlist = serialize(playlist)?;
            self.playlists_tree.set(id, playlist)?;
        } else {
            self.add_playlist(playlist)?;
        }
        Ok(())
    }

    fn search(&self, query: String) -> Result<SearchResults, Error> {
        let tracks = search_entities(&self.tracks_tree, |track: &Track| {
            track.title.contains(&query)
        })?;
        let artists = search_entities(&self.artists_tree, |artist: &Artist| {
            artist.name.contains(&query)
        })?;
        let albums = search_entities(&self.albums_tree, |album: &Album| {
            album.title.contains(&query)
        })?;
        let playlists = search_entities(&self.playlists_tree, |playlist: &Playlist| {
            playlist.title.contains(&query)
        })?;
        Ok(SearchResults {
            albums,
            artists,
            tracks,
            playlists
        })
    }
}