use crate::overlay::Overlay;
use crate::views::MainView;
use crate::SavedState;
use rustic_api::models::{PlayerModel, SyncStateModel, TrackModel};

#[derive(Debug, Clone)]
pub enum Message {
    OpenView(MainView),
    Search(String),
    OpenOverlay(Overlay),
    SelectPlayer(PlayerModel),
    Loaded(SavedState),
    QueueTrack(TrackModel),
    QueueUpdated,
    Syncing(SyncStateModel),
}
