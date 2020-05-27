CREATE TABLE albums
(
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  title VARCHAR(255) NOT NULL,
  artist_id INTEGER,
  image_url TEXT,
  uri TEXT NOT NULL,
  provider INTEGER NOT NULL,
  CONSTRAINT albums_artists_id_fk FOREIGN KEY (artist_id) REFERENCES artists (id) ON DELETE SET NULL
);
CREATE TABLE albums_meta
(
    album_id INTEGER NOT NULL,
    key VARCHAR(255) NOT NULL,
    bool_variant BOOLEAN,
    float_variant REAL,
    string_variant TEXT,
    int_variant INTEGER(64),
    CONSTRAINT albums_meta_album_id_fk FOREIGN KEY (album_id) REFERENCES albums(id) ON DELETE CASCADE,
    CONSTRAINT artists_meta_pk PRIMARY KEY (album_id, key)
);
CREATE UNIQUE INDEX albums_id_uindex ON albums (id);
