use log::error;
use failure::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct Track {
    pub path: String,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Scanner {
    path: String,
}

fn is_mp3(entry: &walkdir::DirEntry) -> bool {
    if entry.file_type().is_file() {
        entry
            .file_name()
            .to_str()
            .map(|s| s.ends_with(".mp3"))
            .unwrap_or(false)
    } else {
        true
    }
}

impl Scanner {
    pub fn new<P: Into<String>>(path: P) -> Scanner {
        Scanner { path: path.into() }
    }

    pub fn scan(&self) -> Result<Vec<Track>, Error> {
        walkdir::WalkDir::new(&self.path)
            .into_iter()
            .filter_entry(|e| is_mp3(e))
            .map(|entry| {
                entry.map_err(failure::Error::from).and_then(|entry| {
                    let path = entry
                        .path()
                        .to_str()
                        .ok_or_else(|| failure::err_msg("Invalid Path"))?
                        .to_string();
                    let filename = entry
                        .file_name()
                        .to_str()
                        .ok_or_else(|| failure::err_msg("Invalid Filename"))?;
                    match id3::Tag::read_from_path(entry.path()) {
                        Ok(tag) => {
                            let title = tag.title().unwrap_or(filename).to_string();
                            let artist = tag.artist().map(|s| s.to_string());
                            let album = tag.album().map(|s| s.to_string());

                            Ok(Track {
                                path,
                                title,
                                artist,
                                album,
                            })
                        }
                        Err(id3::Error {
                            kind: id3::ErrorKind::NoTag,
                            ..
                        }) => Ok(Track {
                            path,
                            title: filename.to_string(),
                            artist: None,
                            album: None,
                        }),
                        Err(e) => {
                            error!("{:?} {:?}", entry.path(), e);
                            Err(failure::Error::from(e))
                        }
                    }
                })
            }).filter(|track| track.is_ok())
            .collect()
    }
}
