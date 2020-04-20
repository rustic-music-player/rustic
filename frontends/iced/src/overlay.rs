use iced::button;
use rustic_core::player::Player;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub enum Overlay {
    PlayerList,
}

#[derive(Debug, Clone)]
pub enum OverlayState {
    PlayerList(Vec<(button::State, Arc<Player>)>),
}
