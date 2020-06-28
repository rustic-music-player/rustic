use libc::c_char;
use rustic_api::models::AlbumModel;
use crate::models::*;

#[derive(Debug)]
#[repr(C)]
pub struct FFIAlbumModel {
    pub cursor: *const c_char,
    pub title: *const c_char,
    pub artist: *const FFIArtistModel,
    // pub tracks:
    // pub provider
    pub coverart: *const c_char,
    pub in_library: bool
}

impl From<AlbumModel> for FFIAlbumModel {
    fn from(album: AlbumModel) -> Self {
        FFIAlbumModel {
            cursor: cstr!(album.cursor),
            title: cstr!(album.title),
            artist: nested_optional!(album.artist, FFIArtistModel),
            coverart: optional_cstr!(album.coverart),
            in_library: album.in_library
        }
    }
}
