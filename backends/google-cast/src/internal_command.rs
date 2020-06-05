use rustic_core::{PlayerState, Track};

pub enum InternalCommand {
    Play(Track, String),
    Volume(f32),
    SetState(PlayerState),
}
