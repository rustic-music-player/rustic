use std::sync::Arc;

use druid::widget::{CrossAxisAlignment, Flex, Label};
use druid::{Widget, WidgetExt};

use rustic_api::models::PlaylistModel;

use crate::state::{Link, PlaylistState, State, TrackList};
use crate::theme;
use crate::ui::track_list::make_tracklist;
use crate::widgets::Async;

pub fn make_playlist_details() -> impl Widget<State> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Async::<Arc<PlaylistModel>, Link, _>::new(
                || Label::dynamic(|title, _| format!("Loading {}...", title)).lens(Link::title),
                || make_playlist_header(),
                || Label::new("failed"),
            )
            .lens(PlaylistState::playlist),
        )
        .with_spacer(theme::grid(1.0))
        .with_child(
            Async::<TrackList, Link, _>::new(
                || Label::dynamic(|title, _| format!("Loading {}...", title)).lens(Link::title),
                || make_tracklist(),
                || Label::new("failed"),
            )
            .lens(PlaylistState::tracks),
        )
        .padding(theme::grid(2.))
        .lens(State::playlist)
}

pub fn make_playlist_header() -> impl Widget<Arc<PlaylistModel>> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(make_playlist_title())
        .with_child(make_playlist_subtitle())
}

pub fn make_playlist_title() -> impl Widget<Arc<PlaylistModel>> {
    Label::dynamic(|playlist: &Arc<PlaylistModel>, _| playlist.title.clone())
        .with_text_size(theme::DEFAULT_FONT_SIZE * 1.5)
}

const MINUTE: u64 = 60;
const HOUR: u64 = MINUTE * 60;

pub fn make_playlist_subtitle() -> impl Widget<Arc<PlaylistModel>> {
    Label::dynamic(|playlist: &Arc<PlaylistModel>, _| {
        let track_count = playlist.tracks.len();
        let total_duration: u64 = playlist
            .tracks
            .iter()
            .filter_map(|track| track.duration)
            .sum();

        let hours = total_duration / HOUR;

        if hours > 0 {
            let minutes = (total_duration - hours * HOUR) / MINUTE;

            format!(
                "{} songs - {} hours, {} minutes",
                track_count, hours, minutes
            )
        } else {
            let minutes = total_duration / MINUTE;

            format!("{} songs - {} minutes", track_count, minutes)
        }
    })
    .with_text_color(theme::TEXT_COLOR_INACTIVE)
}
