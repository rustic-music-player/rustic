use std::sync::Arc;

use failure::Error;

use cursor::to_cursor;
use rustic_core::player::PlayerState;
use rustic_core::Rustic;
use viewmodels::{ExtensionModel, PlayerModel, TrackModel};

pub fn get_extensions(rustic: &Arc<Rustic>) -> Vec<ExtensionModel> {
    rustic.extensions.iter()
        .map(ExtensionModel::from)
        .collect()
}
