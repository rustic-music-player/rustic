CREATE TABLE playlists
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title VARCHAR(255) NOT NULL
);
CREATE TABLE playlist_tracks
(
  playlist_id INTEGER NOT NULL,
  track_id INTEGER NOT NULL,
  CONSTRAINT playlist_tracks_playlist_id_fk FOREIGN KEY (playlist_id) REFERENCES playlists (id) ON DELETE CASCADE,
  CONSTRAINT playlist_tracks_track_id_fk FOREIGN KEY (track_id) REFERENCES tracks (id) ON DELETE CASCADE,
  CONSTRAINT playlist_tracks_pk PRIMARY KEY (playlist_id, track_id)
);
CREATE UNIQUE INDEX playlists_id_uindex ON playlists (id);
