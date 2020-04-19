use crate::component::Component;
use crate::messages::Message;
use iced::{Column, Element, HorizontalAlignment, Length, Text, Button, button};
use rustic_core::{MultiQuery, Rustic, Album, Artist, Track, Playlist};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainView {
    Albums,
    Album(String),
    Artists,
    Tracks,
    Playlists,
    Explore,
}

impl Default for MainView {
    fn default() -> Self {
        MainView::Albums
    }
}

impl Component for MainView {
    fn view(&mut self, app: &Arc<Rustic>) -> Element<'_, Message> {
        let (title, element): (&'static str, Element<_>) = match self {
            MainView::Albums => ("Albums", self.albums_view(app)),
            MainView::Album(_) => ("Album", self.default_view()),
            MainView::Artists => ("Artists", self.artists_view(app)),
            MainView::Tracks => ("Tracks", self.tracks_view(app)),
            MainView::Playlists => ("Playlists", self.playlists_view(app)),
            MainView::Explore => ("Explore", self.default_view()),
        };
        let title = Text::new(title)
            .width(Length::Fill)
            .size(50)
            .horizontal_alignment(HorizontalAlignment::Center);

        Column::new().spacing(20).push(title).push(element).into()
    }
}

impl MainView {
    fn playlists_view(&self, app: &Arc<Rustic>) -> Element<'_, Message> {
        app
            .library
            .query_playlists(MultiQuery::new())
            .unwrap_or_default()
            .iter()
            .fold(Column::new(), |list, playlist| {
                let item = Text::new(&playlist.title);
                list.push(item)
            })
            .into()
    }

    fn albums_view(&self, app: &Arc<Rustic>) -> Element<'_, Message> {
        app
            .library
            .query_albums(MultiQuery::new())
            .unwrap_or_default()
            .iter()
            .fold(Column::new(), |list, album| {
                let name = Text::new(&album.title);
                // let btn = Button::new(state, name)
                //     .on_press(Message::OpenView(MainView::Album(album.uri.clone())));
                list.push(name)
            })
            .into()
    }

    fn artists_view(&self, app: &Arc<Rustic>) -> Element<'_, Message> {
        app
            .library
            .query_artists(MultiQuery::new())
            .unwrap_or_default()
            .iter()
            .fold(Column::new(), |list, artist| {
                let c = Text::new(&artist.name);
                list.push(c)
            })
            .into()
    }

    fn tracks_view(&self, app: &Arc<Rustic>) -> Element<'_, Message> {
        app
            .library
            .query_tracks(MultiQuery::new())
            .unwrap_or_default()
            .iter()
            .fold(Column::new(), |list, track| {
                let c = Text::new(&track.title);
                list.push(c)
            })
            .into()
    }

    fn default_view(&self) -> Element<'_, Message> {
        Column::new().into()
    }
}
