use crate::overlay::Overlay;
use crate::views::MainView;
use crate::SavedState;
use rustic_api::models::TrackModel;

#[derive(Debug, Clone)]
pub enum Message {
    OpenView(MainView),
    Search(String),
    OpenOverlay(Overlay),
    SelectPlayer(String),
    Loaded(SavedState),
    QueueTrack(TrackModel),
    QueueUpdated
}
