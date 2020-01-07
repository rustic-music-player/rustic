use super::TrackModel;

#[derive(Debug, Clone, Serialize)]
pub struct PlayerModel {
    pub cursor: String,
    pub name: String,
    pub playing: bool,
    pub current: Option<TrackModel>,
}
