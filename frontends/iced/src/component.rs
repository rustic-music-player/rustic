use crate::messages::Message;
use iced::Element;
use rustic_core::Rustic;
use std::sync::Arc;

pub trait Component {
    fn view(&mut self, app: &Arc<Rustic>) -> Element<'_, Message>;
}