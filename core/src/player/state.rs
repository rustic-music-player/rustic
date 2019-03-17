use serde_derive::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PlayerState {
    Play,
    Stop,
    Pause,
}

impl Default for PlayerState {
    fn default() -> PlayerState {
        PlayerState::Stop
    }
}
