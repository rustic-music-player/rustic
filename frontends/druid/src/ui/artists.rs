use crate::state::State;
use druid::widget::Label;
use druid::Widget;

pub fn make_artists() -> impl Widget<State> {
    Label::new("artists")
}
