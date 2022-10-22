use rustic_core::provider::ThumbnailState;
pub use self::album::*;
pub use self::playlist::*;
pub use self::track::*;
pub use self::artist::*;

mod album;
mod artist;
mod playlist;
mod track;

pub fn map_thumbnail(thumbnails: Vec<ytmusic::Thumbnail>) -> ThumbnailState {
    thumbnails.iter()
        .max_by_key(|thumb| thumb.width)
        .map(|thumb| ThumbnailState::Url(thumb.url.clone()))
        .unwrap_or(ThumbnailState::None)
}
