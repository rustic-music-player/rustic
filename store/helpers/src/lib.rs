use failure::Error;

use rustic_core::{Album, Library, LibraryQueryJoins, MultiQuery, SingleQuery, Track};

pub fn join_track(
    store: &dyn Library,
    track: Track,
    joins: LibraryQueryJoins,
) -> Result<Track, Error> {
    let artist = if joins.has_artists() {
        if let Some(artist_id) = track.artist_id {
            store.query_artist(SingleQuery::id(artist_id))?
        } else {
            track.artist
        }
    } else {
        track.artist
    };
    let album = if joins.has_albums() {
        if let Some(album_id) = track.album_id {
            store.query_album(SingleQuery::id(album_id))?
        } else {
            track.album
        }
    } else {
        track.album
    };
    Ok(Track {
        album,
        artist,
        ..track
    })
}

pub fn join_album(
    store: &dyn Library,
    album: Album,
    joins: LibraryQueryJoins,
) -> Result<Album, Error> {
    let tracks = if joins.has_tracks() {
        store
            .query_tracks(MultiQuery::new())?
            .into_iter()
            .filter(|track| track.album_id == album.id)
            .collect()
    } else {
        album.tracks
    };

    let artist = if joins.has_artists() {
        if let Some(artist_id) = album.artist_id {
            store.query_artist(SingleQuery::id(artist_id))?
        } else {
            album.artist
        }
    } else {
        album.artist
    };

    Ok(Album {
        artist,
        tracks,
        ..album
    })
}

pub fn join_albums(
    store: &dyn Library,
    albums: &[Album],
    joins: LibraryQueryJoins,
) -> Result<Vec<Album>, Error> {
    let tracks = if joins.has_tracks() {
        store.query_tracks(MultiQuery::new())?
    } else {
        vec![]
    };
    let artists = if joins.has_artists() {
        store.query_artists(MultiQuery::new())?
    } else {
        vec![]
    };
    let albums = albums
        .iter()
        .map(|album| {
            let tracks = tracks
                .iter()
                .filter(|t| t.album_id == album.id)
                .cloned()
                .collect();
            let artist = artists.iter().find(|a| album.artist_id == a.id).cloned();

            Album {
                tracks,
                artist,
                ..album.clone()
            }
        })
        .collect();
    Ok(albums)
}
