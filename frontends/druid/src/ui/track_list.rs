use crate::state::TrackList;
use crate::widgets::hover::HoverExt;
use crate::widgets::remote_image::RemoteImage;
use crate::{commands, theme};
use druid::widget::{CrossAxisAlignment, Flex, Label, List, SizedBox};
use druid::{Data, Widget, WidgetExt};
use rustic_api::models::TrackModel;
use std::sync::Arc;

pub fn make_tracklist() -> impl Widget<TrackList> {
    List::new(|| make_track())
        .with_spacing(theme::grid(1.))
        .lens(TrackList::tracks)
}

fn make_track() -> impl Widget<Arc<TrackModel>> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(make_cover())
        .with_spacer(theme::grid(1.))
        .with_child(make_title())
        .with_spacer(theme::grid(1.))
        .with_child(make_track_duration())
        .must_fill_main_axis(true)
        .hover()
        .on_click(|ctx, track, _| ctx.submit_command(commands::QUEUE_TRACK.with(track.into())))
}

fn make_title() -> impl Widget<Arc<TrackModel>> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(make_track_title())
        .with_child(make_track_artist())
}

fn make_track_title() -> impl Widget<Arc<TrackModel>> {
    Label::dynamic(|track: &Arc<TrackModel>, _| track.title.clone()).with_font(theme::UI_FONT_BOLD)
}

fn make_track_artist() -> impl Widget<Arc<TrackModel>> {
    Label::dynamic(|track: &Arc<TrackModel>, _| {
        track
            .artist
            .as_ref()
            .map(|artist| artist.name.clone())
            .unwrap_or_default()
    })
    .with_text_color(theme::TEXT_COLOR_INACTIVE)
}

fn make_track_duration() -> impl Widget<Arc<TrackModel>> {
    Label::dynamic(|track: &Arc<TrackModel>, _| {
        track
            .duration
            .as_ref()
            .map(|duration| print_duration(*duration))
            .unwrap_or_default()
    })
    .with_font(theme::UI_FONT)
    .with_text_color(theme::TEXT_COLOR_INACTIVE)
}

fn make_cover() -> impl Widget<Arc<TrackModel>> {
    RemoteImage::new(make_placeholder(), move |track: &Arc<TrackModel>, _| {
        track.coverart.clone()
    })
    .fix_size(64., 64.)
}

fn make_placeholder<T: Data>() -> impl Widget<T> {
    SizedBox::empty().background(theme::BACKGROUND_DARK)
}

const SECONDS: u64 = 60;

fn print_duration(duration: u64) -> String {
    let minutes = duration / SECONDS;
    let seconds = duration - minutes * SECONDS;

    format!("{}:{}", minutes, seconds)
}
