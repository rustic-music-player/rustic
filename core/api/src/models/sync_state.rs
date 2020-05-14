use serde::{Deserialize, Serialize};
use crate::models::ProviderType;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum SyncStateModel {
    Synchronizing(Vec<SyncItemModel>),
    Idle,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SyncItemModel {
    pub provider: ProviderType,
    pub state: SyncItemStateModel,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum SyncItemStateModel {
    Idle,
    Syncing,
    Done,
    Error
}