use crate::state::State;
use druid::widget::Label;
use druid::Widget;

pub fn make_songs() -> impl Widget<State> {
    Label::new("songs")
}
