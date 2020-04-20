use crate::component::Component;
use crate::messages::Message;
use crate::overlay::{Overlay, OverlayState};
use crate::views::MainView;
use iced::{
    button, scrollable, text_input, Align, Application, Background, Color, Column, Command,
    Element, Length, Row, Scrollable, Settings, Text, TextInput, Vector,
};
use rustic_core::player::Player;
use rustic_core::Rustic;
use std::sync::Arc;

mod component;
mod messages;
mod overlay;
mod views;

pub fn start(app: Arc<Rustic>) {
    IcedApplication::run(Settings::with_flags(app));
}

struct IcedApplication {
    app: Arc<Rustic>,
    sidenav: Vec<(String, button::State, MainView)>,
    current_view: MainView,
    main_scroll: scrollable::State,
    search_state: text_input::State,
    search_query: String,
    player_button: button::State,
    overlay: Option<OverlayState>,
    player: Option<Arc<Player>>,
}

impl Application for IcedApplication {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Arc<Rustic>;

    fn new(app: Self::Flags) -> (Self, Command<Self::Message>) {
        let sidenav = vec![
            ("Albums".into(), button::State::new(), MainView::Albums),
            ("Artists".into(), button::State::new(), MainView::Artists),
            ("Tracks".into(), button::State::new(), MainView::Tracks),
            (
                "Playlists".into(),
                button::State::new(),
                MainView::Playlists,
            ),
            ("Explore".into(), button::State::new(), MainView::Explore),
        ];
        let player = app.get_default_player();
        (
            IcedApplication {
                app,
                sidenav,
                current_view: MainView::default(),
                main_scroll: scrollable::State::new(),
                search_state: text_input::State::new(),
                search_query: String::new(),
                player_button: button::State::new(),
                overlay: None,
                player,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Rustic")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::OpenView(view) => {
                self.current_view = view;
            }
            Message::Search(query) => {
                self.search_query = query;
            }
            Message::OpenOverlay(overlay) => {
                let state = match overlay {
                    Overlay::PlayerList => {
                        let players = self
                            .app
                            .get_players()
                            .iter()
                            .map(|(_, player)| (button::State::new(), Arc::clone(player)))
                            .collect();
                        OverlayState::PlayerList(players)
                    }
                };
                self.overlay = Some(state);
            }
            Message::SelectPlayer(player) => {
                self.overlay = None;
                self.player = Some(player);
            }
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
                let btn = button::Button::new(state, Text::new(&player.display_name))
                    .on_press(Message::SelectPlayer(Arc::clone(player)));
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
                |q| Message::Search(q),
            );
            nav = nav.push(search);

            let player_label = Text::new(
                &self
                    .player
                    .as_ref()
                    .map(|p| format!("Player: {}", &p.display_name))
                    .unwrap_or_else(|| String::from("-- Select Player -")),
            );
            let player_select = button::Button::new(&mut self.player_button, player_label)
                .style(NavigationButtonStyle)
                .on_press(Message::OpenOverlay(Overlay::PlayerList));
            nav = nav.push(player_select);

            let content = self.current_view.view(&self.app);
            let scroll_container = Scrollable::new(&mut self.main_scroll).push(content);

            Column::new().push(nav).push(scroll_container).into()
        }
    }
}

struct NavigationButtonStyle;

impl iced::widget::button::StyleSheet for NavigationButtonStyle {
    fn active(&self) -> iced::widget::button::Style {
        iced::widget::button::Style {
            shadow_offset: Vector::new(0.0, 0.0),
            background: None,
            border_radius: 0,
            border_width: 0,
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
            border_radius: 0,
            border_width: 0,
            border_color: [0.7, 0.7, 0.7].into(),
            text_color: Color::WHITE,
        }
    }
}
