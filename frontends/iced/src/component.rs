use crate::messages::Message;
use iced::Element;
use crate::SavedState;

pub trait Component {
    fn view(&mut self, state: &SavedState) -> Element<'_, Message>;
}
