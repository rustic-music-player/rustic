use libc::c_char;
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
        FFIPlaylistModel {
            cursor: cstr!(playlist.cursor),
            title: cstr!(playlist.title)
        }
    }
}
