use crate::component::Component;
use crate::messages::Message;
use crate::views::MainView;
use iced::{button, scrollable, text_input, Application, Column, Command, Element, Row, Scrollable, Settings, Text, Vector, Background, Color, TextInput};
use rustic_core::Rustic;
use std::sync::Arc;

mod component;
mod messages;
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
    player_button: button::State
}

impl Application for IcedApplication {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Arc<Rustic>;

    fn new(app: Self::Flags) -> (Self, Command<Self::Message>) {
        let sidenav = vec![
            ("Library".into(), button::State::new(), MainView::Library),
            (
                "Playlists".into(),
                button::State::new(),
                MainView::Playlists,
            ),
            ("Explore".into(), button::State::new(), MainView::Explore),
        ];
        (
            IcedApplication {
                app,
                sidenav,
                current_view: MainView::default(),
                main_scroll: scrollable::State::new(),
                search_state: text_input::State::new(),
                search_query: String::new(),
                player_button: button::State::new()
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
            },
            Message::Search(query) => {
                self.search_query = query;
            },
            Message::ChangePlayer => {}
        }
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let mut nav = Row::new();
        for (label, state, view) in self.sidenav.iter_mut() {
            let mut btn = button::Button::new(state, Text::new(label.clone()))
                .on_press(Message::OpenView(*view));
            if self.current_view == *view {
                btn = btn.style(ActiveNavigationButtonStyle);
            }else {
                btn = btn.style(NavigationButtonStyle);
            };
            nav = nav.push(btn);
        }
        let search = TextInput::new(&mut self.search_state, "Search...", &self.search_query, |q| Message::Search(q));
        nav = nav.push(search);

        let player = self.app.get_default_player().unwrap();
        let player_select = button::Button::new(&mut self.player_button, Text::new(&player.display_name))
            .style(NavigationButtonStyle)
            .on_press(Message::ChangePlayer);
        nav = nav.push(player_select);

        let content = self.current_view.view(&self.app);
        let scroll_container = Scrollable::new(&mut self.main_scroll).push(content);

        Column::new().push(nav).push(scroll_container).into()
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
