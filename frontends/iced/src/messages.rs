use crate::overlay::Overlay;
use crate::views::MainView;
use std::sync::Arc;
use rustic_core::player::Player;

#[derive(Debug, Clone)]
pub enum Message {
    OpenView(MainView),
    Search(String),
    OpenOverlay(Overlay),
    SelectPlayer(Arc<Player>)
}
