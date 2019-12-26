use schema::{playlists, playlist_tracks};
use super::track::TrackEntity;

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "playlists"]
pub struct PlaylistEntity {
    pub id: i32,
    pub title: String
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(PlaylistEntity, foreign_key = "playlist_id")]
#[belongs_to(TrackEntity, foreign_key = "track_id")]
#[table_name = "playlist_tracks"]
#[primary_key(playlist_id, track_id)]
pub struct PlaylistTrack {
    pub playlist_id: i32,
    pub track_id: i32
}
