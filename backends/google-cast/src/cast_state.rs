use rustic_core::PlayerState;

#[derive(Clone, Debug, Default)]
pub struct CastState {
    pub state: PlayerState,
    pub volume: f32,
}
