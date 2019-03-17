CREATE TABLE tracks
(
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  title VARCHAR(255) NOT NULL,
  artist_id INTEGER,
  album_id INTEGER,
  uri TEXT NOT NULL,
  coverart TEXT,
  duration INT,
  provider INT,
  CONSTRAINT tracks_artists_id_fk FOREIGN KEY (artist_id) REFERENCES artists (id) ON DELETE SET NULL,
  CONSTRAINT tracks_albums_id_fk FOREIGN KEY (album_id) REFERENCES albums (id) ON DELETE SET NULL
);
CREATE UNIQUE INDEX tracks_id_uindex ON tracks (id);
CREATE UNIQUE INDEX tracks_uri_uindex ON tracks (uri);