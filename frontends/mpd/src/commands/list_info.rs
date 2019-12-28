use commands::list_playlists::PlaylistEntry;
use commands::MpdCommand;
use failure::Error;
use rustic_core::{Explorer, MultiQuery, Playlist, Rustic, SharedLibrary, Track};
use song::MpdSong;
use std::sync::Arc;

#[derive(Serialize)]
pub struct PathItem {
    directory: String,
}

pub struct ListInfoCommand {
    path: Option<String>,
}

impl ListInfoCommand {
    pub fn new(path: String) -> ListInfoCommand {
        ListInfoCommand {
            path: if path == "" { None } else { Some(path) },
        }
    }

    fn get_playlists(&self, library: &SharedLibrary) -> Result<Vec<PlaylistEntry>, Error> {
        let playlists = library
            .query_playlists(MultiQuery::new())?
            .into_iter()
            .map(PlaylistEntry::from)
            .collect();
        Ok(playlists)
    }
}

type ListInfoResponse = (Vec<PathItem>, Vec<PlaylistEntry>, Vec<MpdSong>);

impl MpdCommand<ListInfoResponse> for ListInfoCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<ListInfoResponse, Error> {
        match self.path {
            None => {
                let explorer = Explorer::new(app.providers.to_vec());
                let folders = explorer
                    .items()
                    .unwrap()
                    .folders
                    .iter()
                    .map(|folder| PathItem {
                        directory: folder.clone(),
                    })
                    .collect();
                let playlists = self.get_playlists(&app.library)?;
                Ok((folders, playlists, vec![]))
            }
            Some(ref path) => {
                let mut explorer = Explorer::new(app.providers.to_vec());
                explorer.navigate_absolute(path);
                let path = explorer.path();
                let folder = explorer.items().unwrap();
                let folders = folder
                    .folders
                    .iter()
                    .map(|folder| PathItem {
                        directory: format!("{}{}", path, folder),
                    })
                    .collect();
                let items = folder
                    .items
                    .iter()
                    .filter(|item| item.is_track())
                    .cloned()
                    .map(Track::from)
                    .map(MpdSong::from)
                    .collect();
                let playlists = folder
                    .items
                    .iter()
                    .filter(|item| item.is_track())
                    .cloned()
                    .map(Playlist::from)
                    .map(PlaylistEntry::from)
                    .collect();
                Ok((folders, playlists, items))
            }
        }
    }
}
