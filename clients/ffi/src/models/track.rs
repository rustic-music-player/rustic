use libc::*;
use rustic_api::models::TrackModel;
use crate::models::*;

#[derive(Debug)]
#[repr(C)]
pub struct FFITrackModel {
    pub cursor: *const c_char,
    pub title: *const c_char,
    pub artist: *const FFIArtistModel,
    pub album: *const FFIAlbumModel,
    pub coverart: *const c_char,
    // pub duration: *const c_ulong
    // pub provider
}

impl From<TrackModel> for FFITrackModel {
    fn from(track: TrackModel) -> Self {
        FFITrackModel {
            cursor: cstr!(track.cursor),
            title: cstr!(track.title),
            coverart: optional_cstr!(track.coverart),
            artist: nested_optional!(track.artist, FFIArtistModel),
            album: nested_optional!(track.album, FFIAlbumModel)
            // duration: track.duration,
        }
    }
}
