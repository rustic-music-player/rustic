use libc::c_char;
use std::ptr;
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
        let artist = album.artist.map(FFIArtistModel::from).map(|artist| Box::into_raw(Box::new(artist)) as _).unwrap_or_else(ptr::null);

        FFIAlbumModel {
            cursor: cstr!(album.cursor),
            title: cstr!(album.title),
            artist,
            coverart: album.coverart.map(|coverart| cstr!(coverart)).unwrap_or_else(ptr::null),
            in_library: album.in_library
        }
    }
}
