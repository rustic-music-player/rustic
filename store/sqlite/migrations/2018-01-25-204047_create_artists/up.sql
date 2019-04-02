CREATE TABLE artists
(
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name VARCHAR(255) NOT NULL,
  image_url TEXT,
  uri TEXT NOT NULL
);
CREATE TABLE artists_meta
(
    artist_id INTEGER NOT NULL,
    key VARCHAR(255) NOT NULL,
    bool_variant INTEGER(1),
    float_variant REAL,
    string_variant TEXT,
    int_variant INTEGER(64),
    CONSTRAINT artists_meta_artist_id_fk FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE,
    CONSTRAINT artists_meta_pk PRIMARY KEY (artist_id, key)
);
CREATE UNIQUE INDEX artists_id_uindex ON artists (id);