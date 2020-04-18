use rustic_core::{PlayerState, Track};

pub enum InternalCommand {
    Play(Track),
    Volume(f32),
    SetState(PlayerState),
}
