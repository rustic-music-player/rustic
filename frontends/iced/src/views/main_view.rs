use crate::component::Component;
use crate::messages::Message;
use crate::SavedState;
use iced::{button, Column, Element, HorizontalAlignment, Length, Text};
use rustic_api::models::{AlbumModel, ArtistModel, PlaylistModel, TrackModel};

type ActionList<T> = Vec<(button::State, T)>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainView {
    Albums,
    Album(String),
    Artists,
    Tracks,
    Playlists(ActionList<PlaylistModel>),
    Playlist((String, ActionList<TrackModel>)),
    Explore,
}

impl Default for MainView {
    fn default() -> Self {
        MainView::Albums
    }
}

impl Component for MainView {
    fn view(&mut self, state: &SavedState) -> Element<'_, Message> {
        let (title, element): (String, Element<_>) = match self {
            MainView::Albums => ("Albums".into(), self.albums_view(&state.albums)),
            MainView::Album(_) => ("Album".into(), self.default_view()),
            MainView::Artists => ("Artists".into(), self.artists_view(&state.artists)),
            MainView::Tracks => ("Tracks".into(), self.tracks_view(&state.tracks)),
            MainView::Playlists(l) => ("Playlists".into(), MainView::playlists_view(l)),
            MainView::Playlist((cursor, tracks)) => {
                let playlist = state
                    .playlists
                    .iter()
                    .find(|p| &p.cursor == cursor)
                    .cloned()
                    .unwrap();
                (playlist.title.clone(), MainView::playlist_view(tracks))
            }
            MainView::Explore => ("Explore".into(), self.default_view()),
        };
        let title = Text::new(title)
            .width(Length::Fill)
            .size(50)
            .horizontal_alignment(HorizontalAlignment::Center);

        Column::new().spacing(20).push(title).push(element).into()
    }
}

impl MainView {
    fn playlists_view(playlists: &mut Vec<(button::State, PlaylistModel)>) -> Element<Message> {
        playlists
            .iter_mut()
            .fold(Column::new(), |list, (state, playlist)| {
                let btn = button::Button::new(state, Text::new(&playlist.title)).on_press(
                    Message::OpenView(MainView::Playlist((
                        playlist.cursor.clone(),
                        playlist
                            .tracks
                            .iter()
                            .map(|track| (button::State::new(), track.clone()))
                            .collect(),
                    ))),
                );
                list.push(btn)
            })
            .into()
    }

    fn playlist_view(tracks: &mut ActionList<TrackModel>) -> Element<Message> {
        tracks
            .iter_mut()
            .fold(Column::new(), |list, (state, track)| {
                let btn = button::Button::new(state, Text::new(&track.title))
                    .on_press(Message::QueueTrack(track.clone()));
                list.push(btn)
            })
            .into()
    }

    fn albums_view(&self, albums: &[AlbumModel]) -> Element<'_, Message> {
        albums
            .iter()
            .fold(Column::new(), |list, album| {
                let name = Text::new(&album.title);
                // let btn = Button::new(state, name)
                //     .on_press(Message::OpenView(MainView::Album(album.uri.clone())));
                list.push(name)
            })
            .into()
    }

    fn artists_view(&self, artists: &[ArtistModel]) -> Element<'_, Message> {
        artists
            .iter()
            .fold(Column::new(), |list, artist| {
                let c = Text::new(&artist.name);
                list.push(c)
            })
            .into()
    }

    fn tracks_view(&self, tracks: &[TrackModel]) -> Element<'_, Message> {
        tracks
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
