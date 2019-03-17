use failure::Error;
use rustic_core::Rustic;
use std::sync::Arc;

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

pub trait MpdCommand<T> {
    fn handle(&self, app: &Arc<Rustic>) -> Result<T, Error>;
}
