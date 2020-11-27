use iced::{
    button, scrollable, text_input, Align, Application, Background, Color, Column, Command,
    Element, Length, Row, Scrollable, Settings, Subscription, Text, TextInput, Vector,
};

use rustic_api::models::{
    AlbumModel, ArtistModel, PlayerModel, PlaylistModel, SyncStateModel, TrackModel,
};
use rustic_api::ApiClient;

use crate::component::Component;
use crate::messages::Message;
use crate::overlay::{Overlay, OverlayState};
use crate::recipes::SyncRecipe;
use crate::views::MainView;

mod component;
mod messages;
mod overlay;
mod recipes;
mod views;

pub fn start(api: ApiClient) {
    IcedApplication::run(Settings::with_flags(api));
}

struct IcedApplication {
    api: ApiClient,
    sidenav: Vec<(String, button::State, MainView)>,
    current_view: MainView,
    main_scroll: scrollable::State,
    search_state: text_input::State,
    search_query: String,
    player_button: button::State,
    sync_button: button::State,
    overlay: Option<OverlayState>,
    player: Option<PlayerModel>,
    state: SavedState,
    sync_state: SyncStateModel,
}

impl Application for IcedApplication {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ApiClient;

    fn new(api: Self::Flags) -> (Self, Command<Self::Message>) {
        let sidenav = vec![
            ("Albums".into(), button::State::new(), MainView::Albums),
            ("Artists".into(), button::State::new(), MainView::Artists),
            ("Tracks".into(), button::State::new(), MainView::Tracks),
            (
                "Playlists".into(),
                button::State::new(),
                MainView::Playlists(Vec::new()),
            ),
            ("Explore".into(), button::State::new(), MainView::Explore),
        ];
        (
            IcedApplication {
                api,
                sidenav,
                current_view: MainView::default(),
                main_scroll: scrollable::State::new(),
                search_state: text_input::State::new(),
                search_query: String::new(),
                player_button: button::State::new(),
                sync_button: button::State::new(),
                overlay: None,
                player: None,
                state: SavedState::default(),
                sync_state: SyncStateModel::Idle,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Rustic".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Loaded(state) => {
                self.state = state;
                if let Some(OverlayState::PlayerList(players)) = self.overlay.as_mut() {
                    *players = self
                        .state
                        .players
                        .iter()
                        .map(|player| (button::State::new(), player.clone()))
                        .collect();
                }
                if let MainView::Playlists(playlists) = &mut self.current_view {
                    *playlists = self
                        .state
                        .playlists
                        .iter()
                        .map(|playlist| (button::State::new(), playlist.clone()))
                        .collect();
                }
            }
            Message::OpenView(view) => {
                self.current_view = view;
                let state = self.state.clone();
                return match self.current_view {
                    MainView::Albums => {
                        Command::perform(state.load_albums(self.api.clone()), Message::Loaded)
                    }
                    MainView::Playlists(_) => {
                        Command::perform(state.load_playlists(self.api.clone()), Message::Loaded)
                    }
                    MainView::Artists => {
                        Command::perform(state.load_artists(self.api.clone()), Message::Loaded)
                    }
                    MainView::Tracks => {
                        Command::perform(state.load_tracks(self.api.clone()), Message::Loaded)
                    }
                    _ => Command::none(),
                };
            }
            Message::Search(query) => {
                self.search_query = query;
            }
            Message::Syncing(model) => {
                self.sync_state = model;
                if self.sync_state == SyncStateModel::Idle {
                    let state = self.state.clone();
                    return Command::batch(vec![
                        Command::perform(
                            state.clone().load_albums(self.api.clone()),
                            Message::Loaded,
                        ),
                        Command::perform(
                            state.clone().load_playlists(self.api.clone()),
                            Message::Loaded,
                        ),
                        Command::perform(
                            state.clone().load_artists(self.api.clone()),
                            Message::Loaded,
                        ),
                        Command::perform(state.load_tracks(self.api.clone()), Message::Loaded),
                    ]);
                }
            }
            Message::QueueTrack(track) => {
                let api = self.api.clone();
                return Command::perform(SavedState::queue_track(api, track), |_| {
                    Message::QueueUpdated
                });
            }
            Message::OpenOverlay(overlay) => {
                return match overlay {
                    Overlay::PlayerList => {
                        self.overlay = Some(OverlayState::PlayerList(Vec::new()));
                        let state = self.state.clone();
                        Command::perform(state.load_players(self.api.clone()), Message::Loaded)
                    }
                };
            }
            Message::SelectPlayer(player) => {
                self.overlay = None;
                self.player = Some(player);
            }
            Message::QueueUpdated => {}
        }
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        if let Some(OverlayState::PlayerList(players)) = self.overlay.as_mut() {
            let mut list = Column::new()
                .align_items(Align::Center)
                .width(Length::Fill)
                .spacing(20)
                .push(Text::new("Players").size(100));
            for (state, player) in players {
                let btn = button::Button::new(state, Text::new(&player.name))
                    .on_press(Message::SelectPlayer(player.clone()));
                list = list.push(btn);
            }
            list.into()
        } else {
            let mut nav = Row::new();
            for (label, state, view) in self.sidenav.iter_mut() {
                let mut btn = button::Button::new(state, Text::new(label.clone()))
                    .on_press(Message::OpenView(view.clone()));
                if self.current_view == *view {
                    btn = btn.style(ActiveNavigationButtonStyle);
                } else {
                    btn = btn.style(NavigationButtonStyle);
                };
                nav = nav.push(btn);
            }
            let search = TextInput::new(
                &mut self.search_state,
                "Search...",
                &self.search_query,
                Message::Search,
            );
            nav = nav.push(search);

            let player_label = Text::new(
                &self
                    .player
                    .as_ref()
                    .map(|p| format!("Player: {}", &p.name))
                    .unwrap_or_else(|| String::from("-- Select Player -")),
            );
            let player_select = button::Button::new(&mut self.player_button, player_label)
                .style(NavigationButtonStyle)
                .on_press(Message::OpenOverlay(Overlay::PlayerList));
            nav = nav.push(player_select);

            let sync_label = match self.sync_state {
                SyncStateModel::Idle => String::from("Sync: Idle"),
                SyncStateModel::Synchronizing(_) => String::from("Sync: Syncing"),
            };
            let sync_label = Text::new(sync_label);
            let sync_label =
                button::Button::new(&mut self.sync_button, sync_label).style(NavigationButtonStyle);
            nav = nav.push(sync_label);

            let content = self.current_view.view(&self.state);
            let scroll_container = Scrollable::new(&mut self.main_scroll).push(content);

            Column::new().push(nav).push(scroll_container).into()
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let recipe = SyncRecipe::new(self.api.clone());
        Subscription::from_recipe(recipe).map(Message::Syncing)
    }
}

#[derive(Debug, Default, Clone)]
pub struct SavedState {
    pub players: Vec<PlayerModel>,
    pub albums: Vec<AlbumModel>,
    pub artists: Vec<ArtistModel>,
    pub playlists: Vec<PlaylistModel>,
    pub tracks: Vec<TrackModel>,
}

impl SavedState {
    async fn queue_track(api: ApiClient, track: TrackModel) {
        api.queue_track(None, &track.cursor).await.unwrap();
    }

    async fn load_players(self, api: ApiClient) -> SavedState {
        let players = api.get_players().await.unwrap();
        SavedState { players, ..self }
    }

    async fn load_albums(self, api: ApiClient) -> SavedState {
        let albums = api.get_albums(None).await.unwrap();
        SavedState { albums, ..self }
    }

    async fn load_artists(self, api: ApiClient) -> SavedState {
        let artists = api.get_artists().await.unwrap();
        SavedState { artists, ..self }
    }

    async fn load_playlists(self, api: ApiClient) -> SavedState {
        let playlists = api.get_playlists(None).await.unwrap();
        SavedState { playlists, ..self }
    }

    async fn load_tracks(self, api: ApiClient) -> SavedState {
        let tracks = api.get_tracks(None).await.unwrap();
        SavedState { tracks, ..self }
    }
}

struct NavigationButtonStyle;

impl iced::widget::button::StyleSheet for NavigationButtonStyle {
    fn active(&self) -> iced::widget::button::Style {
        iced::widget::button::Style {
            shadow_offset: Vector::new(0.0, 0.0),
            background: None,
            border_radius: 0f32,
            border_width: 0f32,
            border_color: [0.7, 0.7, 0.7].into(),
            text_color: Color::BLACK,
        }
    }
}

struct ActiveNavigationButtonStyle;

impl iced::widget::button::StyleSheet for ActiveNavigationButtonStyle {
    fn active(&self) -> iced::widget::button::Style {
        iced::widget::button::Style {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some(Background::Color([0.2, 0.2, 0.2].into())),
            border_radius: 0f32,
            border_width: 0f32,
            border_color: [0.7, 0.7, 0.7].into(),
            text_color: Color::WHITE,
        }
    }
}
