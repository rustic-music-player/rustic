use libc::*;
use std::ffi::CString;
use std::ptr;
use rustic_api::models::TrackModel;
use crate::models::*;

#[derive(Debug)]
#[repr(C)]
pub struct FFITrackModel {
    pub cursor: *const c_char,
    pub title: *const c_char,
    // pub artist: *const FFIArtistModel,
    // pub album: *const FFIAlbumModel,
    pub coverart: *const c_char,
    // pub duration: *const c_ulong
    // pub provider
}

impl From<TrackModel> for FFITrackModel {
    fn from(track: TrackModel) -> Self {
        let cursor = CString::new(track.cursor).unwrap();
        let title = CString::new(track.title).unwrap();
        let coverart = track.coverart.map(|c| CString::new(c).unwrap().as_ptr()).unwrap_or_else(|| ptr::null());

        FFITrackModel {
            cursor: cursor.as_ptr(),
            title: title.as_ptr(),
            coverart: coverart,
            // duration: track.duration,
            // artist: track.artist.map(FFIArtistModel::from).unwrap_or_else(|| ptr::null()),
            // album: track.album.map(FFIAlbumModel::from).unwrap_or_else(|| ptr::null())
        }
    }
}
