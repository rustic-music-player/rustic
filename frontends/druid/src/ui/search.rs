use crate::state::{SearchResults, SearchState, State};
use crate::widgets::Async;
use druid::widget::{Flex, Label};
use druid::{LensExt, Widget, WidgetExt};

pub fn make_search() -> impl Widget<State> {
    Async::new(
        || Label::dynamic(|state: &String, _| state.clone()),
        || make_search_results(),
        || Label::new("failed"),
    )
    .lens(State::search.then(SearchState::results))
}

fn make_search_results() -> impl Widget<SearchResults> {
    Flex::column()
        .with_child(make_artist_results())
        .with_child(make_album_results())
        .with_child(make_playlist_results())
        .with_child(make_track_results())
}

fn make_artist_results() -> impl Widget<SearchResults> {
    Flex::column().lens(SearchResults::artists)
}

fn make_album_results() -> impl Widget<SearchResults> {
    Flex::column().lens(SearchResults::albums)
}

fn make_track_results() -> impl Widget<SearchResults> {
    Flex::column().lens(SearchResults::tracks)
}

fn make_playlist_results() -> impl Widget<SearchResults> {
    Flex::column().lens(SearchResults::playlists)
}
