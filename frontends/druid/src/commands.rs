use crate::state::{Link, Route};
use druid::Selector;

pub const NAVIGATE: Selector<Route> = Selector::new("app.navigate");

pub const PLAY_PAUSE: Selector<PlayerCommand> = Selector::new("player.play-pause");
pub const NEXT: Selector<PlayerCommand> = Selector::new("player.next");
pub const PREV: Selector<PlayerCommand> = Selector::new("player.prev");

pub const LOAD_PLAYLISTS: Selector = Selector::new("library.load-playlists");
pub const LOAD_PLAYLIST: Selector<Link> = Selector::new("library.load-playlist");

pub const LOAD_ALBUMS: Selector = Selector::new("library.load-albums");

pub const QUEUE_ALBUM: Selector<Link> = Selector::new("queue.album");
pub const QUEUE_TRACK: Selector<Link> = Selector::new("queue.track");

pub struct PlayerCommand<T = ()> {
    pub player: Option<String>,
    pub command: T,
}

impl<T> PlayerCommand<T> {
    pub fn new(player: String, command: T) -> Self {
        PlayerCommand {
            player: Some(player),
            command,
        }
    }

    pub fn default_player(command: T) -> Self {
        PlayerCommand {
            player: None,
            command,
        }
    }
}

pub mod events {
    use crate::commands::PlayerCommand;
    use crate::state::Link;
    use druid::Selector;
    use rustic_api::models::{AlbumModel, ArtistModel, PlaylistModel, TrackModel};
    use std::sync::Arc;

    pub const PLAYBACK_CHANGED: Selector<PlayerCommand<bool>> =
        Selector::new("player.playback-changed");
    pub const TRACK_CHANGED: Selector<PlayerCommand<TrackModel>> =
        Selector::new("player.track-changed");

    pub const PLAYLISTS_UPDATED: Selector<Vec<PlaylistModel>> =
        Selector::new("library.playlists-loaded");
    pub const ALBUMS_UPDATED: Selector<Vec<AlbumModel>> = Selector::new("library.albums-loaded");

    pub const PLAYLIST_LOADED: Selector<(Link, Arc<PlaylistModel>)> =
        Selector::new("library.playlist-loaded");

    pub const ALBUM_ADDED: Selector<AlbumModel> = Selector::new("library.album-added");
    pub const ALBUM_REMOVED: Selector<String> = Selector::new("library.album-removed");
    pub const TRACK_ADDED: Selector<TrackModel> = Selector::new("library.track-added");
    pub const TRACK_REMOVED: Selector<String> = Selector::new("library.track-removed");
    pub const PLAYLIST_ADDED: Selector<PlaylistModel> = Selector::new("library.playlist-added");
    pub const PLAYLIST_REMOVED: Selector<String> = Selector::new("library.playlist-removed");
    pub const ARTIST_ADDED: Selector<ArtistModel> = Selector::new("library.artist-added");
    pub const ARTIST_REMOVED: Selector<String> = Selector::new("library.artist-removed");
}
