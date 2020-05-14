use serde::{Deserialize, Serialize};

use crate::models::TrackModel;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum QueueEventModel {
    /// The queue has been changed
    QueueUpdated(Vec<TrackModel>)
}
