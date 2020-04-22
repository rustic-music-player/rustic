use crate::models::TrackModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerModel {
    pub cursor: String,
    pub name: String,
    pub playing: bool,
    pub current: Option<TrackModel>,
}
