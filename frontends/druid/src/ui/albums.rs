use std::sync::Arc;

use druid::im::Vector;
use druid::widget::{Container, CrossAxisAlignment, Flex, Label, LineBreaking, SizedBox};
use druid::{Data, Widget, WidgetExt};

use rustic_api::models::AlbumModel;

use crate::state::State;
use crate::theme;
use crate::widgets::grid::GridList;
use crate::widgets::hover::HoverExt;
use crate::widgets::remote_image::RemoteImage;
use crate::widgets::Async;

const ALBUM_WIDTH: f64 = 178.;

pub fn make_albums() -> impl Widget<State> {
    Async::new(
        || Label::new("Loading..."),
        || make_album_list(),
        || Label::new("some error"),
    )
    .lens(State::albums)
}

fn make_album_list() -> impl Widget<Vector<Arc<AlbumModel>>> {
    GridList::<Arc<AlbumModel>>::new(ALBUM_WIDTH, || make_album_entry(), theme::grid(2.))
        .padding(theme::grid(2.))
}

fn make_album_entry() -> impl Widget<Arc<AlbumModel>> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(make_cover())
        .with_child(make_album_title())
        .with_child(make_album_artist())
        .fix_width(ALBUM_WIDTH)
        .hover()
}

fn make_cover() -> impl Widget<Arc<AlbumModel>> {
    Container::new(
        RemoteImage::new(make_placeholder(), move |album: &Arc<AlbumModel>, _| {
            album.coverart.clone()
        })
        .fix_size(ALBUM_WIDTH, ALBUM_WIDTH),
    )
    .rounded(16.)
}

fn make_album_title() -> impl Widget<Arc<AlbumModel>> {
    Label::dynamic(|album: &Arc<AlbumModel>, _| album.title.clone())
        .with_font(theme::UI_FONT_BOLD)
        .with_line_break_mode(LineBreaking::Clip)
}

fn make_album_artist() -> impl Widget<Arc<AlbumModel>> {
    Label::dynamic(|album: &Arc<AlbumModel>, _| {
        album
            .artist
            .as_ref()
            .map(|artist| artist.name.clone())
            .unwrap_or_default()
    })
    .with_text_color(theme::TEXT_COLOR_INACTIVE)
}

pub fn make_placeholder<T: Data>() -> impl Widget<T> {
    SizedBox::empty().background(theme::BACKGROUND_DARK)
}
