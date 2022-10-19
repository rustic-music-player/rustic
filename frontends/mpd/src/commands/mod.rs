use failure::Error;
use rustic_core::Rustic;
use rustic_api::ApiClient;
use std::sync::Arc;
use futures::future::BoxFuture;

mod albumart;
mod change_volume;
mod commands;
mod current_song;
mod list_artist;
mod list_info;
mod list_playlist;
mod list_playlist_info;
mod list_playlists;
mod load_playlist;
mod next;
mod outputs;
mod pause;
mod play;
mod previous;
mod set_volume;
mod status;
mod stop;
mod tagtypes;
mod playlist_info;
mod add_track;
mod toggle_pause;
mod clear_queue;

pub use self::albumart::AlbumArtCommand;
pub use self::change_volume::ChangeVolumeCommand;
pub use self::commands::CommandsCommand;
pub use self::current_song::CurrentSongCommand;
pub use self::list_artist::ListArtistCommand;
pub use self::list_info::ListInfoCommand;
pub use self::list_playlist::ListPlaylistCommand;
pub use self::list_playlist_info::ListPlaylistInfoCommand;
pub use self::list_playlists::ListPlaylistsCommand;
pub use self::load_playlist::LoadPlaylistCommand;
pub use self::next::NextCommand;
pub use self::outputs::OutputsCommand;
pub use self::pause::PauseCommand;
pub use self::play::PlayCommand;
pub use self::previous::PreviousCommand;
pub use self::set_volume::SetVolumeCommand;
pub use self::status::StatusCommand;
pub use self::stop::StopCommand;
pub use self::tagtypes::TagTypesCommand;
pub use self::playlist_info::PlaylistInfoCommand;
pub use self::add_track::AddTrackCommand;
pub use self::toggle_pause::TogglePauseCommand;
pub use self::clear_queue::ClearQueueCommand;

pub trait MpdCommand<T> {
    fn handle(&self, app: Arc<Rustic>, client: ApiClient) -> BoxFuture<Result<T, Error>>;
}
