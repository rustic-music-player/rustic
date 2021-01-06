use druid::widget::{Flex, Scroll, Split, ViewSwitcher};
use druid::{MenuDesc, Widget, WidgetExt, WindowDesc};

use self::albums::make_albums;
use self::artists::make_artists;
use self::header::make_header;
use self::playlist::make_playlist_details;
use self::search::make_search;
use self::sidebar::make_sidebar;
use self::songs::make_songs;
use crate::state::{Route, State};
use crate::theme;

mod albums;
mod artists;
mod header;
mod playlist;
mod sidebar;
mod songs;
mod track_list;

fn build_ui() -> impl Widget<State> {
    let sidebar = make_sidebar();
    let content = make_content();
    let main = Split::columns(sidebar, content)
        .split_point(0.2)
        .bar_size(1.0)
        .min_size(150., 0.)
        .min_bar_area(1.)
        .solid_bar(true);

    Flex::column()
        .must_fill_main_axis(true)
        .with_child(make_header())
        .with_flex_child(main, 1.0)
}

fn make_content() -> impl Widget<State> {
    Scroll::new(ViewSwitcher::<State, Route>::new(
        |state, _| state.route.clone(),
        |route, _, _| match route {
            Route::Albums => make_albums().boxed(),
            Route::Artists => make_artists().boxed(),
            Route::Songs => make_songs().boxed(),
            Route::Search => make_search().boxed(),
            Route::PlaylistDetails(_) => make_playlist_details().boxed(),
        },
    ))
    .background(theme::WINDOW_BACKGROUND_COLOR)
}

pub fn main_window() -> WindowDesc<State> {
    WindowDesc::new(build_ui).title("Rustic").menu(make_menu())
}

fn make_menu() -> MenuDesc<State> {
    let menu = MenuDesc::empty();

    menu
}

mod search;
