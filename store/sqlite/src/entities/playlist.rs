use entities::provider::{provider_to_int, int_to_provider};
use rustic_core::Playlist;
use schema::{playlist_tracks, playlists};

use super::track::TrackEntity;
use entities::TrackMeta;

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "playlists"]
pub struct PlaylistEntity {
    pub id: i32,
    pub title: String,
    pub uri: String,
    pub provider: i32,
}

impl PlaylistEntity {
    pub fn into_playlist(self, tracks: Vec<(TrackEntity, Vec<TrackMeta>)>) -> Playlist {
        Playlist {
            id: Some(self.id as usize),
            title: self.title,
            uri: self.uri,
            provider: int_to_provider(self.provider),
            tracks: tracks.into_iter().map(|(track, meta)| track.into_track(&meta)).collect()
        }
    }
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(PlaylistEntity, foreign_key = "playlist_id")]
#[belongs_to(TrackEntity, foreign_key = "track_id")]
#[table_name = "playlist_tracks"]
#[primary_key(playlist_id, track_id)]
pub struct PlaylistTrack {
    pub playlist_id: i32,
    pub track_id: i32,
}

impl From<Playlist> for PlaylistInsert {
    fn from(playlist: Playlist) -> Self {
        PlaylistInsert {
            title: playlist.title,
            uri: playlist.uri,
            provider: provider_to_int(playlist.provider),
        }
    }
}

#[derive(Insertable)]
#[table_name = "playlists"]
pub struct PlaylistInsert {
    pub title: String,
    pub uri: String,
    pub provider: i32,
}
