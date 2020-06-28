use libc::c_char;
use rustic_api::models::PlayerModel;
use crate::models::FFITrackModel;

#[derive(Debug)]
#[repr(C)]
pub struct FFIPlayerModel {
    cursor: *const c_char,
    name: *const c_char,
    playing: bool,
    current: *const FFITrackModel,
}

impl From<PlayerModel> for FFIPlayerModel {
    fn from(player: PlayerModel) -> Self {
        FFIPlayerModel {
            cursor: cstr!(player.cursor),
            name: cstr!(player.name),
            playing: player.playing,
            current: nested_optional!(player.current, FFITrackModel)
        }
    }
}
