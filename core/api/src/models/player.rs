use crate::models::TrackModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerModel {
    pub cursor: String,
    pub name: String,
    pub playing: bool,
    pub volume: f32,
    pub current: Option<TrackModel>,
}
