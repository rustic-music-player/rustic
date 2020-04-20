use crate::overlay::Overlay;
use crate::views::MainView;
use rustic_core::player::Player;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Message {
    OpenView(MainView),
    Search(String),
    OpenOverlay(Overlay),
    SelectPlayer(Arc<Player>),
}
