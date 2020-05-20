use libc::c_char;
use rustic_api::models::PlayerModel;
use std::ffi::CString;

#[derive(Debug)]
#[repr(C)]
pub struct Player {
    cursor: *const c_char,
    name: *const c_char,
    playing: bool,
}

impl From<PlayerModel> for Player {
    fn from(player: PlayerModel) -> Self {
        let cursor = CString::new(player.cursor).unwrap();
        let name = CString::new(player.name).unwrap();

        Player {
            cursor: cursor.as_ptr(),
            name: name.as_ptr(),
            playing: player.playing,
        }
    }
}
