use crate::messages::Message;
use crate::SavedState;
use iced::Element;

pub trait Component {
    fn view(&mut self, state: &SavedState) -> Element<'_, Message>;
}
