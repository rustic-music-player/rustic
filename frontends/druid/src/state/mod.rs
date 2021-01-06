mod album;
mod async_data;
mod playback;
mod playlist;
mod search;
mod sidebar;

use std::sync::Arc;

use druid::im::Vector;
use druid::{Data, Lens};
use rustic_api::models::{AlbumModel, ArtistModel, PlayerModel, TrackModel};

pub use self::async_data::*;
pub use self::playback::PlaybackState;
pub use self::playlist::*;
pub use self::search::*;
pub use self::sidebar::*;

#[derive(Clone, Data, Default, Lens)]
pub struct State {
    pub route: Route,
    pub playback: PlaybackState,
    pub players: Vector<Arc<PlayerModel>>,
    pub sidebar: SidebarState,
    pub playlist: PlaylistState,
    pub albums: AsyncData<Vector<Arc<AlbumModel>>>,
    pub artists: AsyncData<Vector<Arc<ArtistModel>>>,
    pub search: SearchState,
}

#[derive(Clone, Debug, Data, Eq, PartialEq)]
pub enum Route {
    Artists,
    Albums,
    Songs,
    Search,
    PlaylistDetails(Link),
}

impl Default for Route {
    fn default() -> Self {
        Route::Albums
    }
}

#[derive(Clone, Debug, Data, Eq, PartialEq, Lens)]
pub struct Link {
    pub cursor: String,
    pub title: String,
}

impl From<&mut Arc<TrackModel>> for Link {
    fn from(track: &mut Arc<TrackModel>) -> Self {
        Link {
            cursor: track.cursor.clone(),
            title: track.title.clone(),
        }
    }
}

#[derive(Clone, Debug, Data, Lens)]
pub struct TrackList {
    pub link: Link,
    pub tracks: Vector<Arc<TrackModel>>,
}
