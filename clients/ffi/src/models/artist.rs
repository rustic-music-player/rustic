use libc::c_char;
use std::ffi::CString;
use rustic_api::models::ArtistModel;

#[derive(Debug)]
#[repr(C)]
pub struct FFIArtistModel {
    pub cursor: *const c_char,
    pub name: *const c_char,
    // pub albums:
    // pub tracks: 
    // pub playlists
    // pub image: *const c_char,
    // pub provider
}

impl From<ArtistModel> for FFIArtistModel {
    fn from(artist: ArtistModel) -> Self {
        FFIArtistModel {
            cursor: cstr!(artist.cursor),
            name: cstr!(artist.name)
        }
    }
}
