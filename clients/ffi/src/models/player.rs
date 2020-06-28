use libc::c_char;
use rustic_api::models::PlayerModel;
use std::ffi::CString;

#[derive(Debug)]
#[repr(C)]
pub struct FFIPlayerModel {
    cursor: *const c_char,
    name: *const c_char,
    playing: bool,
}

impl From<PlayerModel> for FFIPlayerModel {
    fn from(player: PlayerModel) -> Self {
        let cursor = CString::new(player.cursor).unwrap();
        let name = CString::new(player.name).unwrap();

        FFIPlayerModel {
            cursor: cursor.as_ptr(),
            name: name.as_ptr(),
            playing: player.playing,
        }
    }
}
