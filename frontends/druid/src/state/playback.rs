use druid::Data;
use rustic_api::models::TrackModel;
use std::sync::Arc;

#[derive(Clone, Data)]
pub enum PlaybackState {
    Empty,
    Playing { track: Arc<TrackModel> },
}

impl Default for PlaybackState {
    fn default() -> Self {
        PlaybackState::Empty
    }
}
