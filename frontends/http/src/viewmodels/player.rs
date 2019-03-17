use super::TrackModel;

#[derive(Debug, Clone, Serialize)]
pub struct PlayerModel {
    pub playing: bool,
    pub current: Option<TrackModel>,
}
