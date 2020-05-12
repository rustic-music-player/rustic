use iced::button;
use rustic_api::models::PlayerModel;

#[derive(Debug, Clone, Copy)]
pub enum Overlay {
    PlayerList,
}

#[derive(Debug, Clone)]
pub enum OverlayState {
    PlayerList(Vec<(button::State, PlayerModel)>),
}
