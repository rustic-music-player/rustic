use druid::widget::{CrossAxisAlignment, Flex, TextBox};
use druid::{Command, LensExt, Selector, Target, Widget, WidgetExt};

use crate::commands::{self, PlayerCommand};
use crate::icon::{Icon, SvgIcon, NEXT_ICON, PLAY_ICON, PREV_ICON};
use crate::state::{SearchState, State};
use crate::theme;

const HEADER_HEIGHT: f64 = 64.;

pub fn make_header() -> impl Widget<State> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(make_controls(), 1.0)
        .with_flex_child(make_playback_area(), 2.0)
        .with_flex_child(make_search(), 1.0)
        .must_fill_main_axis(true)
        .fix_height(HEADER_HEIGHT)
        .padding(theme::grid(2.))
        .background(theme::BACKGROUND_DARK)
}

fn make_controls() -> impl Widget<State> {
    Flex::row()
        .with_child(make_icon_button(PREV_ICON, commands::PREV))
        .with_child(make_icon_button(PLAY_ICON, commands::PLAY_PAUSE))
        .with_child(make_icon_button(NEXT_ICON, commands::NEXT))
}

fn make_icon_button(icon: SvgIcon, command: Selector<PlayerCommand>) -> impl Widget<State> {
    Icon::new(icon).with_size(48.).on_click(move |ctx, _, _| {
        let cmd = Command::new(command, PlayerCommand::default_player(()), Target::Auto);
        ctx.submit_command(cmd)
    })
}

fn make_playback_area() -> impl Widget<State> {
    Flex::column()
}

fn make_search() -> impl Widget<State> {
    TextBox::new().lens(State::search.then(SearchState::query))
}
