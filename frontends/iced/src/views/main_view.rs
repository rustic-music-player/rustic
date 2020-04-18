use crate::component::Component;
use iced::{Element, Text, Length, HorizontalAlignment, Column, Scrollable};
use crate::messages::Message;
use std::sync::Arc;
use rustic_core::{Rustic, MultiQuery};

#[derive(Debug, Clone, Copy)]
pub enum MainView {
    Library,
    Playlists,
    Explore
}

impl Default for MainView {
    fn default() -> Self {
        MainView::Library
    }
}

impl Component for MainView {
    fn view(&mut self, app: &Arc<Rustic>) -> Element<'_, Message> {
        let (title, element): (&'static str, Element<_>) = match self {
            MainView::Library => ("Library", self.default_view()),
            MainView::Playlists => ("Playlists", self.playlists_view(app)),
            MainView::Explore => ("Explore", self.default_view())
        };
        let title = Text::new(title)
            .width(Length::Fill)
            .size(100)
            .horizontal_alignment(HorizontalAlignment::Center);

        Column::new()
            .spacing(20)
            .push(title)
            .push(element)
            .into()
    }
}

impl MainView {
    fn playlists_view(&self, app: &Arc<Rustic>) -> Element<'_, Message> {
        let playlists = app.library.query_playlists(MultiQuery::new())
            .unwrap_or_default();
        let mut list = Column::new();
        for playlist in playlists {
            let item = Text::new(&playlist.title);
            list = list.push(item);
        }
        list.into()
    }

    fn default_view(&self) -> Element<'_, Message> {
        Column::new().into()
    }
}