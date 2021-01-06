use crate::icon::{SvgIcon, ALBUM_ICON, ARTIST_ICON, PLAYLIST_ICON, TRACK_ICON};
use crate::state::Route;
use druid::im::Vector;
use druid::widget::ListIter;
use druid::{Data, Lens};
use rustic_api::models::PlaylistModel;
use std::sync::Arc;

#[derive(Clone, Data, Default)]
pub struct SidebarState {
    pub playlists: Vector<Arc<PlaylistModel>>,
}

impl SidebarState {
    fn get_entries(&self) -> Vec<SidebarEntry> {
        let mut entries = vec![
            SidebarEntry::Header("Library".to_string()),
            SidebarEntry::NavEntry("Albums".to_string(), Route::Albums, SidebarIcon::Albums),
            SidebarEntry::NavEntry("Artists".to_string(), Route::Artists, SidebarIcon::Artists),
            SidebarEntry::NavEntry("Songs".to_string(), Route::Songs, SidebarIcon::Tracks),
            SidebarEntry::Header("Playlists".to_string()),
        ];
        for playlist in self.playlists.iter() {
            entries.push(SidebarEntry::Playlist(Arc::clone(playlist)));
        }

        entries
    }
}

impl ListIter<SidebarEntry> for SidebarState {
    fn for_each(&self, mut cb: impl FnMut(&SidebarEntry, usize)) {
        let entries = self.get_entries();
        for (i, item) in entries.iter().enumerate() {
            cb(item, i);
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut SidebarEntry, usize)) {
        let mut entries = self.get_entries();
        for (i, item) in entries.iter_mut().enumerate() {
            cb(item, i);
        }
    }

    fn data_len(&self) -> usize {
        1 + self.playlists.len()
    }
}

#[derive(Clone, Data)]
pub enum SidebarEntry {
    Header(String),
    NavEntry(String, Route, SidebarIcon),
    Playlist(Arc<PlaylistModel>),
}

#[derive(Debug, Clone, Copy, Data, PartialEq, Eq)]
pub enum SidebarIcon {
    Albums,
    Artists,
    Playlists,
    Tracks,
}

impl From<&SidebarIcon> for SvgIcon {
    fn from(icon: &SidebarIcon) -> Self {
        match icon {
            SidebarIcon::Albums => ALBUM_ICON,
            SidebarIcon::Artists => ARTIST_ICON,
            SidebarIcon::Playlists => PLAYLIST_ICON,
            SidebarIcon::Tracks => TRACK_ICON,
        }
    }
}

pub struct SidebarEntryNameLens;

impl Lens<SidebarEntry, String> for SidebarEntryNameLens {
    fn with<V, F: FnOnce(&String) -> V>(&self, data: &SidebarEntry, f: F) -> V {
        match data {
            SidebarEntry::Header(header) => f(header),
            SidebarEntry::NavEntry(nav, _, _) => f(nav),
            SidebarEntry::Playlist(playlist) => f(&playlist.title),
        }
    }

    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, data: &mut SidebarEntry, f: F) -> V {
        match data {
            SidebarEntry::Header(header) => f(header),
            SidebarEntry::NavEntry(nav, _, _) => f(nav),
            SidebarEntry::Playlist(playlist) => {
                let mut title = playlist.title.clone();
                f(&mut title)
            }
        }
    }
}
