use libc::c_char;
use std::ffi::CString;
use rustic_api::models::PlaylistModel;

#[derive(Debug)]
#[repr(C)]
pub struct FFIPlaylistModel {
    pub cursor: *const c_char,
    pub title: *const c_char,
    // pub tracks:
    // pub provider
}

impl From<PlaylistModel> for FFIPlaylistModel {
    fn from(playlist: PlaylistModel) -> Self {
        let cursor = CString::new(playlist.cursor).unwrap();
        let title = CString::new(playlist.title).unwrap();

        FFIPlaylistModel {
            cursor: cursor.as_ptr(),
            title: title.as_ptr()
        }
    }
}
