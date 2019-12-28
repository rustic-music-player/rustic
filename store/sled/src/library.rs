use std::path::Path;
use std::sync::Arc;

use bincode::serialize;
use failure::{err_msg, Error};

use rustic_core::{
    Album, Artist, MultiQuery, Playlist, SearchResults, SingleQuery, SingleQueryIdentifier, Track,
};
use rustic_store_helpers::{join_album, join_albums, join_track};

use crate::util::*;

/// **Experimental**
///
/// # TODO: optimize joins with associations tree and maybe the following instead of plain entities
/// ```rust
/// enum SledAssociation {
///     Track(usize),
///     Album(usize),
///     Artist(usize),
///     Playlist(usize)
/// }
///
/// struct SledEntity<E> {
///     entity: E,
///     associations: Vec<SledAssociation>
/// }
/// ```
///
/// Also we could update associations on changes
/// would make writes slower but reads way faster
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

    fn id(&self, id: Option<usize>) -> Result<usize, Error> {
        if let Some(id) = id {
            Ok(id)
        } else {
            self.next_id()
        }
    }

    fn serialize_track(&self, track: &mut Track) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let id = track
            .id
            .ok_or(err_msg("missing id"))
            .and_then(serialize_id)?;
        let bytes = serialize(&track)?;
        Ok((id, bytes))
    }

    fn serialize_artist(&self, artist: &Artist) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let id = artist
            .id
            .ok_or(err_msg("missing id"))
            .and_then(serialize_id)?;
        let bytes = serialize(&artist)?;
        Ok((id, bytes))
    }

    fn serialize_album(&self, album: &Album) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let id = album
            .id
            .ok_or(err_msg("missing id"))
            .and_then(serialize_id)?;
        let bytes = serialize(&album)?;
        Ok((id, bytes))
    }

    fn serialize_playlist(&self, playlist: &Playlist) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let id = playlist
            .id
            .ok_or(err_msg("missing id"))
            .and_then(serialize_id)?;
        let bytes = serialize(&playlist)?;
        Ok((id, bytes))
    }
}

impl rustic_core::Library for SledLibrary {
    fn query_track(&self, query: SingleQuery) -> Result<Option<Track>, Error> {
        let entity = match query.identifier {
            SingleQueryIdentifier::Id(id) => fetch_entity(&self.tracks_tree, id),
            _ => Ok(None),
        }?;
        match entity {
            Some(track) => Ok(Some(join_track(self, track, query.joins)?)),
            _ => Ok(None),
        }
    }

    fn query_tracks(&self, query: MultiQuery) -> Result<Vec<Track>, Error> {
        fetch_entities(&self.tracks_tree)
    }

    fn query_album(&self, query: SingleQuery) -> Result<Option<Album>, Error> {
        let entity = match query.identifier {
            SingleQueryIdentifier::Id(id) => fetch_entity(&self.albums_tree, id),
            _ => Ok(None),
        }?;
        match entity {
            Some(album) => Ok(Some(join_album(self, album, query.joins)?)),
            _ => Ok(None),
        }
    }

    fn query_albums(&self, query: MultiQuery) -> Result<Vec<Album>, Error> {
        let albums = fetch_entities(&self.albums_tree)?;
        join_albums(self, &albums, query.joins)
    }

    fn query_artist(&self, query: SingleQuery) -> Result<Option<Artist>, Error> {
        match query.identifier {
            SingleQueryIdentifier::Id(id) => fetch_entity(&self.artists_tree, id),
            _ => Ok(None),
        }
    }

    fn query_artists(&self, query: MultiQuery) -> Result<Vec<Artist>, Error> {
        fetch_entities(&self.artists_tree)
    }

    fn query_playlist(&self, query: SingleQuery) -> Result<Option<Playlist>, Error> {
        match query.identifier {
            SingleQueryIdentifier::Id(id) => fetch_entity(&self.playlists_tree, id),
            _ => Ok(None),
        }
    }

    fn query_playlists(&self, query: MultiQuery) -> Result<Vec<Playlist>, Error> {
        fetch_entities(&self.playlists_tree)
    }

    fn add_track(&self, track: &mut Track) -> Result<(), Error> {
        track.id = Some(self.id(track.id)?);
        let (id, bytes) = self.serialize_track(track)?;
        self.tracks_tree.set(id, bytes)?;
        Ok(())
    }

    fn add_album(&self, album: &mut Album) -> Result<(), Error> {
        album.id = Some(self.id(album.id)?);
        let (id, bytes) = self.serialize_album(&album)?;
        self.albums_tree.set(id, bytes)?;
        Ok(())
    }

    fn add_artist(&self, artist: &mut Artist) -> Result<(), Error> {
        artist.id = Some(self.id(artist.id)?);
        let (id, bytes) = self.serialize_artist(&artist)?;
        self.artists_tree.set(id, bytes)?;
        Ok(())
    }

    fn add_playlist(&self, playlist: &mut Playlist) -> Result<(), Error> {
        playlist.id = Some(self.id(playlist.id)?);
        let (id, bytes) = self.serialize_playlist(&playlist)?;
        self.playlists_tree.set(id, bytes)?;
        Ok(())
    }

    fn add_tracks(&self, tracks: &mut Vec<Track>) -> Result<(), Error> {
        for mut track in tracks {
            self.add_track(&mut track)?
        }
        Ok(())
    }

    fn add_albums(&self, albums: &mut Vec<Album>) -> Result<(), Error> {
        for mut album in albums {
            self.add_album(&mut album)?
        }
        Ok(())
    }

    fn add_artists(&self, artists: &mut Vec<Artist>) -> Result<(), Error> {
        for mut artist in artists {
            self.add_artist(&mut artist)?
        }
        Ok(())
    }

    fn add_playlists(&self, playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        for mut playlist in playlists {
            self.add_playlist(&mut playlist)?
        }
        Ok(())
    }

    fn sync_track(&self, track: &mut Track) -> Result<(), Error> {
        let find_result = find_entity::<Track, _>(&self.tracks_tree, |t| t.uri == track.uri)?;
        if let Some(found_track) = find_result {
            let id = self.id(found_track.id)?;
            track.id = Some(id);
            let id = serialize_id(id)?;
            let track = serialize(track)?;
            self.tracks_tree.set(id, track)?;
        } else {
            self.add_track(track)?;
        }
        Ok(())
    }

    fn sync_album(&self, album: &mut Album) -> Result<(), Error> {
        let find_result = find_entity::<Album, _>(&self.albums_tree, |a| a.uri == album.uri)?;
        if let Some(found_album) = find_result {
            let id = self.id(found_album.id)?;
            album.id = Some(id);
            let id = serialize_id(id)?;
            let album = serialize(album)?;
            self.albums_tree.set(id, album)?;
        } else {
            self.add_album(album)?;
        }
        Ok(())
    }

    fn sync_artist(&self, artist: &mut Artist) -> Result<(), Error> {
        let find_result = find_entity::<Artist, _>(&self.artists_tree, |a| a.uri == artist.uri)?;
        if let Some(found_artist) = find_result {
            let id = self.id(found_artist.id)?;
            artist.id = Some(id);
            let id = serialize_id(id)?;
            let artist = serialize(artist)?;
            self.artists_tree.set(id, artist)?;
        } else {
            self.add_artist(artist)?;
        }
        Ok(())
    }

    fn sync_playlist(&self, playlist: &mut Playlist) -> Result<(), Error> {
        let find_result =
            find_entity::<Playlist, _>(&self.playlists_tree, |p| p.uri == playlist.uri)?;
        if let Some(found_playlist) = find_result {
            let id = self.id(found_playlist.id)?;
            playlist.id = Some(id);
            let id = serialize_id(id)?;
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
            playlists,
        })
    }
}
