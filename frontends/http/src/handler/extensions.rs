use std::sync::Arc;

use rustic_core::Rustic;
use viewmodels::ExtensionModel;

pub fn get_extensions(rustic: &Arc<Rustic>) -> Vec<ExtensionModel> {
    rustic.extensions.iter().map(ExtensionModel::from).collect()
}
