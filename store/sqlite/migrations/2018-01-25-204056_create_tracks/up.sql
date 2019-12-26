CREATE TABLE tracks
(
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  title VARCHAR(255) NOT NULL,
  artist_id INTEGER,
  album_id INTEGER,
  uri TEXT NOT NULL,
  image_url TEXT,
  duration INTEGER,
  provider INTEGER NOT NULL,
  CONSTRAINT tracks_artists_id_fk FOREIGN KEY (artist_id) REFERENCES artists (id) ON DELETE SET NULL,
  CONSTRAINT tracks_albums_id_fk FOREIGN KEY (album_id) REFERENCES albums (id) ON DELETE SET NULL
);
CREATE TABLE tracks_meta
(
    track_id INTEGER NOT NULL,
    key VARCHAR(255) NOT NULL,
    bool_variant INTEGER(1),
    float_variant REAL,
    string_variant TEXT,
    int_variant INTEGER(64),
    CONSTRAINT tracks_meta_track_id_fk FOREIGN KEY (track_id) REFERENCES tracks(id) ON DELETE CASCADE,
    CONSTRAINT artists_meta_pk PRIMARY KEY (track_id, key)
);
CREATE UNIQUE INDEX tracks_id_uindex ON tracks (id);
CREATE UNIQUE INDEX tracks_uri_uindex ON tracks (uri);