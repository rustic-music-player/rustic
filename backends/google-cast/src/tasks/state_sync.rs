use std::sync::Arc;

use pinboard::NonEmptyPinboard;
use rust_cast::CastDevice;

use crate::cast_state::CastState;
use rustic_core::PlayerState;

pub struct CastStateSyncTask {
    state: Arc<NonEmptyPinboard<CastState>>,
}

impl CastStateSyncTask {
    pub fn new(state: Arc<NonEmptyPinboard<CastState>>) -> Self {
        CastStateSyncTask { state }
    }

    pub fn next(&self, device: &CastDevice<'_>) -> Result<(), failure::Error> {
        let receiver_status = device.receiver.get_status()?;
        let media_status = {
            if let Some(app) = receiver_status.applications.first() {
                let status = device.media.get_status(&app.transport_id, None)?;
                status.entries.first().cloned()
            } else {
                None
            }
        };
        let volume = if receiver_status.volume.muted.unwrap_or(false) {
            0f32
        } else {
            receiver_status.volume.level.unwrap_or_default()
        };
        let player_state = if receiver_status.is_stand_by {
            PlayerState::default()
        } else if let Some(media_status) = media_status {
            CastStateSyncTask::map_player_state(media_status.player_state)
        } else {
            PlayerState::default()
        };
        let cast_state = CastState {
            volume,
            state: player_state,
        };
        self.state.set(cast_state);
        Ok(())
    }

    fn map_player_state(state: rust_cast::channels::media::PlayerState) -> PlayerState {
        use rust_cast::channels::media::PlayerState::*;
        match state {
            Buffering | Playing => PlayerState::Play,
            Paused => PlayerState::Pause,
            Idle => PlayerState::Stop,
        }
    }
}
